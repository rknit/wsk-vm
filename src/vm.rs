use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self, Write},
};

use log::{info, log_enabled, trace};

use crate::{Exception, Inst, InstReport, RawFormat};

const REG_COUNT: usize = 32;

const MEM_LEN: usize = 64 * MEGABYTE;

const STACK_BEGIN: u64 = MEM_LEN as u64 - (8 * MEGABYTE) as u64;
const STACK_LEN: u64 = 8 * MEGABYTE as u64;

const PROG_LEN: usize = MEM_LEN - (STACK_LEN as usize);
const PROG_BEGIN: usize = 0;

const MEGABYTE: usize = 1024 * 1024;

pub struct VM {
    regs: [u64; REG_COUNT],
    mem: Box<[u8]>,
    pub pc: usize,

    pub(crate) halt: bool,
    pub(crate) exit_code: u8,

    dbg_syms: HashMap<usize, HashSet<String>>,
}
impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
impl VM {
    pub fn new() -> Self {
        Self {
            regs: Default::default(),
            mem: vec![0; MEM_LEN].into_boxed_slice(),
            pc: PROG_BEGIN,

            halt: false,
            exit_code: 0,

            dbg_syms: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.halt = false;
        self.pc = PROG_BEGIN;
        self.set_x(2, STACK_BEGIN);
    }

    pub fn halted(&self) -> bool {
        self.halt
    }

    pub fn exit_code(&self) -> u8 {
        self.exit_code
    }

    pub fn load_executable_bytes(&mut self, bytes: &[u8]) -> Result<(), VMLoadError> {
        use goblin::Object;
        use goblin::elf::section_header as sh;

        let obj = match Object::parse(bytes) {
            Ok(v) => v,
            Err(e) => return Err(VMLoadError::Unknown(e.to_string())),
        };
        let Object::Elf(elf) = obj else {
            return Err(VMLoadError::InvalidFileType(format!("{obj:#?}")));
        };

        if elf.is_lib {
            return Err(VMLoadError::NotABinary);
        }

        if !elf.little_endian {
            return Err(VMLoadError::NotLittleEndian);
        }

        if !elf.is_64 {
            return Err(VMLoadError::Not64BitArch);
        }

        // if PROG_BEGIN + bytes.len() >= PROG_LEN {
        //     return Err(VMLoadError::OutOfMemory);
        // }
        // self.mem[0..bytes.len()].copy_from_slice(bytes);

        info!("executable size: {} bytes", bytes.len());
        info!("program headers: {}", elf.program_headers.len());

        for p in &elf.program_headers {
            match p.p_type {
                0 => continue,      // PT_NULL
                1 => (),            // PT_LOAD
                0x60000000.. => (), // reserved
                _ => unimplemented!("p_type {:#x}", p.p_type),
            };

            let bytes_begin = p.p_offset as usize;
            let bytes_len = p.p_filesz as usize;

            let mem_begin = p.p_vaddr as usize;
            let mem_len = p.p_memsz as usize;

            let min_len = bytes_len.min(mem_len);
            self.mem[mem_begin..(mem_begin + min_len)]
                .copy_from_slice(&bytes[bytes_begin..(bytes_begin + min_len)]);

            if bytes_len < mem_len {
                let idx = mem_begin + min_len;
                let end_idx = idx + (mem_len - bytes_len);
                self.mem[idx..end_idx].fill(0);
            }

            info!(
                "loaded program header: v_addr={mem_begin:x}, v_len={mem_len:x}, offset={bytes_begin:x}, f_len={bytes_len:x}"
            );
        }

        // trace!("{:#?}", elf.program_headers);

        self.pc = elf.entry as usize;
        self.set_x(2, STACK_BEGIN);

        info!("start address: {:#x}", self.pc);
        info!("stack address: {:#x}", self.x(2));

        if cfg!(debug_assertions) {
            self.dbg_syms.clear();
            let mut sym_cnt = 0;

            for sym in elf.syms.iter() {
                if sym.st_value == 0 || matches!(sym.st_shndx as u32, sh::SHN_UNDEF) {
                    continue;
                }
                let sttype = sym.st_info & 0xf;
                if !matches!(sttype, 1..=3) {
                    continue;
                }

                let Some(name) = elf.strtab.get_at(sym.st_name) else {
                    continue;
                };
                if name.is_empty() {
                    continue;
                }

                let v = self.dbg_syms.entry(sym.st_value as usize).or_default();
                if !v.insert(name.to_owned()) {
                    panic!("duplicate '{name}' at {:x}", sym.st_value);
                }

                sym_cnt += 1;
            }

            info!("debug symbols: {sym_cnt}");
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<u8, VMRunError> {
        while !self.halt {
            self.step()?;
        }
        Ok(self.exit_code())
    }

    pub fn step(&mut self) -> Result<(), VMRunError> {
        if log_enabled!(log::Level::Trace) {
            let log = |vm: &VM| {
                let raw_inst = {
                    let Ok([b1, b2, b3, b4]) = vm.mem_range(vm.pc, 4) else {
                        return;
                    };
                    u32::from_le_bytes([*b1, *b2, *b3, *b4])
                };

                let format = RawFormat::parse(raw_inst).unwrap();

                let Ok(inst) = vm.fetch_inst(vm.pc) else {
                    return;
                };

                trace!(
                    "{}",
                    InstReport {
                        addr: vm.pc,
                        raw_inst,
                        inst_name: inst.name(),
                        format,
                    }
                );
            };
            log(self);
        }

        let inst = self.fetch_inst(self.pc)?;
        inst.run(self)?;
        self.pc = (self.pc + 4) % (PROG_LEN);
        Ok(())
    }

    pub fn fetch_inst(&self, addr: usize) -> Result<Inst, VMRunError> {
        if addr % 4 != 0 {
            return Err(VMRunError {
                err_addr: addr,
                kind: VMRunErrorKind::Alignment,
                info: "inst alignment",
            });
        }

        let inst = {
            let [b1, b2, b3, b4] = self.mem[addr..addr + 4] else {
                unreachable!();
            };
            u32::from_le_bytes([b1, b2, b3, b4])
        };

        let fmt = RawFormat::parse(inst).ok_or_else(|| VMRunError {
            err_addr: addr,
            kind: VMRunErrorKind::UnknownInst(inst),
            info: "fetch_inst (parse)",
        })?;

        let inst = Inst::decode(fmt).ok_or_else(|| VMRunError {
            err_addr: addr,
            kind: VMRunErrorKind::UnknownInst(inst),
            info: "fetch_inst (decode)",
        })?;

        Ok(inst)
    }

    pub fn mem(&self, addr: usize) -> Result<u8, VMRunError> {
        if addr < MEM_LEN {
            Ok(self.mem[addr])
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "mem",
            })
        }
    }

    pub fn mem_range(&self, addr: usize, len: usize) -> Result<&[u8], VMRunError> {
        if addr + len < MEM_LEN {
            Ok(&self.mem[addr..(addr + len)])
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(MEM_LEN),
                info: "mem_range",
            })
        }
    }

    pub fn mem_range_mut(&mut self, addr: usize, len: usize) -> Result<&mut [u8], VMRunError> {
        if addr + len < MEM_LEN {
            Ok(&mut self.mem[addr..(addr + len)])
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(MEM_LEN),
                info: "mem_range",
            })
        }
    }

    pub fn set_mem(&mut self, addr: usize, val: u8) -> Result<(), VMRunError> {
        if addr < MEM_LEN {
            self.mem[addr] = val;
            Ok(())
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "set_mem",
            })
        }
    }

    pub fn set_mem_range(&mut self, addr: usize, val: &[u8]) -> Result<(), VMRunError> {
        if addr < MEM_LEN {
            let mem = self.mem_range_mut(addr, val.len()).unwrap();
            mem.copy_from_slice(val);
            Ok(())
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "set_mem_range",
            })
        }
    }

    pub fn x(&self, i: u8) -> u64 {
        let i = i as usize;
        assert!(i < REG_COUNT, "invalid register");
        if i == 0 { 0 } else { self.regs[i] }
    }

    pub fn set_x(&mut self, i: u8, val: u64) {
        let i = i as usize;
        assert!(i < REG_COUNT, "invalid register");
        self.regs[i] = val;
    }

    pub fn jump(&mut self, addr: usize, dec_4: bool) -> Result<(), VMRunError> {
        if addr < PROG_LEN {
            if dec_4 {
                // -4 bytes to account for the instruction fetch
                self.pc = addr - 4;
            } else {
                self.pc = addr;
            }
            Ok(())
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "jump",
            })
        }
    }

    pub fn jump_pc_rel(&mut self, offset: isize, dec_4: bool) -> Result<(), VMRunError> {
        let addr = self.pc.wrapping_add_signed(offset) & !1;
        if addr < PROG_LEN {
            self.jump(addr, dec_4)
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "jump_rel",
            })
        }
    }

    pub(crate) fn raise(&mut self, ex: Exception) -> Result<(), VMRunError> {
        match ex {
            Exception::Ecall => self.syscall(),
        }
    }

    pub fn display_regs(&self, out: &mut impl Write) -> io::Result<()> {
        for i in 0..8 {
            for j in 0..4 {
                let idx = j * 4 + i;
                write!(out, "{}:\t{:16x} | ", x_name(idx), self.x(idx))?;
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum VMLoadError {
    Unknown(String),
    InvalidFileType(String),
    NotABinary,
    NotLittleEndian,
    Not64BitArch,
    OutOfMemory,
}

#[derive(Debug)]
pub struct VMRunError {
    pub err_addr: usize,
    pub kind: VMRunErrorKind,
    pub info: &'static str,
}
impl Display for VMRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.info.is_empty() {
            write!(f, "{:x}: {}", self.err_addr, self.kind)
        } else {
            write!(f, "{:x}: {}, {}", self.err_addr, self.kind, self.info)
        }
    }
}

#[derive(Debug)]
pub enum VMRunErrorKind {
    Alignment,
    UnknownInst(u32),
    InvalidAddress(usize),
    UnknownSyscall(i16),
    DivisionByZero,
    Other(String),
}
impl Display for VMRunErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alignment => "address misalignment".to_owned(),
                Self::UnknownInst(inst) => format!("unknown inst: {inst:08x}"),
                Self::InvalidAddress(addr) => format!("invalid address {addr:x}"),
                Self::UnknownSyscall(code) => format!("unknown syscall: {code}"),
                Self::DivisionByZero => "division by zero".to_owned(),
                Self::Other(s) => s.clone(),
            }
        )
    }
}

pub(crate) fn x_name(i: u8) -> &'static str {
    match i {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0",
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        _ => panic!("invalid register"),
    }
}
