use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use log::{info, log_enabled, trace};

use crate::{Byte, Exception, Inst, InstReport, SArch, SHalf, UArch, UHSize, Word};

const REG_COUNT: usize = 32;

const MEM_LEN: UArch = 64 * MEGABYTE;

const STACK_BEGIN: UArch = MEM_LEN - (8 * MEGABYTE);
const STACK_LEN: UArch = 8 * MEGABYTE;

const PROG_LEN: UArch = MEM_LEN - STACK_LEN;
const PROG_BEGIN: UArch = 0;

const MEGABYTE: UArch = 1024 * 1024;

pub struct VM {
    regs: [UArch; REG_COUNT],
    mem: Box<[Byte]>,
    pub pc: UArch,

    pub(crate) halt: bool,
    pub(crate) exit_code: Byte,

    dbg_syms: HashMap<UArch, HashSet<String>>,
}
impl VM {
    pub fn new() -> Self {
        Self {
            regs: Default::default(),
            mem: vec![0; MEM_LEN as UHSize].into_boxed_slice(),
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

    pub fn exit_code(&self) -> Byte {
        self.exit_code
    }

    pub fn load_executable_bytes(&mut self, bytes: &[Byte]) -> Result<(), VMLoadError> {
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

            let bytes_begin = p.p_offset as UHSize;
            let bytes_len = p.p_filesz as UHSize;

            let mem_begin = p.p_vaddr as UHSize;
            let mem_len = p.p_memsz as UHSize;

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

        self.pc = elf.entry as UArch;
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

                let v = self.dbg_syms.entry(sym.st_value as UArch).or_default();
                if !v.insert(name.to_owned()) {
                    panic!("duplicate '{name}' at {:x}", sym.st_value);
                }

                sym_cnt += 1;
            }

            info!("debug symbols: {sym_cnt}");
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<Byte, VMRunError> {
        while !self.halt {
            self.step()?;
        }
        Ok(self.exit_code())
    }

    pub fn step(&mut self) -> Result<(), VMRunError> {
        let inst = self.fetch_inst(self.pc)?;
        if log_enabled!(log::Level::Trace) {
            trace!(
                "{}",
                InstReport {
                    addr: self.pc,
                    inst,
                }
            );
        }

        inst.run(self)?;
        self.pc = (self.pc + 4) % (PROG_LEN);
        Ok(())
    }

    pub fn fetch_inst(&self, addr: UArch) -> Result<Inst, VMRunError> {
        if addr % 4 != 0 {
            return Err(VMRunError {
                err_addr: addr,
                kind: VMRunErrorKind::Alignment,
                info: "inst alignment",
            });
        }

        let inst32 = {
            let [b1, b2, b3, b4] = self.mem[addr as UHSize..addr as UHSize + 4] else {
                unreachable!();
            };
            u32::from_le_bytes([b1, b2, b3, b4])
        };

        let inst = Inst::decode(inst32).ok_or_else(|| VMRunError {
            err_addr: addr,
            kind: VMRunErrorKind::UnknownInst(inst32),
            info: "fetch_inst (decode)",
        })?;

        Ok(inst)
    }

    pub fn mem(&self, addr: UArch) -> Result<Byte, VMRunError> {
        if addr < MEM_LEN {
            Ok(self.mem[addr as UHSize])
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "mem",
            })
        }
    }

    pub fn mem_range(&self, addr: UArch, len: UArch) -> Result<&[Byte], VMRunError> {
        if addr + len < MEM_LEN {
            Ok(&self.mem[addr as UHSize..(addr + len) as UHSize])
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(MEM_LEN),
                info: "mem_range",
            })
        }
    }

    pub fn mem_range_mut(&mut self, addr: UArch, len: UArch) -> Result<&mut [Byte], VMRunError> {
        if addr + len < MEM_LEN {
            Ok(&mut self.mem[addr as UHSize..(addr + len) as UHSize])
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(MEM_LEN),
                info: "mem_range",
            })
        }
    }

    pub fn set_mem(&mut self, addr: UArch, value: Byte) -> Result<(), VMRunError> {
        if addr < MEM_LEN {
            self.mem[addr as UHSize] = value;
            Ok(())
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "set_mem",
            })
        }
    }

    pub fn set_mem_range(&mut self, addr: UArch, values: &[Byte]) -> Result<(), VMRunError> {
        if addr < MEM_LEN {
            let mem = self.mem_range_mut(addr, values.len() as UArch).unwrap();
            mem.copy_from_slice(values);
            Ok(())
        } else {
            Err(VMRunError {
                err_addr: self.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "set_mem_range",
            })
        }
    }

    pub fn x(&self, i: Byte) -> UArch {
        let i = i as usize;
        assert!(i < REG_COUNT, "invalid register");
        if i == 0 { 0 } else { self.regs[i] }
    }

    pub fn set_x(&mut self, i: Byte, val: UArch) {
        let i = i as usize;
        assert!(i < REG_COUNT, "invalid register");
        self.regs[i] = val;
    }

    pub fn jump(&mut self, addr: UArch, dec_4: bool) -> Result<(), VMRunError> {
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

    pub fn jump_pc_rel(&mut self, offset: SArch, dec_4: bool) -> Result<(), VMRunError> {
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

    pub fn raise(&mut self, ex: Exception) -> Result<(), VMRunError> {
        match ex {
            Exception::Ecall => self.syscall(),
        }
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
    pub err_addr: UArch,
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
    UnknownInst(Word),
    InvalidAddress(UArch),
    UnknownSyscall(SHalf),
    DivisionByZero,
    Other(String),
}
impl Display for VMRunErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alignment => write!(f, "address misalignment"),
            Self::UnknownInst(inst) => write!(f, "unknown inst: {inst:08x}"),
            Self::InvalidAddress(addr) => write!(f, "invalid address {addr:x}"),
            Self::UnknownSyscall(code) => write!(f, "unknown syscall: {code}"),
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}
