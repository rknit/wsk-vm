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
        let result = r1.wrapping_add(r2);
        let result = (result & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Subw r
#[derive(Debug, Clone, Copy)]
pub struct Subw;
impl Subw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let result = r1.wrapping_sub(r2);
        let result = (result & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Sllw r
#[derive(Debug, Clone, Copy)]
pub struct Sllw;
impl Sllw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let shift = (r2 & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let result = ((r1 << shift) & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Srlw r
#[derive(Debug, Clone, Copy)]
pub struct Srlw;
impl Srlw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let shift = (r2 & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let val = r1 as u32; // Convert to 32 bits
        let result = (val >> shift) as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Sraw r
#[derive(Debug, Clone, Copy)]
pub struct Sraw;
impl Sraw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        let shift = (r2 & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let val = r1 as i32; // Convert to 32 bits
        let result = (val >> shift) as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Addiw i
#[derive(Debug, Clone, Copy)]
pub struct Addiw;
impl Addiw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let result = r1.wrapping_add_signed(imm);
        let result = (result & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Slliw i
#[derive(Debug, Clone, Copy)]
pub struct Slliw;
impl Slliw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let shift = (imm & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let result = ((r1 << shift) & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Srliw i
#[derive(Debug, Clone, Copy)]
pub struct Srliw;
impl Srliw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let shift = (imm & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let val = r1 as u32; // Convert to 32 bits
        let result = (val >> shift) as i32 as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Sraiw i
#[derive(Debug, Clone, Copy)]
pub struct Sraiw;
impl Sraiw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let shift = (imm & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let val = r1 as i32; // Convert to 32 bits
        let result = (val >> shift) as i64; // Sign-extend to 64 bits
        vm.set_x(rd, result as u64);
        Ok(())
    }
}

// $IMPL Lwu i
#[derive(Debug, Clone, Copy)]
pub struct Lwu;
impl Lwu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let address = r1.wrapping_add_signed(imm);
        let data = vm.mem_range(address as usize, 4)?;
        let val = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        vm.set_x(rd, val as u64);
        Ok(())
    }
}

// $IMPL Ld i
#[derive(Debug, Clone, Copy)]
pub struct Ld;
impl Ld {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let address = r1.wrapping_add_signed(imm);
        let data = vm.mem_range(address as usize, 8)?;
        let val = u64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        vm.set_x(rd, val);
        Ok(())
    }
}

// $IMPL Sd s
#[derive(Debug, Clone, Copy)]
pub struct Sd;
impl Sd {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        let address = r1.wrapping_add_signed(offset);
        let data = r2.to_le_bytes();
        vm.set_mem_range(address as usize, &data)?;
        Ok(())
    }
}
