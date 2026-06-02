# PROJECT GUARDRAILS — TurtleBox

## Project Goal

Build a desktop NES emulator that runs **Teenage Mutant Ninja Turtles III: The Manhattan Project** with playable performance. Not a general-purpose emulator — a focused, polished experience for one game.

## Tech Stack

- Language: **Rust**
- Graphics/Audio/Input: **SDL2**
- Target Platform: **macOS** (primary), Linux/Windows (secondary)

## Development Principles

### Vibe Coding Rules

1. **One module per phase.** Never implement code from a future phase.
2. **Every phase must compile and run.** No broken intermediate states.
3. **Test before commit.** Every phase has explicit acceptance criteria.
4. **No premature abstraction.** Write the simplest code that works for TMNT3.
5. **No generalization.** If TMNT3 doesn't use a mapper/opcode/feature, skip it.
6. **Commit after each phase.** Clear, atomic commits with descriptive messages.
7. **No TODOs in code.** Either implement it or don't include it.
8. **Read before write.** Understand the hardware spec before coding.

### Code Quality

- `cargo build` must pass with zero warnings.
- `cargo test` must pass before any commit.
- No `unsafe` unless absolutely necessary and documented why.
- No `unwrap()` in production code — use proper error handling.

## Phase Acceptance Criteria

Every completed phase must deliver:

1. **Working binary** — `cargo run` produces the expected result.
2. **Tests** — unit tests for core logic.
3. **Documentation** — brief implementation notes.
4. **Commit** — clean git history.

## Forbidden Scope

The following are explicitly **out of scope** for TurtleBox:

- General NES compatibility (other games)
- Netplay / multiplayer over network
- Cheat codes / Game Genie
- Video recording / streaming
- Custom shader pipelines
- Debugger / disassembler UI
- ROM header editor
- iNES 2.0 full support (only what TMNT3 needs)

## Architecture Boundaries

| Layer | Responsibility | Forbidden |
|-------|---------------|-----------|
| CPU | 6502 instruction execution | Cycle-accurate timing beyond TMNT3 needs |
| Cartridge | ROM loading, PRG/CHR banks | iNES 2.0 extended headers |
| Mapper | MMC3 bank switching | Other mappers |
| PPU | Tile rendering, sprites, scrolling | Full PPU open bus behavior |
| Input | Keyboard + Xbox controller | Mouse, touch, custom peripherals |
| Audio | APU channels for TMNT3 | Full APU emulation |
| Save State | Snapshot/restore | Rewind, netplay sync |
| UI | Window, menus, file picker | Complex settings panels |
