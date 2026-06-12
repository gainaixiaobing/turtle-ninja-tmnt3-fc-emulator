# Save State Feasibility Study

> 基于 tetanes-core 0.14.1 实际源码分析，非推测。

## 1. 当前模拟器架构

### 组件树

```
ControlDeck
  └── Cpu (cpu.rs)
        ├── cycle, master_clock, pc, sp, acc, x, y, status, irq_flags
        └── Bus (bus.rs)
              ├── Ppu (ppu.rs)  — 渲染状态、OAM、palette RAM、VRAM
              ├── Apu (apu.rs)  — Pulse1/2、Triangle、Noise、DMC、滤波器
              ├── Input (input.rs) — 手柄状态（serde skip）
              ├── WRAM — 2KB CPU 工作 RAM
              └── region: NesRegion
```

### 各组件位置

| 组件 | 文件 | 结构体 | Serialize/Deserialize |
|------|------|--------|----------------------|
| CPU State | `src/cpu.rs:70` | `Cpu` | 已实现 |
| PPU State | `src/ppu.rs:159` | `Ppu` | 已实现 |
| APU State | `src/apu.rs:70` | `Apu` | 已实现 |
| CPU RAM (WRAM) | `src/bus.rs:51` | `Bus.wram` (2KB) | 已实现 |
| Mapper | `src/mapper.rs:95` | `Mapper` enum (20+ variant) | 已实现 |
| Mapper 004 (TMNT3) | `src/mapper/m004_txrom.rs:63` | `Txrom` | 已实现 |
| PPU OAM | `src/ppu.rs` | 内嵌于 `Ppu` | 已实现 |
| Palette RAM | `src/ppu.rs` | 内嵌于 `Ppu` | 已实现 |

### 关键发现

**tetanes-core 已内置完整 Save State 支持。** TurtleBox 无需自行实现序列化逻辑。

## 2. Save State API

### 已有 API（control_deck.rs）

```rust
// 保存 (control_deck.rs:528)
pub fn save_state(&mut self, path: impl AsRef<Path>) -> Result<()>

// 加载 (control_deck.rs:541)
pub fn load_state(&mut self, path: impl AsRef<Path>) -> Result<()>
```

### 底层序列化（fs.rs）

```rust
// 通用序列化 — 任何 T: Serialize
pub fn save<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()>

// 通用反序列化 — 任何 T: DeserializeOwned
pub fn load<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T>
```

### 序列化链路

```
deck.save_state(path)
  → fs::save(path, &self.cpu)
    → bincode::serde::encode_to_vec(cpu, config)   // 二进制序列化
    → DeflateEncoder 压缩
    → 写入文件：[Header 9 bytes] + [压缩后的 bincode 数据]
```

### 反序列化链路

```
deck.load_state(path)
  → fs::load::<Cpu>(path)
    → validate_header()  // 校验 TETANES\x1a + version
    → DeflateDecoder 解压
    → bincode::serde::decode_from_slice → Cpu
    → cpu.bus.input.clear()  // 清除存档时的输入状态
    → self.load_cpu(cpu)     // 恢复完整状态
```

## 3. 保存格式

### 二进制格式

```
偏移 0x00: [u8; 8]  魔数 "TETANES\x1a"
偏移 0x08: [u8; 1]  版本号 "1"
偏移 0x09: [deflate 压缩的 bincode 数据]
```

- **格式**: 二进制（非 JSON、非文本）
- **序列化库**: bincode（高效二进制格式）
- **压缩**: flate2 deflate（默认压缩级别）
- **魔数**: `TETANES\x1a`（8 字节）

### 文件大小预估

| 内容 | 大小 |
|------|------|
| Header | 9 bytes |
| CPU 寄存器 | ~30 bytes |
| WRAM (2KB) | ~2048 bytes |
| PPU 状态（含 OAM 256B + palette 32B + VRAM） | ~4-8 KB |
| APU 状态 | ~1-2 KB |
| Mapper 004 (Txrom) 寄存器 | ~200 bytes |
| **压缩后总计（预估）** | **3-8 KB** |

deflate 对重复模式（如全零 WRAM）压缩率极高。实际 TMNT3 存档文件预计 3-8KB。

### #[serde(skip)] 字段

以下字段在序列化时被跳过（运行时缓存，非关键状态）：

| 字段 | 位置 | 原因 |
|------|------|------|
| `ram_state` | Bus | RAM 初始化状态，非运行时数据 |
| `channel_outputs` | Apu | 音频输出缓冲，运行时重建 |
| `audio_samples` | Apu | 当前帧音频，无需保存 |
| `last_frame_number` | ControlDeck | 帧计数缓存 |

这些字段跳过不影响恢复后的正确性。

## 4. 风险分析

| 风险 | 等级 | 说明 |
|------|------|------|
| 序列化正确性 | **低** | tetanes-core 已有成熟实现，上游 tetanes 模拟器长期使用 |
| Mapper 004 支持 | **低** | Txrom 已实现 Serialize/Deserialize，TMNT3 使用此 Mapper |
| 跨平台兼容 | **低** | bincode 使用固定字节序，deflate 是标准算法 |
| 跨版本兼容 | **中** | SAVE_VERSION 当前为 "1"，版本变更会导致旧存档不可读 |
| 文件损坏 | **低** | 有 header 校验 + deflate 解压校验 |
| 加载后游戏状态不一致 | **低** | load_state 自动清除 input buffer，load_cpu 恢复完整状态 |
| WRAM 全零 vs 随机初始化 | **低** | `ram_state` 被 skip，加载后使用默认初始化 |

## 5. 工作量评估

### 最小版本（单存档槽位）

| 工作项 | 工作量 |
|--------|--------|
| 新增快捷键处理（F5/F8） | ~10 行 main.rs |
| 存档路径计算 | ~15 行 state.rs |
| 调用 deck.save_state / deck.load_state | ~10 行 state.rs |
| 反馈 overlay（SAVED / LOADED） | ~30 行 overlay.rs |
| 错误处理 | ~5 行 |
| **总计** | **~70 行代码，0 新文件** |

### 完整版本（多槽位 + 配置）

| 工作项 | 工作量 |
|--------|--------|
| 最小版本 | ~70 行 |
| 多槽位支持（F5/F6/F7/F8 对应 1-4 槽位） | ~40 行 |
| 槽位选择 overlay | ~50 行 overlay.rs |
| config.toml 新增 save_dir 配置 | ~10 行 config.rs |
| 存档管理（删除、列表） | ~60 行 |
| **总计** | **~230 行代码，0 新文件** |

## 6. 推荐路线

### 推荐 A: v0.2 立即实现最小版本

**理由：**

1. **tetanes-core 已完成全部底层工作** — TurtleBox 只需调用两个现成 API
2. **开发成本极低** — 最小版本约 70 行代码，半天工作量
3. **TMNT3 玩家刚需** — 关卡难度高，Save State 是最直接影响游戏体验的功能
4. **零风险** — 不需要修改任何模拟器核心逻辑，只是在 UI 层添加调用
5. **上游验证** — tetanes 模拟器本身已长期使用这套 API

**具体路径：**
- F5 保存当前状态到 ROM 同目录的 `.sav` 文件
- F8 从 `.sav` 文件加载状态
- 保存/加载时显示 overlay 反馈（1.5 秒）
- 单槽位，覆盖保存

完整版本（多槽位）可推迟到 v0.3。
