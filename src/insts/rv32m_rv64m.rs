// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError, VMRunErrorKind};

// $IMPL Mul r
#[derive(Debug, Clone, Copy)]
pub struct Mul;
impl Mul {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let result = (r1 as i64).wrapping_mul(r2 as i64);
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Mulh r
#[derive(Debug, Clone, Copy)]
pub struct Mulh;
impl Mulh {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let r1 = r1 as i64 as i128;
        let r2 = r2 as i64 as i128;
        let result = ((r1 * r2) >> 64) as i64;
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Mulhsu r
#[derive(Debug, Clone, Copy)]
pub struct Mulhsu;
impl Mulhsu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let r1 = r1 as i64 as i128;
        let r2 = r2 as i128; // r2 is treated as unsigned, so we don't sign-extend it
        let result = ((r1 * r2) >> 64) as i64;
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Mulhu r
#[derive(Debug, Clone, Copy)]
pub struct Mulhu;
impl Mulhu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let r1 = r1 as u128;
        let r2 = r2 as u128;
        let result = ((r1 * r2) >> 64) as u64;
        vm.set_x(rd, result);
        Ok(())
    }
}

// $IMPL Div r
#[derive(Debug, Clone, Copy)]
pub struct Div;
impl Div {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = (r1 as i64).wrapping_div(r2 as i64);
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Divu r
#[derive(Debug, Clone, Copy)]
pub struct Divu;
impl Divu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = r1.wrapping_div(r2);
        vm.set_x(rd, result);
        Ok(())
    }
}

// $IMPL Rem r
#[derive(Debug, Clone, Copy)]
pub struct Rem;
impl Rem {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = (r1 as i64).wrapping_rem(r2 as i64);
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Remu r
#[derive(Debug, Clone, Copy)]
pub struct Remu;
impl Remu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r2 == 0 {
            return Err(VMRunError {
                err_addr: vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = r1.wrapping_rem(r2);
        vm.set_x(rd, result);
        Ok(())
    }
}
