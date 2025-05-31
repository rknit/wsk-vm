// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError, exception::Exception, ext};

// $IMPL Add r
#[derive(Debug, Clone, Copy)]
pub struct Add;
impl Add {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1.wrapping_add(r2));
        Ok(())
    }
}

// $IMPL Sub r
#[derive(Debug, Clone, Copy)]
pub struct Sub;
impl Sub {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1.wrapping_sub(r2));
        Ok(())
    }
}

// $IMPL Sll r
#[derive(Debug, Clone, Copy)]
pub struct Sll;
impl Sll {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 << ext!(r2, u64; 4;0));
        Ok(())
    }
}

// $IMPL Slt r
#[derive(Debug, Clone, Copy)]
pub struct Slt;
impl Slt {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if (r1 as i64) < (r2 as i64) {
            vm.set_x(rd, 1);
        } else {
            vm.set_x(rd, 0);
        }
        Ok(())
    }
}

// $IMPL Sltu r
#[derive(Debug, Clone, Copy)]
pub struct Sltu;
impl Sltu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        if r1 < r2 {
            vm.set_x(rd, 1);
        } else {
            vm.set_x(rd, 0);
        }
        Ok(())
    }
}

// $IMPL Xor r
#[derive(Debug, Clone, Copy)]
pub struct Xor;
impl Xor {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 ^ r2);
        Ok(())
    }
}

// $IMPL Srl r
#[derive(Debug, Clone, Copy)]
pub struct Srl;
impl Srl {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 >> ext!(r2, u64; 4;0));
        Ok(())
    }
}

// $IMPL Sra r
#[derive(Debug, Clone, Copy)]
pub struct Sra;
impl Sra {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, ((r1 as i64) >> ext!(r2, u64; 4;0)) as u64);
        Ok(())
    }
}

// $IMPL Or r
#[derive(Debug, Clone, Copy)]
pub struct Or;
impl Or {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 | r2);
        Ok(())
    }
}

// $IMPL And r
#[derive(Debug, Clone, Copy)]
pub struct And;
impl And {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 & r2);
        Ok(())
    }
}

// $IMPL Addi i
#[derive(Debug, Clone, Copy)]
pub struct Addi;
impl Addi {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1.wrapping_add_signed(imm));
        Ok(())
    }
}

// $IMPL Slti i
#[derive(Debug, Clone, Copy)]
pub struct Slti;
impl Slti {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        if (r1 as i64) < imm {
            vm.set_x(rd, 1);
        } else {
            vm.set_x(rd, 0);
        }
        Ok(())
    }
}

// $IMPL Sltiu i
#[derive(Debug, Clone, Copy)]
pub struct Sltiu;
impl Sltiu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        if r1 < (imm as u64) {
            vm.set_x(rd, 1);
        } else {
            vm.set_x(rd, 0);
        }
        Ok(())
    }
}

// $IMPL Xori i
#[derive(Debug, Clone, Copy)]
pub struct Xori;
impl Xori {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 ^ (imm as u64));
        Ok(())
    }
}

// $IMPL Ori i
#[derive(Debug, Clone, Copy)]
pub struct Ori;
impl Ori {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 | (imm as u64));
        Ok(())
    }
}

// $IMPL Andi i
#[derive(Debug, Clone, Copy)]
pub struct Andi;
impl Andi {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        vm.set_x(rd, r1 & (imm as u64));
        Ok(())
    }
}

// $IMPL Slli i
#[derive(Debug, Clone, Copy)]
pub struct Slli;
impl Slli {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, r1 << shamt);
        Ok(())
    }
}

// $IMPL Srli i
#[derive(Debug, Clone, Copy)]
pub struct Srli;
impl Srli {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, r1 >> shamt);
        Ok(())
    }
}

// $IMPL Srai i
#[derive(Debug, Clone, Copy)]
pub struct Srai;
impl Srai {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, ((r1 as i64) >> shamt) as u64);
        Ok(())
    }
}

// $IMPL Lb i
#[derive(Debug, Clone, Copy)]
pub struct Lb;
impl Lb {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(imm);
        let data = vm.mem(addr as usize)?;
        let data = data as i8 as i64; // Sign-extend the byte
        vm.set_x(rd, data as u64);
        Ok(())
    }
}

// $IMPL Lh i
#[derive(Debug, Clone, Copy)]
pub struct Lh;
impl Lh {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(imm);
        let data = vm.mem_range(addr as usize, 2)?;
        let data = i16::from_le_bytes([data[0], data[1]]) as i64; // Sign-extend the halfword
        vm.set_x(rd, data as u64);
        Ok(())
    }
}

// $IMPL Lw i
#[derive(Debug, Clone, Copy)]
pub struct Lw;
impl Lw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(imm);
        let data = vm.mem_range(addr as usize, 4)?;
        let data = i32::from_le_bytes([data[0], data[1], data[2], data[3]]) as i64; // Sign-extend the word
        vm.set_x(rd, data as u64);
        Ok(())
    }
}

// $IMPL Lbu i
#[derive(Debug, Clone, Copy)]
pub struct Lbu;
impl Lbu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(imm);
        let data = vm.mem(addr as usize)?;
        let data = data as u64; // Zero-extend the byte
        vm.set_x(rd, data);
        Ok(())
    }
}

// $IMPL Lhu i
#[derive(Debug, Clone, Copy)]
pub struct Lhu;
impl Lhu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(imm);
        let data = vm.mem_range(addr as usize, 2)?;
        let data = u16::from_le_bytes([data[0], data[1]]) as u64; // Zero-extend the halfword
        vm.set_x(rd, data);
        Ok(())
    }
}

// $IMPL Jalr i
#[derive(Debug, Clone, Copy)]
pub struct Jalr;
impl Jalr {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        let return_address = vm.pc + 4;
        vm.jump(r1.wrapping_add_signed(imm) as usize, true)?; // Jump to the target address
        vm.set_x(rd, return_address as u64); // Save the return address
        Ok(())
    }
}

// $IMPL Sb s
#[derive(Debug, Clone, Copy)]
pub struct Sb;
impl Sb {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(offset);
        let data = (r2 & 0xFF) as u8; // Store only the least significant byte
        vm.set_mem(addr as usize, data)?;
        Ok(())
    }
}

// $IMPL Sh s
#[derive(Debug, Clone, Copy)]
pub struct Sh;
impl Sh {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(offset);
        let data = (r2 & 0xFFFF) as u16; // Store only the least significant halfword
        let bytes = data.to_le_bytes();
        vm.set_mem_range(addr as usize, &bytes)?;
        Ok(())
    }
}

// $IMPL Sw s
#[derive(Debug, Clone, Copy)]
pub struct Sw;
impl Sw {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        let addr = r1.wrapping_add_signed(offset);
        let data = (r2 & 0xFFFFFFFF) as u32; // Store only the least significant word
        let bytes = data.to_le_bytes();
        vm.set_mem_range(addr as usize, &bytes)?;
        Ok(())
    }
}

// $IMPL Beq b
#[derive(Debug, Clone, Copy)]
pub struct Beq;
impl Beq {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        if r1 == r2 {
            vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        }
        Ok(())
    }
}

// $IMPL Bne b
#[derive(Debug, Clone, Copy)]
pub struct Bne;
impl Bne {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        if r1 != r2 {
            vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        }
        Ok(())
    }
}

// $IMPL Blt b
#[derive(Debug, Clone, Copy)]
pub struct Blt;
impl Blt {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        if (r1 as i64) < (r2 as i64) {
            vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        }
        Ok(())
    }
}

// $IMPL Bge b
#[derive(Debug, Clone, Copy)]
pub struct Bge;
impl Bge {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        if (r1 as i64) >= (r2 as i64) {
            vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        }
        Ok(())
    }
}

// $IMPL Bltu b
#[derive(Debug, Clone, Copy)]
pub struct Bltu;
impl Bltu {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        if r1 < r2 {
            vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        }
        Ok(())
    }
}

// $IMPL Bgeu b
#[derive(Debug, Clone, Copy)]
pub struct Bgeu;
impl Bgeu {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        if r1 >= r2 {
            vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        }
        Ok(())
    }
}

// $IMPL Lui u
#[derive(Debug, Clone, Copy)]
pub struct Lui;
impl Lui {
    pub fn run(vm: &mut VM, rd: u8, imm: i64) -> Result<(), VMRunError> {
        // imm has already been shifted left by 12 bits and sign-extended to 64 bits
        vm.set_x(rd, imm as u64);
        Ok(())
    }
}

// $IMPL Auipc u
#[derive(Debug, Clone, Copy)]
pub struct Auipc;
impl Auipc {
    pub fn run(vm: &mut VM, rd: u8, imm: i64) -> Result<(), VMRunError> {
        // imm has already been shifted left by 12 bits and sign-extended to 64 bits
        vm.set_x(rd, vm.pc.wrapping_add_signed(imm as isize) as u64);
        Ok(())
    }
}

// $IMPL Jal j
#[derive(Debug, Clone, Copy)]
pub struct Jal;
impl Jal {
    pub fn run(vm: &mut VM, rd: u8, offset: i64) -> Result<(), VMRunError> {
        let return_address = vm.pc + 4;
        vm.set_x(rd, return_address as u64); // Save the return address
        vm.jump_pc_rel(offset as isize, true)?; // Jump to the target address
        Ok(())
    }
}

// $IMPL Ecall o
#[derive(Debug, Clone, Copy)]
pub struct Ecall;
impl Ecall {
    pub fn run(vm: &mut VM) -> Result<(), VMRunError> {
        vm.raise(Exception::Ecall)
    }
}
