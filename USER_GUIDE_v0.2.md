# TurtleBox v0.2 User Guide

## Getting Started

### Starting TurtleBox

**Option 1:** Double-click the app. A file dialog will appear to select your ROM.

**Option 2:** Drag a `.nes` ROM file onto the TurtleBox app.

**Option 3:** From terminal:
```
./turtlebox /path/to/TMNT3.nes
```

If a ROM file named `TMNT3.nes` is placed next to the executable, it loads automatically.

### Controls

#### Xbox Controller (Player 1)
- **DPad / Left Stick** — Move
- **LB (Left Shoulder)** — Jump (NES A)
- **RB (Right Shoulder)** — Attack (NES B)
- **Start** — Start (short press) / Pause (hold 1 second)
- **Back** — Select

#### Keyboard (Player 2)
- **Arrow Keys** — Move
- **A** — Jump (NES A)
- **D** — Attack (NES B)
- **Enter** — Start
- **Right Shift** — Select

### System Shortcuts

| Key | Action |
|-----|--------|
| F1 | Show/hide controls overlay |
| F5 | Save game state |
| F8 | Load game state |
| F11 | Toggle fullscreen |
| P | Pause / resume |
| = | Volume up 5% |
| - | Volume down 5% |
| ESC | Quit |

## Save States

Press **F5** at any time to save your current game progress. A "STATE SAVED" message appears at the top of the screen for 2 seconds.

Press **F8** to load your last save. A "STATE LOADED" confirms the restore.

If no save exists, pressing F8 shows "LOAD FAILED" — no crash, no side effects.

Save files are stored in `./saves/` next to the executable.

## Configuration

TurtleBox creates a `config.toml` file on first launch. You can edit it to customize:

```toml
fullscreen = false
volume = 100

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

Changes to `config.toml` take effect on next launch. Volume and fullscreen changes made via keyboard shortcuts are saved automatically.

### Custom Key Mapping

To remap keyboard keys, edit the `[keyboard]` section. Values are SDL2 key names (e.g., `"SPACE"`, `"X"`, `"LSHIFT"`, `"F2"`).

To remap controller buttons, edit the `[controller]` section. Values are SDL2 button names (e.g., `"A"`, `"B"`, `"X"`, `"Y"`, `"LeftStick"`).

## Troubleshooting

**No audio:** Check system volume. TurtleBox volume can be adjusted with `=` and `-`.

**Controller not detected:** Connect before launching, or hot-plug is supported during gameplay.

**Game runs too fast/slow:** TurtleBox targets NTSC 60fps. Frame timing is enforced automatically.

**Config not loading:** Delete `config.toml` and restart. Defaults will be regenerated.
