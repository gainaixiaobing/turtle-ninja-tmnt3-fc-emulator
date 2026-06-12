# BUG REPORT — L1: F1 Help Overlay Causes Stuck Keyboard Input

## Status

**Static Analysis Result: HIGH CONFIDENCE**
**Runtime Reproduction: NOT VERIFIED**

## Severity

Medium — gameplay-breaking for keyboard player (P2)

## Summary

When the F1 help overlay is open, all `KeyUp` events are silently discarded due to the `if !show_help` guard. If a key was held when F1 was pressed, that key's joypad state remains `true` permanently after the overlay is closed, causing the character to move/act uncontrollably.

## Affected Scope

- **Player:** Keyboard (Player 2) — all 8 buttons
- **Player:** Xbox Controller (Player 1) — NOT affected (controller events have no `show_help` guard)
- **Buttons affected:** UP, DOWN, LEFT, RIGHT, A, B, START, SELECT

## Reproduction Steps

1. Launch TurtleBox with TMNT3 ROM
2. Start game, select a character
3. **Hold** any direction key (e.g., Right) — character moves right
4. **While still holding**, press F1 — help overlay appears
5. **Release** the direction key
6. Press F1 again — help overlay closes
7. Observe: character continues moving right with no key pressed

## Root Cause

In `src/main.rs`, both `KeyDown` and `KeyUp` event handlers are guarded by `if !show_help`:

```rust
// Line 364
Event::KeyDown { keycode: Some(key), .. } if !show_help => { ... }

// Line 379
Event::KeyUp { keycode: Some(key), .. } if !show_help => { ... }
```

When `show_help = true`:
- `KeyDown` is correctly ignored (no new presses registered)
- `KeyUp` is **incorrectly** ignored (existing presses cannot be released)

The joypad state is a one-shot latch: `set_button(key, true)` on KeyDown, `set_button(key, false)` on KeyUp. Missing a KeyUp event leaves the button permanently latched.

## Impact

| Scenario | Impact |
|----------|--------|
| Player holds direction + opens F1 | Direction stuck after closing F1 |
| Player holds attack + opens F1 | Attack button stuck after closing F1 |
| Player taps F1 quickly (no key held) | No impact |
| Xbox controller user | No impact (controller events are not guarded) |

## Proposed Fix Direction

Remove the `if !show_help` guard from the `KeyUp` handler only. Key releases must always be processed to keep joypad state consistent, even when the overlay is visible.

```rust
// Line 379 — remove guard
Event::KeyUp { keycode: Some(key), .. } => {
    // Always process key releases
}
```

Alternatively, on F1 overlay open, release all keyboard keys:

```rust
Event::KeyDown { keycode: Some(Keycode::F1), .. } => {
    show_help = !show_help;
    if show_help {
        // Release all P2 keys to prevent stuck input
        for btn in &[JoypadBtnState::UP, DOWN, LEFT, RIGHT, A, B, START, SELECT] {
            deck.joypad_mut(Player::Two).set_button(*btn, false);
        }
    }
}
```

## Classification

- **Type:** Input state leak
- **Introduced in:** Current uncommitted changes (F1 help overlay feature)
- **Lines:** 364, 379 in `src/main.rs`

---

*Report generated: 2026-06-03*
*Analysis method: Static code analysis — runtime reproduction pending*
