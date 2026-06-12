# NEXT DEVELOPER HANDOFF — TurtleBox

**Date:** 2026-06-05  
**Project:** TurtleBox / 忍者神龟3 FC 模拟器  
**Current version line:** v0.1 baseline, with partial v0.2 refactor work in progress

This document is the fastest way for the next developer to understand the current state of the repository. Read this before changing code.

---

## 1. Project Goal

TurtleBox is a focused desktop NES/FC emulator wrapper for **Teenage Mutant Ninja Turtles III: The Manhattan Project**. It is not intended to become a general-purpose emulator.

The emulator core comes from `tetanes-core`; this project owns the desktop shell around it:

- ROM selection and validation
- SDL2 window/video/audio/input loop
- keyboard and Xbox controller mapping
- F1 controls overlay
- macOS-oriented app packaging

Primary platform is macOS. Linux/Windows may work later, but current assumptions are macOS-heavy.

---

## 2. Current Repository State

Important: the repository is no longer exactly the pure single-file v0.1 described in some older reports.

Current source files:

```text
src/main.rs     Main entry point, SDL2 setup, ROM loading, audio callback, render loop
src/input.rs    Keyboard/controller input mapping helpers
src/overlay.rs  5x7 bitmap font and F1 help overlay renderer
```

Current package file:

```text
Cargo.toml      turtlebox 0.1.0, Rust 2021
```

Dependencies:

```toml
sdl2 = "0.38"
rfd = "0.14"
tetanes-core = "0.14"
```

There are many project reports in the root directory. The most useful ones are:

- `README.md` — user-facing build/run instructions
- `PROJECT_GUARDRAILS.md` — scope boundaries and development rules
- `PROJECT_HANDOVER.md` — earlier handover context
- `AUDIT_V0.1.md` — full v0.1 audit
- `PROJECT_HEALTH.md` — health assessment
- `V0.2_EXECUTION_PLAN.md` — recommended next work
- `REFACTOR_SPRINT1_REPORT.md` — likely describes the current partial refactor

---

## 3. Git / Working Tree Notes

At the time this handoff was created, the working tree contained uncommitted changes and untracked files.

Observed status:

```text
 M src/main.rs
?? .DS_Store
?? AUDIT_V0.1.md
?? REFACTOR_SPRINT1_REPORT.md
?? RELEASE_NOTES_v0.1.md
?? TMNT3.nes
?? V0.1_FINAL_STATUS.md
?? V0.2_EXECUTION_PLAN.md
?? src/input.rs
?? src/overlay.rs
?? 忍者神龟3.app/
```

Do not assume all Markdown files are committed. Do not blindly clean the working tree; some files are valuable project documentation.

Do not commit:

- `.DS_Store`
- `TMNT3.nes` or any commercial ROM
- `忍者神龟3.app/` unless packaging artifacts are explicitly intended to be versioned
- `target/`

Recommended `.gitignore` expansion:

```gitignore
/target
.DS_Store
*.nes
*.app/
```

Review whether `spritecans.nes` is a legal test ROM before keeping or publishing it.

---

## 4. How To Build And Run

Requirements:

- Rust toolchain
- SDL2 installed locally
- On Apple Silicon macOS, `.cargo/config.toml` currently points Rust linker search path at `/opt/homebrew/lib`

Build:

```bash
cargo build
```

Run with explicit ROM:

```bash
cargo run --release -- /path/to/TMNT3.nes
```

Run without explicit ROM:

```bash
cargo run --release
```

ROM lookup order:

1. CLI argument
2. bundled `TMNT3.nes` in `.app/Contents/Resources`
3. bundled `spritecans.nes` in `.app/Contents/Resources`
4. native file picker via `rfd`

The ROM must exist and end in `.nes`.

---

## 5. Runtime Behavior

Window:

- 960x720 SDL2 window
- NES texture is 256x240 RGBA
- Window title includes selected ROM filename

Video loop:

- `deck.clock_frame()`
- `deck.frame_buffer()`
- update SDL2 streaming texture
- copy texture to canvas
- optionally draw F1 overlay
- present
- sleep to approximate NTSC frame duration

Audio loop:

- `deck.audio_samples()` are appended to `Arc<Mutex<Vec<f32>>>`
- SDL2 audio callback drains samples from that vector
- output is 44100 Hz, mono, 1024 samples

Inputs:

- Player 1: Xbox controller
- Player 2: keyboard
- F1 toggles help overlay
- ESC exits

Current key mappings:

```text
P2 keyboard:
Arrows       Move
A            NES A / jump
D            NES B / attack
Enter        Start
Right Shift  Select

P1 controller:
DPad         Move
Left Stick   Move
LB           NES A / jump
RB           NES B / attack
Start        Start
Back         Select
```

---

## 6. Code Architecture

`src/main.rs`

- owns application startup and shutdown
- parses CLI args
- resolves bundled ROM resources
- validates ROM file path and extension
- initializes SDL2 video/audio/controller systems
- creates `ControlDeck`
- loads ROM
- owns the SDL2 event loop
- owns audio buffer and `NesAudioCallback`
- owns render loop and frame pacing

`src/input.rs`

- maps SDL2 keyboard events to `Player::Two`
- maps SDL2 controller button events to `Player::One`
- maps controller left stick axes to NES directional buttons
- contains `STICK_THRESHOLD = 16000`

Current limitation: the down/up mappings are still duplicated. A future cleanup should replace them with mapping tables or helper functions.

`src/overlay.rs`

- contains the 5x7 bitmap character table
- renders characters and text manually with SDL2 rectangles
- renders the F1 controls overlay

Current limitation: drawing errors are still ignored with `let _ = canvas.fill_rect(...)`.

---

## 7. Known Strengths

- The emulator core is delegated to `tetanes-core`, avoiding unnecessary CPU/PPU/APU work.
- Runtime data flow is simple: ROM -> `ControlDeck` -> video/audio/input.
- Core v0.1 feature set works: ROM load, video, audio, keyboard P2, Xbox P1, help overlay.
- Documentation is unusually strong for a small project.
- Scope is well constrained by `PROJECT_GUARDRAILS.md`.
- The recent split into `input.rs` and `overlay.rs` is a good direction and should continue.

---

## 8. Known Risks And Technical Debt

Highest priority:

- `main.rs` still owns too much. Audio, ROM loading, and runtime loop are still mixed together.
- No automated tests exist.
- Rendering code in `overlay.rs` silently discards SDL2 draw errors.
- Audio callback uses `Mutex::lock().unwrap()`, which can panic if the mutex is poisoned.
- Audio buffer is an unbounded `Vec<f32>`; if production outruns playback, memory can grow.
- Frame pacing uses `thread::sleep`, not SDL2 vsync.
- `.cargo/config.toml` is Apple Silicon/Homebrew-specific because it hardcodes `/opt/homebrew/lib`.
- Only one controller is opened. P2 cannot currently use a second controller.

Input-specific caution:

- The old L1 stuck-key bug was caused by ignoring `KeyUp` while the F1 overlay was open.
- Current code correctly processes `KeyUp` even when `show_help` is true.
- If adding pause/menu/input blocking, always keep release events flowing or explicitly clear latched joypad state.

Repository hygiene:

- ROM files should not be committed.
- `.DS_Store` should not be committed.
- macOS `.app` bundle is currently untracked build output.

---

## 9. Recommended Next Steps

Suggested order:

1. Verify the current partial refactor compiles.

```bash
cargo build
```

2. Run a short smoke test with a local ROM.

```bash
cargo run --release -- ./TMNT3.nes
```

3. Finish Sprint 1 cleanup.

- Replace overlay `let _ = canvas.fill_rect(...)` with visible error handling or propagated errors.
- Consider SDL2 `present_vsync()` for the canvas builder, then remove manual sleep if behavior is stable.
- Reduce duplicate input mapping code.

4. Continue module split.

Suggested target layout:

```text
src/
  main.rs      Entrypoint and high-level app loop only
  audio.rs     Audio callback and sample buffer
  input.rs     Input mapping and controller state
  overlay.rs   Text/overlay renderer
  rom.rs       ROM path resolution and validation
```

5. Add a small test base.

Good first tests:

- key mapping table maps expected keys to expected NES buttons
- controller mapping table maps expected buttons to expected NES buttons
- ROM validation rejects non-`.nes` paths
- overlay character bitmap returns blank for unsupported characters

6. Add user-facing v0.2 features after cleanup.

Best low-risk features:

- Pause with `P`
- Fullscreen with `F11`
- Volume control with `+` / `-`
- FPS overlay for development builds

Save State is valuable, but should come after the code is less centralized.

---

## 10. Do Not Do Without Discussion

- Do not replace `tetanes-core` with a custom emulator core.
- Do not broaden scope into a general NES emulator.
- Do not commit commercial ROMs.
- Do not delete existing Markdown reports just because there are many.
- Do not reset or clean the working tree without checking which files are intentional.
- Do not add large UI frameworks for the overlay; the current bitmap renderer is sufficient.
- Do not introduce broad configuration before deciding the minimal v0.2 settings surface.

---

## 11. Quick Mental Model

Think of this project as a small SDL2 shell around `tetanes-core`:

```text
SDL2 events
  -> input.rs
  -> ControlDeck joypads
  -> ControlDeck clock_frame()
  -> frame_buffer -> SDL2 texture -> window
  -> audio_samples -> SDL2 audio callback
```

The next developer's job is not to make the emulator bigger. The job is to keep the focused TMNT3 experience stable while gradually extracting the obvious modules and adding only the user-facing features that make the game more playable.

---

## 12. Suggested First Commit For Next Developer

If taking over from this state, a clean first commit would be:

```text
Refactor input and overlay modules
```

Include:

- `src/main.rs`
- `src/input.rs`
- `src/overlay.rs`
- `REFACTOR_SPRINT1_REPORT.md` if it accurately describes the change
- possibly this file, `NEXT_DEVELOPER_HANDOFF.md`

Exclude:

- `.DS_Store`
- ROM files
- `.app` bundle
- `target/`

Before committing, run:

```bash
cargo fmt
cargo build
```

