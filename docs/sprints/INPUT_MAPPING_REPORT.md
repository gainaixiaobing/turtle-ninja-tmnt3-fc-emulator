# Sprint 3 Report: Config-Driven Input Mapping

## Status: COMPLETE

## Modified Files

| File | Lines | Change |
|------|-------|--------|
| `src/config.rs` | 93 → 123 | +`KeyboardConfig` struct, `#[serde(default)]` on both config fields |
| `src/input.rs` | 181 → 281 | +`keycode_name()`, `button_name()`, `match_keyboard_nes_button()`, `match_controller_nes_button()`, 4 handler signatures updated |
| `src/main.rs` | 322 → 322 | 5 lines changed: `_app_config` → `app_config`, 4 handler calls pass config refs |

## Default Mapping Table

### Keyboard (Player 2)

| NES Button | Default Key | Config Key |
|------------|-------------|------------|
| A (jump)   | A           | `keyboard.a` |
| B (attack) | D           | `keyboard.b` |
| START      | Return      | `keyboard.start` |
| SELECT     | RShift      | `keyboard.select` |
| Directions | ↑↓←→        | Hardcoded (not configurable) |

### Xbox Controller (Player 1)

| NES Button | Default Button      | Config Key |
|------------|---------------------|------------|
| A (jump)   | LeftShoulder (LB)   | `controller.a` |
| B (attack) | RightShoulder (RB)  | `controller.b` |
| START      | Start               | `controller.start` |
| SELECT     | Back                | `controller.select` |
| Directions | DPad + Left Stick   | Hardcoded (not configurable) |

## Config Format

```toml
[keyboard]
a = "A"
b = "D"
start = "RETURN"
select = "RSHIFT"

[controller]
a = "LeftShoulder"
b = "RightShoulder"
start = "Start"
select = "Back"
```

### Test Config Example (swapped A/B)

```toml
[keyboard]
a = "D"
b = "A"
start = "RETURN"
select = "RSHIFT"

[controller]
a = "RightShoulder"
b = "LeftShoulder"
start = "Start"
select = "Back"
```

## Fallback Mechanism

- `KeyboardConfig` and `ControllerConfig` both implement `Default`
- `AppConfig` fields use `#[serde(default)]` — old config.toml files without `[keyboard]` section load successfully with keyboard defaults
- Config file missing or unreadable → `AppConfig::default()` used, defaults saved to disk
- Config file has parse error → `eprintln!` warning + `AppConfig::default()` used
- No `panic!`, no `unwrap()` in config loading path
- Config value comparison is case-insensitive (`"leftshoulder"` matches `"LeftShoulder"`)

## Build Verification

```
cargo fmt        → OK
cargo build --release → OK (0.77s)
cargo clippy     → 0 new warnings (2 pre-existing from v0.1)
```

## Risk Assessment

- **Low risk**: `#[serde(default)]` ensures backward compatibility with existing config.toml
- **Low risk**: Case-insensitive comparison means config values like `"leftshoulder"` or `"RETURN"` all work
- **No risk**: Direction keys remain hardcoded and unchanged
- **No risk**: Overlay (F1), audio, axis input all untouched

## What Was NOT Changed

- No new features
- No Pause / Fullscreen / Volume / Save State
- No overlay modifications
- No axis input (Left Stick) changes
- No new dependencies added
