# PROJECT ROADMAP — TurtleBox

## Strategy: Vibe Coding Approach

Traditional emulator development builds every hardware component from scratch (CPU → PPU → APU → Mapper). This takes months and produces nothing playable until the very end.

TurtleBox uses a **core-integration strategy**: embed a mature NES emulator core, then build the user-facing shell around it. TMNT3 becomes playable in weeks, not months.

**Build order:** Shell First → Core Integration → Playability → Polish → Research

Each phase produces a runnable binary.

---

## Phase 0: Development Environment

**Goal:** Rust + SDL2 toolchain ready.

**Tasks:**
- Install Rust via rustup
- Install SDL2 via Homebrew
- Verify `cargo build` works with `sdl2` crate

**Verification:**
- `rustc --version` prints version
- `cargo build` succeeds with zero errors
- SDL2 links correctly

---

## Phase 1: Window System

**Goal:** SDL2 window with event loop.

**Tasks:**
- Initialize SDL2 video subsystem
- Create 960x720 window titled "TurtleBox"
- Run event loop
- Handle ESC key to quit cleanly

**Verification:**
- `cargo run` opens a window
- Window is 960x720 pixels
- Title bar reads "TurtleBox"
- Pressing ESC closes the application

---

## Phase 2: Input System

**Goal:** Keyboard and Xbox controller input.

**Tasks:**
- SDL2 event polling for keyboard
- SDL2 GameController API for Xbox controller
- Detect controller connection/disconnection
- Print key/button events to console for debugging

**Verification:**
- Keyboard key presses print to console
- Xbox controller button presses print to console
- Controller hot-plug is detected
- ESC still quits

**Forbidden:** NES controller protocol, $4016/$4017 registers — that's the core's job.

---

## Phase 3: ROM Browser

**Goal:** Select and inspect .nes files without executing them.

**Tasks:**
- Open file dialog (native macOS via `rfd` crate or SDL2)
- Filter for .nes files
- Parse iNES header (16 bytes): PRG size, CHR size, mapper number, mirroring
- Display ROM metadata in window or console

**Verification:**
- File picker opens and filters .nes files
- Selecting tmnt3.nes prints: mapper=4, PRG=128KB, CHR=128KB
- Invalid files show error message
- ROM is NOT executed — inspection only

**Forbidden:** Running any ROM, loading into emulator core.

---

## Phase 4: Emulator Core Evaluation

**Goal:** Choose the best existing NES core for integration.

**Research Targets:**

| Core | Language | Rust Binding | Notes |
|------|----------|-------------|-------|
| Mesen | C++ | FFI possible | Most accurate, active dev |
| FCEUX | C/C++ | FFI possible | Mature, widely used |
| Nestopia | C++ | FFI possible | Good accuracy, clean code |

**Evaluation Criteria:**

1. **TMNT3 Compatibility** — Does it run TMNT3 without glitches?
2. **Rust Integration** — Can we bind via FFI? Is there a Rust crate wrapper?
3. **macOS Support** — Builds on ARM Mac? SDL2 output?
4. **Maintenance** — Last commit? Active community?
5. **Audio Quality** — APU output quality for TMNT3 music
6. **Save State** — Built-in snapshot support?

**Deliverable:** `docs/core-evaluation.md` with comparison table and recommendation.

**Forbidden:** Writing our own CPU, PPU, APU, or Mapper code.

---

## Phase 5: TMNT3 Boot

**Goal:** TMNT3 title screen appears on macOS.

**Tasks:**
- Integrate chosen emulator core via FFI or crate
- Load tmnt3.nes ROM file
- Hook core video output to SDL2 texture (256x240 → 960x720)
- Hook core audio output to SDL2 audio device
- Hook core input to SDL2 keyboard/gamepad events
- Run emulation loop at 60 FPS

**Verification:**
- TMNT3 title screen renders correctly
- Title screen music plays
- Pressing Start navigates to character select
- Can select turtles and begin Stage 1
- Audio is synced and clean

**This is the first milestone where TMNT3 is interactive.**

---

## Phase 6: Save State

**Goal:** Save and restore game progress.

**Tasks:**
- Use core's built-in save state API (if available)
- Serialize state to file
- Deserialize and restore from file
- Keybind: F5 = save, F8 = load
- Multiple save slots (1-9 via number keys)
- Save state thumbnail preview (optional)

**Verification:**
- Save during gameplay
- Close emulator
- Reopen, load state — game resumes at exact frame
- All 9 slots work independently

---

## Phase 7: Fullscreen + UI Polish

**Goal:** Apple Design + Swiss Style interface.

**Tasks:**
- Fullscreen toggle (F11 or Alt+Enter)
- Pause menu overlay
- File picker for ROM selection (replaces console path)
- FPS counter (toggle with F1)
- Error messages for invalid/unplayable ROMs
- Window icon
- Clean typography and layout

**Verification:**
- Fullscreen toggles without visual artifacts
- Pause menu is readable and functional
- File picker works for ROM selection
- FPS counter displays correctly

---

## Phase 8: Performance Optimization

**Goal:** Stable 60 FPS, low CPU usage.

**Tasks:**
- Profile CPU usage
- Optimize texture upload (SDL2 texture streaming)
- Minimize audio latency
- Check for frame drops during heavy scenes
- Memory usage audit

**Verification:**
- 60 FPS stable during all TMNT3 stages
- CPU usage < 30% on Apple Silicon
- No audio pops or desync
- Memory usage < 200MB

---

## Phase 9: Optional Research (Post-Playable)

**Only after TMNT3 is fully playable (Phase 5+ complete).**

**Goal:** Understand the hardware for future work.

**Research Topics:**

1. **CPU (6502)** — Instruction set, addressing modes, timing
2. **PPU** — Tile rendering, sprite system, scrolling
3. **Mapper (MMC3)** — Bank switching, IRQ counter
4. **APU** — Pulse, triangle, noise channels

**Deliverables:**
- `docs/research/cpu-6502.md`
- `docs/research/ppu.md`
- `docs/research/mapper-mmc3.md`
- `docs/research/apu.md`

**Use:** Education, potential future self-built core, better debugging.

**Forbidden:** Replacing the integrated core with custom code unless there's a compelling reason.

---

## Timeline Estimate

| Phase | Duration | Cumulative |
|-------|----------|-----------|
| Phase 0 | 30 min | 30 min |
| Phase 1 | 1 hour | 1.5 hours |
| Phase 2 | 1 hour | 2.5 hours |
| Phase 3 | 2 hours | 4.5 hours |
| Phase 4 | 1-2 days | ~2 days |
| Phase 5 | 2-3 days | ~5 days |
| Phase 6 | 1 day | ~6 days |
| Phase 7 | 2 days | ~8 days |
| Phase 8 | 1 day | ~9 days |
| Phase 9 | Ongoing | — |

**TMNT3 playable by Phase 5: ~5 days.**
