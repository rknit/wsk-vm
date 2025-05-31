// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError};

// $IMPL Addw r
#[derive(Debug, Clone, Copy)]
pub struct Addw;
impl Addw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Addw please!");
        Ok(())
    }
}

// $IMPL Subw r
#[derive(Debug, Clone, Copy)]
pub struct Subw;
impl Subw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Subw please!");
        Ok(())
    }
}

// $IMPL Sllw r
#[derive(Debug, Clone, Copy)]
pub struct Sllw;
impl Sllw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Sllw please!");
        Ok(())
    }
}

// $IMPL Srlw r
#[derive(Debug, Clone, Copy)]
pub struct Srlw;
impl Srlw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Srlw please!");
        Ok(())
    }
}

// $IMPL Sraw r
#[derive(Debug, Clone, Copy)]
pub struct Sraw;
impl Sraw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Sraw please!");
        Ok(())
    }
}

// $IMPL Addiw i
#[derive(Debug, Clone, Copy)]
pub struct Addiw;
impl Addiw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Addiw please!");
        Ok(())
    }
}

// $IMPL Slliw i
#[derive(Debug, Clone, Copy)]
pub struct Slliw;
impl Slliw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Slliw please!");
        Ok(())
    }
}

// $IMPL Srliw i
#[derive(Debug, Clone, Copy)]
pub struct Srliw;
impl Srliw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Srliw please!");
        Ok(())
    }
}

// $IMPL Sraiw i
#[derive(Debug, Clone, Copy)]
pub struct Sraiw;
impl Sraiw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Sraiw please!");
        Ok(())
    }
}

// $IMPL Lwu i
#[derive(Debug, Clone, Copy)]
pub struct Lwu;
impl Lwu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lwu please!");
        Ok(())
    }
}

// $IMPL Ld i
#[derive(Debug, Clone, Copy)]
pub struct Ld;
impl Ld {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Ld please!");
        Ok(())
    }
}

// $IMPL Sd s
#[derive(Debug, Clone, Copy)]
pub struct Sd;
impl Sd {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Sd please!");
        Ok(())
    }
}
