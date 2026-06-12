# Sprint 4 Final Report: Pause / Fullscreen / Volume

## Status: COMPLETE

## Build Verification

```
cargo fmt          → OK
cargo build --release → OK (3.05s)
cargo clippy       → 0 warnings
```

## Modified Files

| File | Lines | Role |
|------|-------|------|
| `src/state.rs` | 219 | New module: all Sprint 4+5 runtime state + logic |
| `src/main.rs` | 337 | System key handlers, pause gates, audio volume scaling |
| `src/overlay.rs` | 277 | Pause overlay, volume overlay, state overlay |
| `src/config.rs` | 123 | Unchanged (fullscreen/volume fields pre-existed) |
| `src/input.rs` | 281 | Unchanged |

## New Shortcuts

| Key | Action |
|-----|--------|
| `P` | Toggle pause/resume |
| `F11` | Toggle fullscreen (Desktop borderless) |
| `=` | Volume +5% (wraps 100→0) |
| `-` | Volume -5% (wraps 0→100) |
| Xbox `Start` (hold ≥1s) | Toggle pause/resume |

## Feature Verification (Code Review)

### A. Pause

| Check | Result |
|-------|--------|
| P key triggers `gs.toggle_pause()` | Verified (main.rs:204) |
| `deck.clock_frame()` skipped when paused | Verified (main.rs:273) |
| Audio buffer cleared on pause | Verified (state.rs:51) |
| PAUSED overlay rendered | Verified (main.rs:311-312) |
| Controller Start long-press ≥1s triggers pause | Verified (state.rs:62-67) |
| Short Start press still sends NES START | Verified (main.rs:234-238) |
| Event loop continues during pause (allows unpause) | Verified (event loop structure) |

### B. Fullscreen

| Check | Result |
|-------|--------|
| F11 triggers `gs.toggle_fullscreen()` | Verified (main.rs:203) |
| Uses `FullscreenType::Desktop` | Verified (state.rs:89) |
| `config.fullscreen` updated on toggle | Verified (state.rs:91,95) |
| `config.save()` called after toggle | Verified (state.rs:99) |
| Startup restores fullscreen from config | Verified (main.rs:115) |
| `apply_initial_fullscreen()` called before event loop | Verified (main.rs:115) |

### C. Volume

| Check | Result |
|-------|--------|
| `=` triggers `adjust_volume(5)` | Verified (main.rs:205) |
| `-` triggers `adjust_volume(-5)` | Verified (main.rs:206) |
| Range wraps: 0→100→0 | Verified (state.rs:116-122) |
| `config.volume` updated | Verified (state.rs:123) |
| `config.save()` called | Verified (state.rs:125) |
| Audio samples scaled by `volume/100.0` | Verified (main.rs:279-288) |
| 100% fast path (skip multiplication) | Verified (main.rs:283) |
| Volume overlay shown for 1.5s | Verified (state.rs:137-138) |
| Startup loads volume from config | Verified (main.rs:60) |

### D. Cross-Feature Verification

| Scenario | Result |
|----------|--------|
| F11 works during pause | Verified (system keys processed regardless of pause state) |
| Volume adjust works during pause | Verified (system keys not gated by `!gs.paused`) |
| Pause works during fullscreen | Verified (independent features, no coupling) |
| Overlays stack correctly (pause + volume + help) | Verified (main.rs:311-319, sequential rendering) |
| Volume persists across restart | Verified (config.save() + config.load()) |
| Fullscreen persists across restart | Verified (config.save() + apply_initial_fullscreen()) |

## Config Persistence

```toml
# config.toml — updated on every toggle/adjust
fullscreen = true   # ← F11 writes this
volume = 75         # ← = / - writes this

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

## main.rs Line Count Change

| Version | Lines | Delta |
|---------|-------|-------|
| Sprint 3 | 322 | — |
| Sprint 4 | 337 | +15 |

## Risk Assessment

| Risk | Level | Detail |
|------|-------|--------|
| Fullscreen on macOS | Low | `FullscreenType::Desktop` (borderless) is the safest option |
| Audio distortion at low volume | Low | Linear scaling, no clipping; 0% = silence via buffer clear |
| Controller Start conflict | Low | Short press → NES START; long press (≥1s) → emulator pause |
| Config file corruption | Low | Parse errors fall back to defaults; fire-and-forget save |
| Overlay z-ordering | Low | Sequential rendering: pause → volume → help, no blending conflicts |
