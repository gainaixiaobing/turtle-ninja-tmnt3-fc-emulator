# Sprint 2 — Config System Report

**日期：** 2026-06-07
**Sprint：** Sprint 2 — 配置系统基础设施

---

## 1. 新增文件

| 文件 | 行数 | 说明 |
|------|------|------|
| `src/config.rs` | 93 | AppConfig 结构体、ControllerConfig、load/save、默认值生成 |

## 2. 修改文件

| 文件 | Before | After | 变化 |
|------|--------|-------|------|
| `Cargo.toml` | 9 | 11 | +2（serde、toml 依赖） |
| `src/main.rs` | 318 | 322 | +4（mod config、AppConfig::load 调用） |

未修改：

| 文件 | 状态 |
|------|------|
| `src/input.rs` | 未改动（181 行） |
| `src/overlay.rs` | 未改动（198 行） |

---

## 3. 配置格式

```toml
fullscreen = false
volume = 100

[controller]
a = "A"
b = "B"
start = "START"
select = "BACK"
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `fullscreen` | `bool` | 全屏模式开关（当前仅存储，未启用功能） |
| `volume` | `u8` | 音量百分比 0-100（当前仅存储，未启用功能） |
| `controller.a` | `String` | SDL2 手柄按钮名 -> NES A |
| `controller.b` | `String` | SDL2 手柄按钮名 -> NES B |
| `controller.start` | `String` | SDL2 手柄按钮名 -> NES Start |
| `controller.select` | `String` | SDL2 手柄按钮名 -> NES Select |

## 4. 默认配置

当 `config.toml` 不存在时，程序自动生成以下默认配置：

```toml
fullscreen = false
volume = 100

[controller]
a = "A"
b = "B"
start = "START"
select = "BACK"
```

## 5. 配置文件位置

优先级：
1. 可执行文件同目录下的 `config.toml`
2. 回退到当前工作目录下的 `config.toml`

## 6. 加载行为

```
启动
  ├── config.toml 存在？
  │   ├── 是 → 解析 TOML
  │   │   ├── 解析成功 → 使用该配置
  │   │   └── 解析失败 → 打印错误，使用默认值
  │   └── 否 → 使用默认值
  └── 无论哪种情况 → 如果文件不存在则自动生成默认 config.toml
```

## 7. 新增依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| `serde` | 1.x（features = ["derive"]） | 序列化/反序列化 |
| `toml` | 0.8 | TOML 格式解析 |

## 8. 编译结果

```bash
cargo build --release
# Finished `release` profile [optimized] target(s) in 0.04s
# 0 errors
# warning: block v0.1.6 未来兼容性提示（既有，未新增）
```

```bash
cargo clippy
# 2 warnings（均为 v0.1 既有，未新增）
# - needless_range_loop（main.rs:33）
# - single_match（main.rs:181）
```

## 9. 功能行为确认

| 项目 | 状态 |
|------|------|
| Xbox 手柄 | 未改动（input.rs 不变） |
| Keyboard | 未改动（input.rs 不变） |
| TMNT3 模拟 | 未改动（main.rs 模拟逻辑不变） |
| Overlay | 未改动（overlay.rs 不变） |
| 配置加载 | 新增，启动时自动加载/生成 config.toml |

## 10. 代码结构（Sprint 2 后）

```
src/
├── main.rs    — 322 行（入口 + 主循环）
├── config.rs  —  93 行（配置系统）
├── input.rs   — 181 行（输入映射）
└── overlay.rs — 198 行（覆盖层渲染）
总计：794 行
```

## 11. 未实现（按要求排除）

- 自定义键位界面
- Save State
- Pause
- Fullscreen
- Volume 调节

以上功能仅在配置中预留了字段，未连接到游戏逻辑。

---

*Sprint 2 completed: 2026-06-07*
*Config infrastructure only — no functional changes*
