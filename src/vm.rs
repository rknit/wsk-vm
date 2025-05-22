use std::io::{self, Write};

use crate::{INST_LEN, Inst, decode_inst};

const REG_COUNT: usize = 32;

const MEM_LEN: usize = 64 * 1024;
const STACK_BEGIN: u64 = (MEM_LEN as u64) - 1;
const STACK_LEN: u64 = 8 * 1024;

const PROG_LEN: usize = MEM_LEN - (STACK_LEN as usize);
const PROG_BEGIN: usize = 0;

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

    pub fn load_program_from_bytes(&mut self, bytes: &[u8]) -> Result<(), VMLoadError> {
        if PROG_BEGIN + bytes.len() >= PROG_LEN {
            return Err(VMLoadError::OutOfMemory);
        }

        self.mem[0..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMRunError> {
        while !self.halt {
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

    pub fn push(&mut self, bytes: &[u8]) -> Result<(), VMRunError> {
        for b in bytes.iter().rev() {
            if self.x(2) <= STACK_BEGIN - STACK_LEN {
                return Err(VMRunError::StackOverflow);
            }
            self.mem[self.x(2) as usize] = *b;
            self.set_x(2, self.x(2) - 1);
        }
        Ok(())
    }

    pub fn pop(&mut self, len: usize) -> Result<&[u8], VMRunError> {
        if self.x(2) + (len as u64) > STACK_BEGIN {
            return Err(VMRunError::StackUnderflow);
        }
        let v = (self.x(2) + 1) as usize;
        self.set_x(2, self.x(2) + len as u64);
        Ok(&self.mem[v..v + len])
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
    OutOfMemory,
}

#[derive(Debug)]
pub enum VMRunError {
    InvalidAddress,
    StackOverflow,
    StackUnderflow,
}
