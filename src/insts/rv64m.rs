// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError};

// $IMPL Mulw r
#[derive(Debug, Clone, Copy)]
pub struct Mulw;
impl Mulw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Mulw please!");
        Ok(())
    }
}

// $IMPL Divw r
#[derive(Debug, Clone, Copy)]
pub struct Divw;
impl Divw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Divw please!");
        Ok(())
    }
}

// $IMPL Divuw r
#[derive(Debug, Clone, Copy)]
pub struct Divuw;
impl Divuw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Divuw please!");
        Ok(())
    }
}

// $IMPL Remw r
#[derive(Debug, Clone, Copy)]
pub struct Remw;
impl Remw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Remw please!");
        Ok(())
    }
}

// $IMPL Remuw r
#[derive(Debug, Clone, Copy)]
pub struct Remuw;
impl Remuw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Remuw please!");
        Ok(())
    }
}
