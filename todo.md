# TODO

---

- [ ] priviledges
  - [ ] u mode
  - [ ] s mode
  - [ ] m mode
- [ ] complete rv64imafdc
  - [-] rv32i, rv64i
  - [x] rv64i
  - [x] rv32m, rv64m
  - [x] rv64m
  - [-] rv32a, rv64a
  - [-] rv64a
  - [-] rv32f, rv64d
  - [-] rv64f
  - [-] rv64d
  - [ ] rv32c, rv64c
- [ ] 16 bits instruction mode
  - [ ] if not 0b11 in 32 bits inst mode, then check for 16 bits mode.
  - [ ] Inst::decode can behave as normal and top 16 bits is zeroed when decoding 16 bits inst
- [ ] load debug symbols
  - [ ] break at symbol
