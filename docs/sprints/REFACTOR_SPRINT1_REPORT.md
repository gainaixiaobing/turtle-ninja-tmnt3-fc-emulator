# REFACTOR SPRINT 1 REPORT

**版本:** TurtleBox v0.2
**Sprint:** Architecture Refactor Sprint 1
**范围:** Phase 1 - input / overlay extraction

---

## 1. 修改文件

| 文件 | 变更 |
|------|------|
| `src/main.rs` | 移除输入映射实现与 Controls Overlay 绘制实现，改为调用 `input` / `overlay` 模块 |
| `src/input.rs` | 新增键盘与 Xbox 手柄输入处理函数 |
| `src/overlay.rs` | 新增位图字体、文字绘制、Controls Overlay 绘制函数 |
| `REFACTOR_SPRINT1_REPORT.md` | 新增本次重构报告 |

未修改：

| 范围 | 状态 |
|------|------|
| ROM 加载逻辑 | 未改动 |
| SDL 音频逻辑 | 未改动 |
| 视频渲染管线 | 未改动 |
| 帧率限制逻辑 | 未改动 |
| 用户可见功能 | 未新增 |

---

## 2. main.rs 行数变化

| 文件 | Before | After |
|------|--------|-------|
| `src/main.rs` | 541 | 318 |

新增模块行数：

| 文件 | 行数 |
|------|------|
| `src/input.rs` | 181 |
| `src/overlay.rs` | 198 |

说明：

- `cargo fmt` 对部分长行进行了格式化展开。
- `main.rs` 仍保留主流程、ROM 加载、SDL 初始化、音频、渲染和游戏循环。
- 输入映射细节已迁移到 `input.rs`。
- Overlay 位图字体和绘制细节已迁移到 `overlay.rs`。

---

## 3. 编译结果

命令：

```bash
cargo build --release
```

结果：

```text
Finished `release` profile [optimized] target(s) in 2.31s
```

编译状态：

| 项目 | 状态 |
|------|------|
| `cargo build --release` | 通过 |
| 编译错误 | 无 |
| 新增 warning | 无 |

既有提示：

```text
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
```

该提示来自既有间接依赖，不是本次重构新增。

---

## 4. 风险说明

### 低风险

| 风险 | 说明 | 缓解 |
|------|------|------|
| Overlay 模块迁移 | `char_bitmap()`, `draw_char()`, `draw_text()`, `render_help_overlay()` 仅从 `main.rs` 移入 `overlay.rs` | 保持绘制文本、颜色、坐标、scale、blend mode 不变 |
| 输入模块迁移 | 键盘与 Xbox 映射仅从 match arm 移入函数 | 保持 P1/P2、按下/释放、摇杆阈值不变 |
| `main.rs` 调用点变化 | 主循环改为调用模块函数 | 事件 match 顺序保持不变 |

### 中风险

| 风险 | 说明 | 缓解 |
|------|------|------|
| F1 overlay 输入屏蔽行为 | KeyDown 仍受 `if !show_help` 保护，KeyUp 仍不受保护 | 保留原事件 guard 位置，避免回退 L1 修复 |
| Xbox 左摇杆方向状态 | Axis 逻辑迁入函数后仍需同时设置正反方向状态 | 原样迁移阈值与方向释放逻辑 |

### 高风险

无。

---

## 结论

Architecture Refactor Sprint 1 Phase 1 已完成。

- 输入系统已从 `main.rs` 分离到 `src/input.rs`
- Controls Overlay 系统已从 `main.rs` 分离到 `src/overlay.rs`
- `cargo build --release` 通过
- 未进入 config.toml、Save State、Fullscreen、Pause、Volume

等待下一步指令。
