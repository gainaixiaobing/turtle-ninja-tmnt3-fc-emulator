# Save State Report

## Status: COMPLETE

## API Calls

| API | Location | Purpose |
|-----|----------|---------|
| `ControlDeck::save_state(path)` | tetanes-core `control_deck.rs:528` | Serialize entire CPU state tree to file |
| `ControlDeck::load_state(path)` | tetanes-core `control_deck.rs:541` | Deserialize and restore CPU state tree |
| `fs::save(path, &cpu)` | tetanes-core `fs.rs:142` | bincode + deflate compression |
| `fs::load::<Cpu>(path)` | tetanes-core `fs.rs:160` | Header validation + decompress + bincode decode |

## Modified Files

| File | Lines | Change |
|------|-------|--------|
| `src/state.rs` | 150→219 | +`save_state()`, `load_state()`, `set_state_overlay()`, `show_state_overlay()`, `state_overlay_message()`, `state_path()` |
| `src/main.rs` | 337 | +F5/F8 key handlers (2 lines), +state overlay render (3 lines) |
| `src/overlay.rs` | 260→277 | +`render_state_overlay()` function |

## Implementation Details

### Save: `GameState::save_state()`
- Computes path: `./saves/<rom_stem>.state`
- Creates `./saves/` directory if missing (`create_dir_all`)
- Calls `deck.save_state(&path)`
- On success: shows "STATE SAVED" overlay for 2 seconds
- On failure: shows "SAVE FAILED", logs error to stderr
- No panic, no unwrap

### Load: `GameState::load_state()`
- Checks file exists before calling `deck.load_state()`
- If file missing: shows "LOAD FAILED" (safe failure)
- On success: shows "STATE LOADED" overlay for 2 seconds
- On failure: shows "LOAD FAILED", logs error to stderr
- No panic, no unwrap

### Save File Format
```
[TETANES\x1a][version 1][deflate(bincode(Cpu))]
```
- Binary format, ~3-8 KB per save
- Contains: CPU registers, PPU state (OAM+VRAM), APU state, WRAM (2KB), Mapper state

### Overlay
- Green text centered at top of screen
- Semi-transparent black background
- Auto-dismisses after 2 seconds

## Build Verification

```
cargo fmt          → OK
cargo build --release → OK (0.16s)
cargo clippy       → 0 warnings
```

## Manual Test Checklist

| Test | Expected |
|------|----------|
| F5 during gameplay | "STATE SAVED" overlay, file created in `./saves/` |
| F8 after F5 | "STATE LOADED" overlay, game returns to saved point |
| F8 without prior save | "LOAD FAILED" overlay, no crash |
| Delete state file, press F8 | "LOAD FAILED" overlay, no crash |
| Close and reopen, press F8 | "STATE LOADED" if file exists |
| Pause + F5 | Works (system keys active during pause) |

## Risk Assessment

| Risk | Level | Detail |
|------|-------|--------|
| tetanes-core API stability | Low | Upstream library is mature, save/load is core feature |
| Cross-platform save files | Low | Binary format with fixed header, platform-independent |
| Save file corruption | Low | Header validation + deflate CRC |
| tetanes-core version lock | Medium | Upgrading tetanes-core could break save compatibility |
