# TODO

---

- [ ] priviledges
- [x] gen_inst v2
    -- [x] format struct 'FmtI', 'FmtR' ... (usage: 'fmt.r1(vm)', 'fmt.rs1', 'fmt.imm' etc.)
        --- maybe FmtI, FmtR... can store ref mut VM? (usage: fmt.vm(), fmt.r1() instead of fmt.r1(vm))
    -- [x] implementation migrating (new token '$IMPL_START' ... '$IMPL_END'?)
- [ ] load debug symbols
    -- [ ] break at symbol
- [x] refactor repl
- [x] break at inst
- [x] inst generator
    -- [x] module generator
    -- [x] perserve instruction
