# Save State Implementation Plan

> 基于 tetanes-core 0.14.1 实际源码。所有 API 引用来自 `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tetanes-core-0.14.1/`。

## 1. Save State 相关 API

### 控制台级 API

| 函数 | 文件:行 | 参数 | 返回值 | 说明 |
|------|---------|------|--------|------|
| `ControlDeck::save_state` | `control_deck.rs:528` | `&mut self, path: impl AsRef<Path>` | `Result<()>` | 保存完整 CPU 状态树到文件 |
| `ControlDeck::load_state` | `control_deck.rs:541` | `&mut self, path: impl AsRef<Path>` | `Result<()>` | 从文件恢复 CPU 状态树 |
| `ControlDeck::loaded_rom` | `control_deck.rs` | `&self` | `Option<&LoadedRom>` | 检查是否已加载 ROM |

### 序列化层 API

| 函数 | 文件:行 | 参数 | 返回值 | 说明 |
|------|---------|------|--------|------|
| `fs::save` | `fs.rs:142` | `path: impl AsRef<Path>, value: &T` (T: Serialize) | `Result<()>` | bincode + deflate 序列化写入 |
| `fs::load` | `fs.rs:160` | `path: impl AsRef<Path>` (返回 T: DeserializeOwned) | `Result<T>` | 读取 + 校验 header + 解压 + bincode 反序列化 |
| `fs::exists` | `fs.rs:186` | `path: &Path` | `bool` | 检查文件是否存在 |

### 错误类型

| 错误 | 文件:行 | 说明 |
|------|---------|------|
| `Error::RomNotLoaded` | `control_deck.rs` | save_state/load_state 时无 ROM |
| `Error::NoSaveStateFound` | `control_deck.rs:45` | load_state 时文件不存在 |
| `Error::SaveState(fs::Error)` | `control_deck.rs:42` | 序列化/IO 错误 |

### 关键实现细节

```rust
// save_state 完整实现 (control_deck.rs:528-536)
pub fn save_state(&mut self, path: impl AsRef<Path>) -> Result<()> {
    if self.loaded_rom().is_none() {
        return Err(Error::RomNotLoaded);
    };
    let path = path.as_ref();
    fs::save(path, &self.cpu).map_err(Error::SaveState)
}
// 直接序列化 self.cpu — 包含 Bus → Ppu + Apu + WRAM + 所有运行时状态

// load_state 完整实现 (control_deck.rs:541-557)
pub fn load_state(&mut self, path: impl AsRef<Path>) -> Result<()> {
    if self.loaded_rom().is_none() {
        return Err(Error::RomNotLoaded);
    };
    let path = path.as_ref();
    if fs::exists(path) {
        fs::load::<Cpu>(path)
            .map_err(Error::SaveState)
            .map(|mut cpu| {
                cpu.bus.input.clear(); // ← 清除存档时的手柄输入
                self.load_cpu(cpu)     // ← 恢复完整 CPU 状态
            })
    } else {
        Err(Error::NoSaveStateFound)
    }
}
```

## 2. 最小可运行实现

### 需要修改的文件

| 文件 | 修改内容 | 预估行数 |
|------|----------|----------|
| `src/state.rs` | 新增 `save_state()` 和 `load_state()` 方法 | +25 行 |
| `src/main.rs` | 事件循环中添加 F5/F8 键处理 | +10 行 |
| `src/overlay.rs` | 新增 "SAVED" / "LOADED" / "NOT FOUND" overlay | +40 行 |

**不需要新增文件。**

### state.rs 新增方法

```rust
// 需要添加的 use：
use std::path::PathBuf;

impl GameState {
    // 保存状态 — 需要 ROM 文件路径来计算 .sav 路径
    pub fn save_state(&self, deck: &mut ControlDeck, rom_path: &str) -> Result<(), String> {
        let sav_path = sav_path_from_rom(rom_path);
        deck.save_state(&sav_path)
            .map_err(|e| format!("Save failed: {}", e))
    }

    // 加载状态
    pub fn load_state(&self, deck: &mut ControlDeck, rom_path: &str) -> Result<(), String> {
        let sav_path = sav_path_from_rom(rom_path);
        if !sav_path.exists() {
            return Err("No save state found".to_string());
        }
        deck.load_state(&sav_path)
            .map_err(|e| format!("Load failed: {}", e))
    }
}

// 路径计算：TMNT3.nes → TMNT3.sav（同目录）
fn sav_path_from_rom(rom_path: &str) -> PathBuf {
    let mut p = PathBuf::from(rom_path);
    p.set_extension("sav");
    p
}
```

### main.rs 事件处理

```rust
// 在系统按键 match 中添加：
Keycode::F5 => {
    gs.save_state(&mut deck, &rom_path);
    // 显示 SAVED overlay
}
Keycode::F8 => {
    gs.load_state(&mut deck, &rom_path);
    // 显示 LOADED overlay
}
```

### overlay.rs 新增

```rust
pub fn render_state_save_overlay(canvas: &mut Canvas<Window>, text: &str) {
    // 屏幕中央显示 "SAVED" 或 "LOADED" 或 "NO SAVE"
    // 1.5 秒后自动消失
}
```

### F5/F8 快捷键选择理由

- F5/F8 是 NES 模拟器社区的事实标准（FCEUX、Mesen、higan 均使用）
- 不与现有快捷键冲突（F1=Help, F11=Fullscreen）
- 不与游戏按键冲突

## 3. 数据格式

### 保存文件格式

```
文件名: TMNT3.sav（与 ROM 同名同目录）

二进制结构:
┌─────────────────────────┐
│ TETANES\x1a             │  ← 8 bytes 魔数
│ 0x31                    │  ← 1 byte 版本号 ("1")
├─────────────────────────┤
│ deflate 压缩数据         │
│  ┌─────────────────────┐│
│  │ bincode 序列化的     ││
│  │ Cpu 结构体           ││
│  │  ├─ 寄存器           ││
│  │  ├─ Bus              ││
│  │  │  ├─ Ppu (OAM+VRAM)││
│  │  │  ├─ Apu           ││
│  │  │  ├─ WRAM (2KB)    ││
│  │  │  └─ region        ││
│  │  └─ cycle/irq 状态   ││
│  └─────────────────────┘│
└─────────────────────────┘
```

### 文件大小预估

| 数据 | 原始大小 | deflate 压缩后 |
|------|----------|----------------|
| CPU 寄存器 + 元数据 | ~100 bytes | ~50 bytes |
| WRAM (2KB, 大量零) | 2,048 bytes | ~200 bytes |
| PPU (OAM 256B + VRAM 2KB + 寄存器) | ~3 KB | ~1-2 KB |
| APU (通道状态 + 滤波器) | ~500 bytes | ~200 bytes |
| Mapper 004 (bank 寄存器 + IRQ) | ~200 bytes | ~100 bytes |
| **TMNT3 预估总计** | **~6 KB** | **2-5 KB** |

## 4. 兼容性风险

| 维度 | 评估 | 说明 |
|------|------|------|
| 跨平台 | **兼容** | bincode 固定字节序，deflate 标准算法，header 魔数不变 |
| 跨版本 (tetanes-core) | **有风险** | `SAVE_VERSION` 当前为 "1"，版本变更会导致旧存档 header 校验失败 |
| 跨版本 (TurtleBox) | **兼容** | 只要 tetanes-core 版本不变，TurtleBox 版本升级不影响存档 |
| tetanes-core 升级 | **中风险** | 升级 tetanes-core 可能改变 Cpu 结构体布局，导致旧存档反序列化失败 |
| 不同 ROM 版本 | **低风险** | Mapper 类型不同会自然失败，同 Mapper 不同 ROM 版本可能正常工作 |
| 文件损坏 | **有保护** | header 校验 + deflate CRC 提供基本完整性检查 |

### 降低版本风险的方案

- 锁定 tetanes-core 版本（当前 `Cargo.toml` 已固定为 `"0.14"`）
- 升级前测试存档兼容性
- 不需要额外实现 — 库层已有版本保护

## 5. 开发成本评估

### 最小版本（单槽位 F5/F8）

| 项目 | 量 |
|------|-----|
| 修改文件 | 3 (state.rs, main.rs, overlay.rs) |
| 新增文件 | 0 |
| 代码修改量 | ~75 行 |
| 新增依赖 | 0（全部使用已有 API） |
| 测试工作量 | 手动测试：保存→加载→验证游戏状态一致 |
| 预估工时 | 2-3 小时 |

### 完整版本（多槽位）

| 项目 | 量 |
|------|-----|
| 在最小版本基础上 | +160 行 |
| 支持 4 个槽位（F5/F6/F7/F8） | 槽位选择逻辑 |
| 槽位预览 overlay | 显示各槽位保存时间 |
| 存档管理 | 删除存档 |
| 预估工时 | 1 天 |

## 6. 最终建议

### 推荐 A: 立即进入 Sprint 5 — 实现最小版本

**理由：**

1. **成本极低** — 75 行代码，0 新依赖，0 新文件，3 小时工时
2. **API 已就绪** — `deck.save_state(path)` 和 `deck.load_state(path)` 是现成的
3. **TMNT3 玩家刚需** — 这款游戏难度极高（水下关卡、战车关卡），Save State 是最直接影响游戏体验的功能
4. **零架构风险** — 不修改任何模拟器核心，只是 UI 层调用
5. **上游验证** — tetanes 模拟器本身已使用这套 API 多年

**实现顺序：**
1. state.rs: `save_state()` + `load_state()` + `sav_path_from_rom()`
2. overlay.rs: `render_state_overlay()` — SAVED/LOADED/NOT FOUND 反馈
3. main.rs: F5/F8 键处理

**推迟到 v0.3 的理由不成立** — 因为实现成本和风险都极低，且玩家体验提升最大。
