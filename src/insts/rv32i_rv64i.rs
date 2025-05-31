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

// $IMPL Sub r
#[derive(Debug, Clone, Copy)]
pub struct Sub;
impl Sub {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Sub please!");
        Ok(())
    }
}

// $IMPL Sll r
#[derive(Debug, Clone, Copy)]
pub struct Sll;
impl Sll {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Sll please!");
        Ok(())
    }
}

// $IMPL Slt r
#[derive(Debug, Clone, Copy)]
pub struct Slt;
impl Slt {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Slt please!");
        Ok(())
    }
}

// $IMPL Sltu r
#[derive(Debug, Clone, Copy)]
pub struct Sltu;
impl Sltu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Sltu please!");
        Ok(())
    }
}

// $IMPL Xor r
#[derive(Debug, Clone, Copy)]
pub struct Xor;
impl Xor {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Xor please!");
        Ok(())
    }
}

// $IMPL Srl r
#[derive(Debug, Clone, Copy)]
pub struct Srl;
impl Srl {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Srl please!");
        Ok(())
    }
}

// $IMPL Sra r
#[derive(Debug, Clone, Copy)]
pub struct Sra;
impl Sra {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Sra please!");
        Ok(())
    }
}

// $IMPL Or r
#[derive(Debug, Clone, Copy)]
pub struct Or;
impl Or {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement Or please!");
        Ok(())
    }
}

// $IMPL And r
#[derive(Debug, Clone, Copy)]
pub struct And;
impl And {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
        todo!("implement And please!");
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

// $IMPL Slti i
#[derive(Debug, Clone, Copy)]
pub struct Slti;
impl Slti {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Slti please!");
        Ok(())
    }
}

// $IMPL Sltiu i
#[derive(Debug, Clone, Copy)]
pub struct Sltiu;
impl Sltiu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Sltiu please!");
        Ok(())
    }
}

// $IMPL Xori i
#[derive(Debug, Clone, Copy)]
pub struct Xori;
impl Xori {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Xori please!");
        Ok(())
    }
}

// $IMPL Ori i
#[derive(Debug, Clone, Copy)]
pub struct Ori;
impl Ori {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Ori please!");
        Ok(())
    }
}

// $IMPL Andi i
#[derive(Debug, Clone, Copy)]
pub struct Andi;
impl Andi {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Andi please!");
        Ok(())
    }
}

// $IMPL Slli i
#[derive(Debug, Clone, Copy)]
pub struct Slli;
impl Slli {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Slli please!");
        Ok(())
    }
}

// $IMPL Srli i
#[derive(Debug, Clone, Copy)]
pub struct Srli;
impl Srli {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Srli please!");
        Ok(())
    }
}

// $IMPL Srai i
#[derive(Debug, Clone, Copy)]
pub struct Srai;
impl Srai {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Srai please!");
        Ok(())
    }
}

// $IMPL Lb i
#[derive(Debug, Clone, Copy)]
pub struct Lb;
impl Lb {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lb please!");
        Ok(())
    }
}

// $IMPL Lh i
#[derive(Debug, Clone, Copy)]
pub struct Lh;
impl Lh {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lh please!");
        Ok(())
    }
}

// $IMPL Lw i
#[derive(Debug, Clone, Copy)]
pub struct Lw;
impl Lw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lw please!");
        Ok(())
    }
}

// $IMPL Lbu i
#[derive(Debug, Clone, Copy)]
pub struct Lbu;
impl Lbu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lbu please!");
        Ok(())
    }
}

// $IMPL Lhu i
#[derive(Debug, Clone, Copy)]
pub struct Lhu;
impl Lhu {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Lhu please!");
        Ok(())
    }
}

// $IMPL Jalr i
#[derive(Debug, Clone, Copy)]
pub struct Jalr;
impl Jalr {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Jalr please!");
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

// $IMPL Sh s
#[derive(Debug, Clone, Copy)]
pub struct Sh;
impl Sh {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Sh please!");
        Ok(())
    }
}

// $IMPL Sw s
#[derive(Debug, Clone, Copy)]
pub struct Sw;
impl Sw {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Sw please!");
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

// $IMPL Bne b
#[derive(Debug, Clone, Copy)]
pub struct Bne;
impl Bne {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Bne please!");
        Ok(())
    }
}

// $IMPL Blt b
#[derive(Debug, Clone, Copy)]
pub struct Blt;
impl Blt {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Blt please!");
        Ok(())
    }
}

// $IMPL Bge b
#[derive(Debug, Clone, Copy)]
pub struct Bge;
impl Bge {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Bge please!");
        Ok(())
    }
}

// $IMPL Bltu b
#[derive(Debug, Clone, Copy)]
pub struct Bltu;
impl Bltu {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Bltu please!");
        Ok(())
    }
}

// $IMPL Bgeu b
#[derive(Debug, Clone, Copy)]
pub struct Bgeu;
impl Bgeu {
    pub fn run(vm: &mut VM, r1: u64, r2: u64, offset: i64) -> Result<(), VMRunError> {
        todo!("implement Bgeu please!");
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

// $IMPL Auipc u
#[derive(Debug, Clone, Copy)]
pub struct Auipc;
impl Auipc {
    pub fn run(vm: &mut VM, rd: u8, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Auipc please!");
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
