use crate::{INST_LEN, Inst, decode_inst};

const REG_COUNT: usize = 32;

const MEM_LEN: usize = 64 * 1024;
const STACK_BEGIN: usize = MEM_LEN - 1;
const STACK_LEN: usize = 8 * 1024;

const PROG_LEN: usize = MEM_LEN - STACK_LEN;
const PROG_BEGIN: usize = 0;

#[derive(Debug)]
pub struct VM {
    regs: [u64; REG_COUNT],
    mem: Box<[u8]>,

    halt: bool,
    ip: usize,
    sp: usize,
}
impl VM {
    pub fn new() -> Self {
        Self {
            regs: Default::default(),
            mem: vec![0; MEM_LEN].into_boxed_slice(),

            halt: false,
            ip: PROG_BEGIN,
            sp: STACK_BEGIN,
        }
    }

    pub fn reset(&mut self) {
        self.halt = false;
        self.ip = PROG_BEGIN;
        self.sp = STACK_BEGIN;
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
            inst(self)?;
        }
        Ok(())
    }

    pub fn next_inst(&mut self) -> Inst {
        let mut bytes: [u8; INST_LEN] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = self.mem[(self.ip + i) % (PROG_LEN)];
        }
        self.ip = (self.ip + INST_LEN) % (PROG_LEN);

        decode_inst(bytes)
    }

    pub fn push(&mut self, bytes: &[u8]) -> Result<(), VMRunError> {
        for b in bytes.iter().rev() {
            if self.sp <= STACK_BEGIN - STACK_LEN {
                return Err(VMRunError::StackOverflow);
            }
            self.mem[self.sp] = *b;
            self.sp -= 1;
        }
        Ok(())
    }

    pub fn pop(&mut self, len: usize) -> Result<&[u8], VMRunError> {
        if self.sp + len > STACK_BEGIN {
            return Err(VMRunError::StackUnderflow);
        }
        let v = self.sp + 1;
        self.sp += len;
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
