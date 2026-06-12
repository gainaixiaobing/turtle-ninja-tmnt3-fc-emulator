# L1 FIX RECOMMENDATION

## Bug Reference

See `BUG_REPORT_L1.md` — F1 help overlay causes stuck keyboard input for Player 2.

## Fix Candidates

### 方案A: KeyUp 始终处理

移除 `KeyUp` 事件处理器上的 `if !show_help` guard，使按键释放始终生效。

```rust
// Before (line 379)
Event::KeyUp { keycode: Some(key), .. } if !show_help => {

// After
Event::KeyUp { keycode: Some(key), .. } => {
```

改动量：删除 1 个 guard 条件。

### 方案B: 打开 Overlay 时清空按键状态

在 F1 切换 `show_help = true` 时，主动将 Player 2 的所有 joypad 按钮重置为 `false`。

```rust
Event::KeyDown { keycode: Some(Keycode::F1), .. } => {
    show_help = !show_help;
    if show_help {
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::UP, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::DOWN, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::LEFT, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::RIGHT, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::A, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::B, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::START, false);
        deck.joypad_mut(Player::Two).set_button(JoypadBtnState::SELECT, false);
    }
}
```

改动量：新增 ~10 行代码。

---

## Analysis

### 1. 正确性

| 维度 | 方案A | 方案B |
|------|-------|-------|
| 修复 L1 | ✅ 完全修复 | ⚠️ 部分修复 |
| 防御深度 | 根因修复 | 症状缓解 |

**方案A** 直接解决根因：KeyUp 事件不应被丢弃。无论 overlay 状态如何，物理按键释放必然对应 joypad 状态释放。状态机始终一致。

**方案B** 在 overlay 打开时清空状态，修复了"已知按键"的卡死问题。但存在一个未修复的边界：

1. 打开 overlay（状态已清空）
2. **在 overlay 打开期间按下方向键**（KeyDown 被 guard 拦截，joypad 未设置）
3. 关闭 overlay
4. 松开方向键（KeyUp 被 guard 拦截 — 如果 guard 仍在）

如果方案B配合保留 KeyUp guard，则步骤 2-4 的按键被完全忽略，无卡死但也无响应 — 可接受但不理想。如果方案B同时移除 KeyUp guard，则退化为方案A，清空操作变得多余。

### 2. 风险

| 风险点 | 方案A | 方案B |
|--------|-------|-------|
| 引入新 bug | 极低 | 低 |
| Overlay 期间游戏状态变化 | 游戏本身仍在 clock_frame()，KeyUp 更新 joypad 是正常行为 | 清空后如按键仍被物理按住，关闭 overlay 后按键不生效（SDL 不会重发 KeyDown） |

**方案A 的关键洞察：** Overlay 打开时，游戏主循环并未暂停 — `deck.clock_frame()` 每帧仍在执行。允许 KeyUp 更新 joypad 状态是**语义正确**的：物理按键释放 = joypad 释放。这与 Xbox 手柄的行为一致（手柄事件从不检查 `show_help`）。

**方案B 的风险：** SDL2 的按键事件模型是"按下时发一次 KeyDown，释放时发一次 KeyUp"。如果 overlay 打开时清空了 joypad 状态，但玩家手指仍按在键上，关闭 overlay 后 SDL 不会重发 KeyDown。玩家必须先松开再按下才能恢复响应 — 这是一个新的、不直观的行为缺陷。

### 3. 复杂度

| 指标 | 方案A | 方案B |
|------|-------|-------|
| 代码改动 | 1 行删除 | ~10 行新增 |
| 状态变量 | 无新增 | 无新增 |
| 逻辑路径 | 减少（更简洁） | 增加（更多分支） |
| 认知负荷 | 降低 | 不变或略增 |

### 4. 长期维护性

| 维度 | 方案A | 方案B |
|------|-------|-------|
| 新增按键时 | KeyDown/KeyUp 自然对称，不易遗漏 | 需同步更新清空列表，易遗漏 |
| 新增 overlay 类似功能时 | 无需特殊处理 | 需要在每个 overlay 切换点复制清空逻辑 |
| 代码审查负担 | guard 条件更少，更易理解 | 需要理解"清空状态"的动机 |
| 与 P1 手柄一致性 | ✅ KeyUp 无 guard，P1/P2 行为一致 | ❌ P2 有特殊清空逻辑，P1 没有 |

---

## Recommendation

### 推荐：方案A

理由：

1. **根因修复** — KeyUp 被丢弃是问题的唯一原因，移除 guard 直接消除根因
2. **语义正确** — 游戏未暂停，KeyUp 更新 joypad 是正确行为，与手柄一致
3. **零新增风险** — 不引入 SDL KeyDown/KeyUp 不对称的新问题
4. **最小改动** — 1 行删除，无新代码，无新逻辑路径
5. **维护友好** — 新增按键无需同步维护额外清空列表

方案B 作为**补充防御**可考虑在未来叠加，但不应替代方案A。单独使用方案B 存在 SDL 按键事件不对称导致的新边界问题。

---

*Generated: 2026-06-03*
