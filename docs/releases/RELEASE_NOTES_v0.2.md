# TurtleBox v0.2 Release Notes

## What's New

### Pause
Press `P` to pause or resume the game. Xbox controller users can hold `Start` for 1 second. When paused, the game is frozen and audio is silenced.

### Fullscreen
Press `F11` to toggle between windowed and fullscreen mode. Your preference is saved to `config.toml` and restored on next launch.

### Volume Control
Press `=` to increase volume by 5%, `-` to decrease. Range is 0-100%. Wraps around. Saved to config.

### Save State
Press `F5` to save the current game state, `F8` to load it. Saves are stored in `./saves/` as binary files. A confirmation overlay appears for 2 seconds.

### Configurable Input Mapping
Keyboard and controller button mappings are now configurable via `config.toml`. Default mappings match v0.1 behavior.

## Quick Reference

| Key | Action |
|-----|--------|
| F1 | Controls overlay |
| F5 | Save state |
| F8 | Load state |
| F11 | Fullscreen toggle |
| P | Pause/resume |
| = | Volume up 5% |
| - | Volume down 5% |
| ESC | Quit |

## System Requirements
- macOS (primary platform)
- SDL2
- NES ROM file (.nes)

## Dependencies
- tetanes-core 0.14
- SDL2 0.38
- serde + toml (config)
- rfd (file dialog)
