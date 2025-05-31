// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError, VMRunErrorKind};

// $IMPL Mulw r
#[derive(Debug, Clone, Copy)]
pub struct Mulw;
impl Mulw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let result = r1.wrapping_mul(r2) as i32 as i64;
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Divw r
#[derive(Debug, Clone, Copy)]
pub struct Divw;
impl Divw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = (r1 as i32).wrapping_div(r2 as i32) as i64;
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Divuw r
#[derive(Debug, Clone, Copy)]
pub struct Divuw;
impl Divuw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = (r1 as u32).wrapping_div(r2 as u32) as u64;
        vm.set_x(rd, result);
        Ok(())
    }
}

// $IMPL Remw r
#[derive(Debug, Clone, Copy)]
pub struct Remw;
impl Remw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = (r1 as i32).wrapping_rem(r2 as i32) as i64;
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Remuw r
#[derive(Debug, Clone, Copy)]
pub struct Remuw;
impl Remuw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = (r1 as u32).wrapping_rem(r2 as u32) as u64;
        vm.set_x(rd, result);
        Ok(())
    }
}
