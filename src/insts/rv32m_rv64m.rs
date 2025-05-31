// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError};

// $IMPL Mul r
#[derive(Debug, Clone, Copy)]
pub struct Mul;
impl Mul {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Mul please!");
        Ok(())
    }
}

// $IMPL Mulh r
#[derive(Debug, Clone, Copy)]
pub struct Mulh;
impl Mulh {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Mulh please!");
        Ok(())
    }
}

// $IMPL Mulhsu r
#[derive(Debug, Clone, Copy)]
pub struct Mulhsu;
impl Mulhsu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Mulhsu please!");
        Ok(())
    }
}

// $IMPL Mulhu r
#[derive(Debug, Clone, Copy)]
pub struct Mulhu;
impl Mulhu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Mulhu please!");
        Ok(())
    }
}

// $IMPL Div r
#[derive(Debug, Clone, Copy)]
pub struct Div;
impl Div {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Div please!");
        Ok(())
    }
}

// $IMPL Divu r
#[derive(Debug, Clone, Copy)]
pub struct Divu;
impl Divu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Divu please!");
        Ok(())
    }
}

// $IMPL Rem r
#[derive(Debug, Clone, Copy)]
pub struct Rem;
impl Rem {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Rem please!");
        Ok(())
    }
}

// $IMPL Remu r
#[derive(Debug, Clone, Copy)]
pub struct Remu;
impl Remu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Remu please!");
        Ok(())
    }
}
