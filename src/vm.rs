use std::io::{self, Write};

use log::trace;

use crate::{INST_LEN, Inst, decode_inst};

const REG_COUNT: usize = 32;

const MEM_LEN: usize = 64 * MEGABYTE;
const STACK_BEGIN: u64 = (MEM_LEN as u64) - 1;
const STACK_LEN: u64 = 8 * MEGABYTE as u64;

const PROG_LEN: usize = MEM_LEN - (STACK_LEN as usize);
const PROG_BEGIN: usize = 0;

const MEGABYTE: usize = 1024 * 1024;

pub struct VM {
    regs: [u64; REG_COUNT],
    mem: Box<[u8]>,

    pub(crate) halt: bool,
    pub(crate) pc: usize,
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

            halt: false,
            pc: PROG_BEGIN,
        }
    }

    pub fn reset(&mut self) {
        self.halt = false;
        self.pc = PROG_BEGIN;
        self.set_x(2, STACK_BEGIN);
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
        }

        // trace!("{:#?}", elf.program_headers);

        self.pc = elf.entry as usize;
        self.set_x(2, STACK_BEGIN);

        trace!("executable size: {} bytes", bytes.len());
        trace!("program headers: {}", elf.program_headers.len());
        trace!("start address: {:#x}", self.pc);
        trace!("stack address: {:#x}", self.x(2));

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMRunError> {
        while !self.halt {
            trace!("pc {:#x}", self.pc);
            let inst = self.next_inst();
            inst.run_inst(self)?;
        }
        Ok(())
    }

    pub(crate) fn next_inst(&mut self) -> Inst {
        let mut bytes: [u8; INST_LEN] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = self.mem[(self.pc + i) % (PROG_LEN)];
        }
        self.pc = (self.pc + INST_LEN) % (PROG_LEN);

        decode_inst(bytes)
    }

    pub fn mem(&self, addr: usize) -> Result<u8, VMRunError> {
        if addr < MEM_LEN {
            Ok(self.mem[addr])
        } else {
            Err(VMRunError::InvalidAddress)
        }
    }

    pub fn set_mem(&mut self, addr: usize, val: u8) -> Result<(), VMRunError> {
        if addr < MEM_LEN {
            self.mem[addr] = val;
            Ok(())
        } else {
            Err(VMRunError::InvalidAddress)
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
pub enum VMRunError {
    InvalidAddress,
    StackOverflow,
    StackUnderflow,
}

const ELF_MAGIC: &[u8; 16] = b"\x7f\x45\x4c\x46\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00";
