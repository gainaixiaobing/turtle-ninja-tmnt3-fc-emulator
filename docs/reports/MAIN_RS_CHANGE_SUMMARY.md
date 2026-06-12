# main.rs Change Summary — Sprint 4

## Line Count

| Version | Lines |
|---------|-------|
| Sprint 3 | 322 |
| Sprint 4 | 333 |
| Delta | +11 |

## New Module Declarations

```rust
mod state;  // Sprint 4: GameState (pause, fullscreen, volume)
```

## New State Variables (in main)

| Variable | Type | Purpose |
|----------|------|---------|
| `app_config` | `mut AppConfig` | Was `let app_config` (immutable), now mutable for config writes |
| `gs` | `GameState` | Encapsulates all Sprint 4 runtime state |

## Extracted Functions (in state.rs, NOT in main.rs)

| Function | Lines | Purpose |
|----------|-------|---------|
| `GameState::new()` | 14-23 | Constructor from volume config |
| `toggle_pause()` | 28-36 | Toggle pause + clear audio buffer |
| `controller_start_down()` | 39-42 | Record Start press time |
| `controller_start_up()` | 45-48 | Clear Start press tracking |
| `tick_controller_pause()` | 51-64 | Check long-press ≥1s → trigger pause |
| `toggle_fullscreen()` | 69-81 | Toggle Desktop fullscreen + save config |
| `apply_initial_fullscreen()` | 84-91 | Restore fullscreen from config on startup |
| `adjust_volume()` | 96-112 | ±5% volume + save config |
| `volume_factor()` | 115-117 | Returns 0.0..=1.0 scale factor |
| `show_volume_overlay()` | 120-122 | Whether volume HUD is visible |

## Event Loop Changes

### New system key handlers (always active, even during pause/help):
- `Keycode::Escape` → quit
- `Keycode::F1` → toggle help overlay
- `Keycode::F11` → toggle fullscreen
- `Keycode::P` → toggle pause
- `Keycode::Equals` → volume +5%
- `Keycode::Minus` → volume -5%

### New controller handling:
- `Button::Start` down → `gs.controller_start_down()`
- `Button::Start` up → `gs.controller_start_up()`
- `Button::Start` blocked from NES input when paused
- `ControllerAxisMotion` blocked when paused (match guard)

### Frame loop changes:
- Emulation skipped when `gs.paused`
- Audio samples scaled by `gs.volume_factor()` (100% skips multiplication)
- Pause overlay rendered when paused
- Volume overlay rendered for 1.5s after adjustment

## Technical Debt Assessment

- **main.rs grew only +11 lines** despite 3 new features
- All new logic lives in `state.rs` (140 lines)
- main.rs event loop is cleaner: system keys are a flat match, game keys are separate
- Audio callback fix (iterator) is a net improvement
- No new unwrap/panic paths introduced
