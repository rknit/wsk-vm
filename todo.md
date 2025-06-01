# TODO

---

- [ ] priviledges
- [ ] complete rv64imafdc
  - [ ] rv32i, rv64i (active WIP)
  - [x] rv64i
  - [x] rv32m, rv64m
  - [x] rv64m
  - [ ] rv32a, rv64a
  - [ ] rv64a
  - [ ] rv32f, rv64d
  - [ ] rv64f
  - [ ] rv64d
  - [ ] rv32c, rv64c
  - [ ] rv64c
- [x] gen_inst v2
  - [x] format struct 'FmtI', 'FmtR' ... (usage: 'fmt.r1(vm)', 'fmt.rs1', 'fmt.imm' etc.)
    - maybe FmtI, FmtR... can store ref mut VM? (usage: fmt.vm(), fmt.r1() instead of fmt.r1(vm))
  - [x] implementation migrating (new token '$IMPL_START' ... '$IMPL_END'?)
- [ ] load debug symbols
  - [ ] break at symbol
- [x] refactor repl
- [x] break at inst
- [x] inst generator
  - [x] module generator
  - [x] perserve instruction
