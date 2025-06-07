# TODO

---

## Features

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
  - [-] rv64c
- [ ] load debug symbols
  - [ ] break at symbol

## Optimizations

- [-] tree-based decoder (tri-ary tree)
  - [x] phase 1: simple tri-ary match.
    - tree height: 16/32
- [ ] copying implementations into run function (to avoid making a stackframe)
  - [ ] split run function into another file (it will be lengthy).
- [ ] prefetching instructions (branch predictions)
