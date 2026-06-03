# Controls Overlay Implementation

## Summary

F1 key toggles a controls help overlay on/off. Uses a bitmap font renderer (no new dependencies).

## What Was Added

### State

- `show_help: bool` — toggled by F1

### Functions

| Function | Purpose |
|----------|---------|
| `char_bitmap(c: char) -> [u8; 7]` | Returns 5x7 pixel bitmap for each character |
| `draw_char()` | Renders one character via `fill_rect()` |
| `draw_text()` | Renders a string by calling `draw_char()` per character |
| `render_help_overlay()` | Draws the full help screen |

### Event Handling

- **F1 KeyDown** — toggles `show_help`, does NOT propagate to game input
- **All other keyboard events** — guarded by `if !show_help`, blocked while overlay is visible

### Rendering

Overlay is drawn after the game frame (`canvas.copy`) and before `canvas.present()`:

```
canvas.clear();
canvas.copy(&texture, None, None);   // game frame
if show_help {
    render_help_overlay(&mut canvas); // overlay on top
}
canvas.present();
```

## Design Decisions

1. **No SDL2_ttf** — only ~15 fixed strings needed; a 5x7 bitmap font (66 chars) avoids the system library dependency and font path issues
2. **No new dependencies** — `Cargo.toml` unchanged; uses only `canvas.fill_rect()` and `canvas.set_draw_color()`
3. **No input leak** — F1 is intercepted before the game key handler; all other keys blocked while overlay is shown
4. **No audio/input logic changes** — controller and audio code untouched

## Controls Displayed

**Keyboard (P2):**
- Arrow Keys - Move
- A - Jump (NES A)
- D - Attack (NES B)
- Enter - Start
- Right Shift - Select
- F1 - Toggle This Help
- ESC - Quit

**Xbox Controller (P1):**
- DPad/LStick - Move
- LB - Jump (NES A)
- RB - Attack (NES B)
- Start - Start
- Back - Select

## Files Modified

| File | Change |
|------|--------|
| `src/main.rs` | Added `show_help`, F1 handler, bitmap font functions, overlay rendering |
