# TurtleBox v0.2 Release Checklist

## 1. Git Status

| Item | Status |
|------|--------|
| Branch | `master` |
| Uncommitted changes | Yes — Sprint 2-5 source + docs not yet committed |
| Untracked files | Yes — new modules (state.rs, config.rs) + reports |
| Last commit | `01deffc` — Sprint 1 refactor |
| Tag `v0.1` | Exists |
| Tag `v0.2` | **NOT YET CREATED** |

**Action required:** Commit all v0.2 changes, create `v0.2` tag before release.

## 2. Tag Status

| Tag | Commit | Status |
|-----|--------|--------|
| v0.1 | exists | OK |
| v0.2 | — | **Needs creation after commit** |

## 3. Release Documentation

| Document | Exists | Lines | Status |
|----------|--------|-------|--------|
| CHANGELOG.md | Yes | 37 | OK |
| RELEASE_NOTES_v0.2.md | Yes | 42 | OK |
| USER_GUIDE_v0.2.md | Yes | 94 | OK |
| V0.2_RELEASE_AUDIT.md | Yes | 114 | OK |
| V0.2_FINAL_STATUS.md | Yes | 81 | OK |
| SAVE_STATE_REPORT.md | Yes | 77 | OK |

## 4. User Guide Completeness

| Topic | Covered in USER_GUIDE_v0.2.md |
|-------|------|
| How to launch | Yes (3 methods) |
| How to load ROM | Yes |
| Controller usage | Yes |
| Keyboard usage | Yes |
| Save/Load state | Yes |
| All shortcuts | Yes (table) |
| Config customization | Yes (full example) |
| Troubleshooting | Yes (4 items) |

## 5. Config Defaults

| Field | Default | Verified |
|-------|---------|----------|
| fullscreen | `false` | Yes (config.rs) |
| volume | `100` | Yes (config.rs) |
| keyboard.a | `"A"` | Yes |
| keyboard.b | `"D"` | Yes |
| keyboard.start | `"RETURN"` | Yes |
| keyboard.select | `"RSHIFT"` | Yes |
| controller.a | `"LeftShoulder"` | Yes |
| controller.b | `"RightShoulder"` | Yes |
| controller.start | `"Start"` | Yes |
| controller.select | `"Back"` | Yes |

## 6. Save State Directory

| Item | Value |
|------|-------|
| Directory | `./saves/` |
| Auto-create | Yes (`create_dir_all`) |
| File name | `<rom_stem>.state` |
| Example | `TMNT3.state` |

## 7. Shortcut Consistency

Shortcuts in code match USER_GUIDE and RELEASE_NOTES:

| Key | Code (main.rs) | USER_GUIDE | RELEASE_NOTES | Overlay (F1) |
|-----|----------------|------------|---------------|--------------|
| ESC | Quit | Yes | Yes | Yes |
| F1 | Help | Yes | Yes | — |
| F5 | Save | Yes | Yes | Yes |
| F8 | Load | Yes | Yes | Yes |
| F11 | Fullscreen | Yes | Yes | Yes |
| P | Pause | Yes | Yes | Yes |
| = | Vol+ | Yes | Yes | Yes |
| - | Vol- | Yes | Yes | Yes |

## Build Gate

| Check | Result |
|-------|--------|
| `cargo fmt` | PASS |
| `cargo build --release` | PASS |
| `cargo clippy` | PASS (0 warnings) |

## Blocking Issues

| Issue | Blocks Release? |
|-------|----------------|
| v0.2 not committed | **Yes** — must commit before tagging |
| v0.2 tag not created | **Yes** — must tag before release |
| block v0.1.6 warning | No — pre-existing indirect dep |
| .DS_Store untracked | No — add to .gitignore |

## Verdict

**BLOCKED on git commit + tag.** All code, tests, and documentation are ready. Once committed and tagged, v0.2 is release-ready.
