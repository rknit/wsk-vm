use std::{
    fmt::Display,
    io::{self, Write},
};

use log::trace;

use crate::{Exception, INST_LEN, Inst, decode_inst};

const REG_COUNT: usize = 32;

const MEM_LEN: usize = 64 * MEGABYTE;

const STACK_BEGIN: u64 = (MEM_LEN as u64) - 0x8000;
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

    pub(crate) rep: Report,
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

            rep: Default::default(),
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

        trace!("executable size: {} bytes", bytes.len());
        trace!("program headers: {}", elf.program_headers.len());

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

            trace!(
                "loaded program header: v_addr={mem_begin:x}, v_len={mem_len:x}, offset={bytes_begin:x}, f_len={bytes_len:x}"
            );
        }

        // trace!("{:#?}", elf.program_headers);

        self.pc = elf.entry as usize;
        self.set_x(2, STACK_BEGIN);

        trace!("start address: {:#x}", self.pc);
        trace!("stack address: {:#x}", self.x(2));

        Ok(())
    }

    pub fn run(&mut self) -> Result<u8, VMRunError> {
        while !self.halt {
            self.step()?;
        }
        Ok(self.exit_code())
    }

    pub fn step(&mut self) -> Result<(), VMRunError> {
        self.rep = Default::default();
        self.rep.pc = self.pc;

        let inst = self.fetch_inst()?;
        inst.run_inst(self)?;

        self.pc = (self.pc + INST_LEN) % (PROG_LEN);

        trace!("{}", self.report());
        Ok(())
    }

    pub(crate) fn fetch_inst(&mut self) -> Result<Inst, VMRunError> {
        let mut bytes: [u8; INST_LEN] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = self.mem[(self.pc + i) % (PROG_LEN)];
        }

        let inst = u32::from_le_bytes(bytes);
        self.rep.raw_inst = inst;
        decode_inst(inst).map_err(|e| VMRunError {
            pc: self.rep.pc,
            kind: e,
            info: "decode",
        })
    }

    pub fn mem(&self, addr: usize) -> Result<u8, VMRunError> {
        if addr < MEM_LEN {
            Ok(self.mem[addr])
        } else {
            Err(VMRunError {
                pc: self.rep.pc,
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
                pc: self.rep.pc,
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
                pc: self.rep.pc,
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
                pc: self.rep.pc,
                kind: VMRunErrorKind::InvalidAddress(addr),
                info: "set_mem",
            })
        }
    }

    pub fn x(&self, i: usize) -> u64 {
        assert!(i < REG_COUNT, "invalid register");
        if i == 0 { 0 } else { self.regs[i] }
    }

    pub fn set_x(&mut self, i: usize, val: u64) {
        assert!(i < REG_COUNT, "invalid register");
        self.regs[i] = val;
    }

    pub fn display(&self, out: &mut impl Write) -> io::Result<()> {
        writeln!(out, "# state")?;
        writeln!(out, "ip: {:x}", self.pc)?;
        writeln!(out, "sp: {:x}", self.x(2))?;
        writeln!(out, "# registers")?;
        for j in 0..8 {
            for i in 0..4 {
                let idx = j * 4 + i;
                write!(out, "x{idx}: {:x}, ", self.x(idx))?;
            }
            writeln!(out)?;
        }
        Ok(())
    }

    pub fn report(&self) -> String {
        format!(
            "{:x}:\t{:08x}\t{}",
            self.rep.pc, self.rep.raw_inst, self.rep.inst
        )
    }

    pub(crate) fn raise(&mut self, ex: Exception) -> Result<(), VMRunError> {
        match ex {
            Exception::EnvCall => self.syscall(),
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
    pub pc: usize,
    pub kind: VMRunErrorKind,
    pub info: &'static str,
}
impl Display for VMRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.info.is_empty() {
            write!(f, "{:x}: {}", self.pc, self.kind)
        } else {
            write!(f, "{:x}: {}, {}", self.pc, self.kind, self.info)
        }
    }
}

#[derive(Debug)]
pub enum VMRunErrorKind {
    UnknownInst(u32),
    InvalidAddress(usize),
    UnknownSyscall(i16),
    Other(String),
}
impl Display for VMRunErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownInst(inst) => format!("unknown inst: {inst:08x}"),
                Self::InvalidAddress(addr) => format!("invalid address {addr:x}"),
                Self::UnknownSyscall(code) => format!("unknown syscall: {code}"),
                Self::Other(s) => s.clone(),
            }
        )
    }
}

#[derive(Default)]
pub(crate) struct Report {
    pub pc: usize,
    pub raw_inst: u32,
    pub inst: String,
    pub is_syscall: bool,
    pub syscall_name: &'static str,
}
impl Report {
    pub(crate) fn misc(&mut self, inst: &'static str) {
        if self.is_syscall {
            self.inst = format!("{inst}\t# syscall = {}", self.syscall_name)
        } else {
            self.inst = inst.to_owned();
        }
    }

    pub(crate) fn u(&mut self, inst: &'static str, rd: usize, imm: i64) {
        self.inst = if imm < 0 {
            format!("{inst}\t{}, {}", Self::x_name(rd), imm >> 12)
        } else {
            format!("{inst}\t{}, {:#x}", Self::x_name(rd), imm >> 12)
        };
    }

    pub(crate) fn i(&mut self, inst: &'static str, rd: usize, rs1: usize, imm: i64) {
        self.inst = if matches!(inst, "jalr") {
            if imm < 0 {
                format!("{inst}\t{}, {imm}({})", Self::x_name(rd), Self::x_name(rs1))
            } else {
                format!(
                    "{inst}\t{}, {imm:#x}({})",
                    Self::x_name(rd),
                    Self::x_name(rs1)
                )
            }
        } else if imm < 0 {
            format!("{inst}\t{}, {}, {imm}", Self::x_name(rd), Self::x_name(rs1))
        } else {
            format!(
                "{inst}\t{}, {}, {imm:#x}",
                Self::x_name(rd),
                Self::x_name(rs1)
            )
        };
    }

    pub(crate) fn l(&mut self, inst: &'static str, rd: usize, rs1: usize, imm: i64) {
        self.inst = if imm < 0 {
            format!("{inst}\t{}, {imm}({})", Self::x_name(rd), Self::x_name(rs1))
        } else {
            format!(
                "{inst}\t{}, {imm:#x}({})",
                Self::x_name(rd),
                Self::x_name(rs1)
            )
        };
    }

    pub(crate) fn r(&mut self, inst: &'static str, rd: usize, rs1: usize, rs2: usize) {
        self.inst = format!(
            "{inst}\t{}, {}, {}",
            Self::x_name(rd),
            Self::x_name(rs1),
            Self::x_name(rs2)
        );
    }

    pub(crate) fn b(&mut self, inst: &'static str, rs1: usize, rs2: usize, imm: i64) {
        self.inst = if imm < 0 {
            format!(
                "{inst}\t{}, {}, {}",
                Self::x_name(rs1),
                Self::x_name(rs2),
                imm >> 1
            )
        } else {
            format!(
                "{inst}\t{}, {}, {:#x}",
                Self::x_name(rs1),
                Self::x_name(rs2),
                imm >> 1
            )
        };
    }

    pub(crate) fn j(&mut self, inst: &'static str, rd: usize, imm: i64) {
        self.inst = format!(
            "{inst}\t{:#x}\t# rd = {}",
            self.pc.wrapping_add_signed(imm as isize),
            Self::x_name(rd)
        );
    }

    pub(crate) fn s(&mut self, inst: &'static str, rs1: usize, rs2: usize, imm: i64) {
        self.inst = if imm < 0 {
            format!(
                "{inst}\t{}, {imm}({})",
                Self::x_name(rs2),
                Self::x_name(rs1)
            )
        } else {
            format!(
                "{inst}\t{}, {imm:#x}({})",
                Self::x_name(rs2),
                Self::x_name(rs1)
            )
        };
    }

    fn x_name(i: usize) -> &'static str {
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
}
