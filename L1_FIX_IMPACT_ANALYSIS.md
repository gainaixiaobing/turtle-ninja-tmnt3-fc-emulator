# L1 FIX IMPACT ANALYSIS — 方案A

## 修改范围

### 文件：src/main.rs

| 行号 | 当前内容 | 修改摘要 |
|------|----------|----------|
| 379 | `Event::KeyUp { keycode: Some(key), .. } if !show_help =>` | 删除 `if !show_help` guard |

仅此一行。无新增代码，无新增依赖，无新增变量。

---

## 影响面逐项确认

### 1. Xbox 输入（Player 1）

| 代码区域 | 行号范围 | 是否受影响 |
|----------|----------|------------|
| ControllerDeviceAdded | 393-404 | ❌ 无关联 |
| ControllerDeviceRemoved | 406-410 | ❌ 无关联 |
| ControllerButtonDown | 412-433 | ❌ 无关联 |
| ControllerButtonUp | 435-460 | ❌ 无关联 |
| ControllerAxisMotion | 462-500 | ❌ 无关联 |

**结论：不影响。** Xbox 手柄事件处理器是独立的 match 分支，不检查 `show_help`，不引用被修改的代码路径。

### 2. 音频系统

| 代码区域 | 行号范围 | 是否受影响 |
|----------|----------|------------|
| NesAudioCallback 结构体 | 17-38 | ❌ 无关联 |
| 音频缓冲写入 | 503-509 | ❌ 无关联 |

**结论：不影响。** 音频系统由 `deck.clock_frame()` 产生样本，通过 `Arc<Mutex<Vec<f32>>>` 传递给 SDL2 回调。与事件处理分支无关。

### 3. Overlay 渲染

| 代码区域 | 行号范围 | 是否受影响 |
|----------|----------|------------|
| show_help 状态变量 | 348 | ❌ 不修改 |
| F1 切换逻辑 | 359-361 | ❌ 不修改 |
| render_help_overlay 调用 | 523-525 | ❌ 不修改 |
| render_help_overlay 函数 | 138-210 | ❌ 不修改 |

**结论：不影响。** `show_help` 变量本身不受影响。修改仅影响 KeyUp 事件是否更新 joypad 状态，不影响 overlay 的显示/隐藏判断和渲染逻辑。

### 4. 游戏逻辑（模拟核心）

| 代码区域 | 行号范围 | 是否受影响 |
|----------|----------|------------|
| deck.clock_frame() | 502 附近 | ❌ 无关联 |
| frame_buffer / texture | 511-516 | ❌ 无关联 |
| KeyDown 处理 (P2) | 364-376 | ❌ 不修改 |

**结论：不影响。** `clock_frame()` 读取 joypad 当前状态进行模拟。方案A 使 joypad 状态更准确（物理释放 = joypad 释放），对模拟核心而言是输入正确性的改善，不改变任何模拟逻辑。

---

## 副作用分析

### 行为变化（仅 P2 键盘）

| 场景 | 修改前 | 修改后 |
|------|--------|--------|
| Overlay 打开期间松开按键 | joypad 状态卡死 | joypad 状态正确释放 |
| Overlay 打开期间按下按键 | 被 guard 拦截，不生效 | 仍被 KeyDown guard 拦截，不生效（未变） |
| 无 overlay 时正常输入 | 正常 | 正常（未变） |

行为变化仅限于：overlay 期间的 KeyUp 事件从"被丢弃"变为"被处理"。

### 不会引入的变化

- 不会改变 match 分支结构（F1、Escape、KeyDown 的 guard 保持不变）
- 不会改变 joypad API 调用方式
- 不会新增状态变量
- 不会改变事件处理顺序

---

## 结论

方案A 是**纯局部修复**：

- 修改点：1 行
- 影响范围：P2 键盘 KeyUp 事件路径
- 不触碰：Xbox 输入、音频系统、Overlay 渲染、游戏模拟逻辑
- 副作用：仅消除 overlay 期间的 joypad 状态泄漏

风险等级：**极低**。

---

*Generated: 2026-06-03*
