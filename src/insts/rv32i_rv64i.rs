// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError};

// $IMPL Add r
#[derive(Debug, Clone, Copy)]
pub struct Add;
impl Add {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Add please!");
        Ok(())
    }
}

// $IMPL Addi i
#[derive(Debug, Clone, Copy)]
pub struct Addi;
impl Addi {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Addi please!");
        Ok(())
    }
}

// $IMPL Sb s
#[derive(Debug, Clone, Copy)]
pub struct Sb;
impl Sb {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Sb please!");
        Ok(())
    }
}

// $IMPL Beq b
#[derive(Debug, Clone, Copy)]
pub struct Beq;
impl Beq {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Beq please!");
        Ok(())
    }
}

// $IMPL Lui u
#[derive(Debug, Clone, Copy)]
pub struct Lui;
impl Lui {
    pub fn run(vm: &mut VM, rd: u8, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lui please!");
        Ok(())
    }
}

// $IMPL Jal j
#[derive(Debug, Clone, Copy)]
pub struct Jal;
impl Jal {
    pub fn run(vm: &mut VM, rd: u8, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Jal please!");
        Ok(())
    }
}

// $IMPL Ecall o
#[derive(Debug, Clone, Copy)]
pub struct Ecall;
impl Ecall {
    pub fn run(vm: &mut VM) -> Result<(), VMRunError> {
        todo!("implement Ecall please!");
        Ok(())
    }
}
