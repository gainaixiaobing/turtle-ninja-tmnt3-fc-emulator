# CURRENT PHASE — TurtleBox

## Phase 3: ROM Browser

### Goal

NES ROM file selection and metadata reading. No execution.

### Tasks

- [ ] Add file dialog for .nes files
- [ ] Parse iNES header (16 bytes)
- [ ] Extract ROM name
- [ ] Calculate PRG ROM size
- [ ] Calculate CHR ROM size
- [ ] Identify mapper number
- [ ] Detect mirroring type
- [ ] Check battery backed RAM
- [ ] Check trainer present
- [ ] Display metadata in terminal
- [ ] Handle invalid iNES files

### Acceptance Criteria

File Selection:
- [ ] Can open file dialog
- [ ] Filters for .nes files

iNES Header Parsing:
- [ ] ROM name displayed
- [ ] File size displayed
- [ ] PRG ROM size calculated correctly
- [ ] CHR ROM size calculated correctly
- [ ] Mapper number identified
- [ ] Mapper name shown (e.g., MMC3)
- [ ] Mirroring type detected
- [ ] Battery flag detected
- [ ] Trainer flag detected

Error Handling:
- [ ] Invalid iNES magic number → error message
- [ ] Unsupported format → error message

### Forbidden

Do **not** implement any of the following:

- CPU / 6502 emulation
- PPU / graphics rendering
- APU / audio
- Mapper implementation
- ROM execution
- Emulator core

### Exit Criteria

When all acceptance criteria pass, commit and proceed to Phase 4.
