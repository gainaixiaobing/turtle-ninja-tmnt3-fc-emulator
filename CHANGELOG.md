# Changelog

## v0.2 (2026-06-07)

### Added
- **Pause** — Press `P` to pause/resume. Xbox `Start` hold ≥1s also works.
- **Fullscreen** — Press `F11` to toggle. Preference saved to config.
- **Volume** — Press `=`/`-` to adjust ±5%. Range 0-100%. Saved to config.
- **Save State** — Press `F5` to save, `F8` to load. Files in `./saves/`.
- **Config system** — `config.toml` for keyboard/controller mapping, volume, fullscreen.
- **Configurable input mapping** — `[keyboard]` and `[controller]` sections in config.
- **State module** — `src/state.rs` encapsulates pause, fullscreen, volume, save state logic.
- **Volume overlay** — Shows "VOL: XX%" briefly after adjustment.
- **Save state overlay** — Shows "STATE SAVED" / "STATE LOADED" / "LOAD FAILED" for 2 seconds.

### Changed
- `main.rs` refactored: system keys extracted to flat match, game keys separated.
- `input.rs` now accepts config references instead of hardcoded mappings.
- `overlay.rs` extended with pause, volume, and state overlays + numeric glyphs.
- `main.rs` grew from 318 to 337 lines (+19) despite 4 new features.

### Technical
- tetanes-core `ControlDeck::save_state()` / `load_state()` used for save state.
- Audio volume scaling: samples multiplied by `volume/100.0` before buffer push.
- Fullscreen uses `FullscreenType::Desktop` (borderless, macOS compatible).
- All error paths use safe handling — no panic, no unwrap in user-facing code.

## v0.1 (2026-06-03)

### Added
- Initial NES emulator using tetanes-core.
- Xbox controller support (P1).
- Keyboard support (P2): arrows, A/D, Enter, RShift.
- Audio playback (44.1kHz mono).
- Controls overlay (F1).
- ROM file dialog (rfd).
- Bundled ROM support (.app Resources).
