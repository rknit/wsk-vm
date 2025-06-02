#![allow(dead_code)]
use super::bits::*;
use crate::{VM, VMRunError, ext};

pub struct RunData<'vm> {
    pub inst: RawInst,
    pub vm: &'vm mut VM,
}
impl<'vm> RunData<'vm> {
    #[inline]
    pub const fn new(inst: RawInst, vm: &'vm mut VM) -> Self {
        RunData { inst, vm }
    }

    #[inline]
    pub fn x(&self, reg: u8) -> u64 {
        self.vm.x(reg)
    }

    #[inline]
    pub fn xs(&self, reg: u8) -> i64 {
        self.vm.x(reg) as i64
    }

    #[inline]
    pub fn set_x(&mut self, reg: u8, value: u64) {
        self.vm.set_x(reg, value);
    }

    #[inline]
    pub fn set_xs(&mut self, reg: u8, value: i64) {
        self.vm.set_x(reg, value as u64);
    }

    #[inline]
    pub fn mem(&self, addr: usize) -> Result<u8, VMRunError> {
        self.vm.mem(addr)
    }

    #[inline]
    pub fn set_mem(&mut self, addr: usize, value: u8) -> Result<(), VMRunError> {
        self.vm.set_mem(addr, value)
    }

    #[inline]
    pub fn mem_range(&self, addr: usize, len: usize) -> Result<&[u8], VMRunError> {
        self.vm.mem_range(addr, len)
    }

    #[inline]
    pub fn set_mem_range(&mut self, addr: usize, data: &[u8]) -> Result<(), VMRunError> {
        self.vm.set_mem_range(addr, data)
    }

    #[inline]
    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.inst.rd(), value);
    }

    #[inline]
    pub fn set_rds(&mut self, value: i64) {
        self.vm.set_x(self.inst.rd(), value as u64);
    }

    #[inline]
    pub fn r1(&self) -> u64 {
        self.vm.x(self.inst.rs1())
    }

    #[inline]
    pub fn r1s(&self) -> i64 {
        self.vm.x(self.inst.rs1()) as i64
    }

    #[inline]
    pub fn r2(&self) -> u64 {
        self.vm.x(self.inst.rs2())
    }

    #[inline]
    pub fn r2s(&self) -> i64 {
        self.vm.x(self.inst.rs2()) as i64
    }

    #[inline]
    pub const fn immu(&self, hi: usize, lo: usize) -> u64 {
        self.inst.immu(hi, lo)
    }

    #[inline]
    pub const fn imm(&self, hi: usize, lo: usize) -> i64 {
        self.inst.imm(hi, lo)
    }

    #[inline]
    pub const fn imm_fmt_i(&self) -> i64 {
        self.inst.imm_fmt_i()
    }

    #[inline]
    pub const fn immu_fmt_i(&self) -> u64 {
        self.inst.immu_fmt_i()
    }

    #[inline]
    pub const fn imm_fmt_s(&self) -> i64 {
        self.inst.imm_fmt_s()
    }

    #[inline]
    pub const fn imm_fmt_b(&self) -> i64 {
        self.inst.imm_fmt_b()
    }

    #[inline]
    pub const fn imm_fmt_u(&self) -> i64 {
        self.inst.imm_fmt_u()
    }

    #[inline]
    pub const fn immu_fmt_u(&self) -> u64 {
        self.inst.immu_fmt_u()
    }

    #[inline]
    pub const fn imm_fmt_j(&self) -> i64 {
        self.inst.imm_fmt_j()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawInst(u32);
impl RawInst {
    pub fn new(inst: u32) -> Self {
        RawInst(inst)
    }

    #[inline]
    pub const fn opcode(&self) -> u8 {
        opcode(self.0)
    }

    #[inline]
    pub const fn funct3(&self) -> u8 {
        funct3_14_12(self.0)
    }

    #[inline]
    pub const fn funct7(&self) -> u8 {
        funct7_31_25(self.0)
    }

    #[inline]
    pub const fn rd(&self) -> u8 {
        rd_11_7(self.0)
    }

    #[inline]
    pub const fn rs1(&self) -> u8 {
        rs1_19_15(self.0)
    }

    #[inline]
    pub const fn rs2(&self) -> u8 {
        rs2_24_20(self.0)
    }

    #[inline]
    pub const fn immu(&self, hi: usize, lo: usize) -> u64 {
        ext!(self.0, u64; hi; lo)
    }

    #[inline]
    pub const fn imm(&self, hi: usize, lo: usize) -> i64 {
        sext(self.immu(hi, lo), hi - lo + 1)
    }

    #[inline]
    pub const fn imm_fmt_i(&self) -> i64 {
        self.imm(31, 20)
    }

    #[inline]
    pub const fn immu_fmt_i(&self) -> u64 {
        self.imm_fmt_i() as u64
    }

    #[inline]
    pub const fn imm_fmt_s(&self) -> i64 {
        sext(self.immu(31, 25) << 5 | self.immu(11, 7), 11)
    }

    #[inline]
    pub const fn raw_imm_fmt_b(&self) -> i64 {
        sext(
            (self.immu(31, 31) << 11)
                | (self.immu(7, 7) << 10)
                | (self.immu(30, 25) << 4)
                | (self.immu(11, 8)),
            11,
        )
    }

    #[inline]
    pub const fn imm_fmt_b(&self) -> i64 {
        self.raw_imm_fmt_b() << 1
    }

    #[inline]
    pub const fn raw_imm_fmt_u(&self) -> i64 {
        sext(self.immu(31, 12), 19)
    }

    #[inline]
    pub const fn imm_fmt_u(&self) -> i64 {
        (self.raw_imm_fmt_u() << 12) as i32 as i64
    }

    #[inline]
    pub const fn immu_fmt_u(&self) -> u64 {
        self.imm_fmt_u() as u64
    }

    #[inline]
    pub const fn raw_imm_fmt_j(&self) -> i64 {
        sext(
            (self.immu(31, 31) << 19)
                | (self.immu(19, 12) << 11)
                | (self.immu(20, 20) << 10)
                | (self.immu(30, 21)),
            19,
        )
    }

    #[inline]
    pub const fn imm_fmt_j(&self) -> i64 {
        self.raw_imm_fmt_j() << 1
    }
}
impl From<u32> for RawInst {
    fn from(inst: u32) -> Self {
        RawInst::new(inst)
    }
}

pub struct DataR<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}
impl<'vm> DataR<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r2(&self) -> u64 {
        self.vm.x(self.rs2)
    }

    pub fn r1s(&self) -> i64 {
        self.vm.x(self.rs1) as i64
    }

    pub fn r2s(&self) -> i64 {
        self.vm.x(self.rs2) as i64
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

pub struct DataI<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub rs1: u8,
    pub raw_imm: i16,
}
impl<'vm> DataI<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r1s(&self) -> i64 {
        self.vm.x(self.rs1) as i64
    }

    pub fn imm(&self) -> i64 {
        sext(self.raw_imm as u64, 11)
    }

    pub fn immu(&self) -> u64 {
        self.imm() as u64
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

pub struct DataS<'vm> {
    pub vm: &'vm mut VM,
    pub rs1: u8,
    pub rs2: u8,
    pub raw_imm: i16,
}
impl<'vm> DataS<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r2(&self) -> u64 {
        self.vm.x(self.rs2)
    }

    pub fn r1s(&self) -> i64 {
        self.vm.x(self.rs1) as i64
    }

    pub fn r2s(&self) -> i64 {
        self.vm.x(self.rs2) as i64
    }

    pub fn imm(&self) -> i64 {
        sext(self.raw_imm as u64, 11)
    }

    pub fn immu(&self) -> u64 {
        self.imm() as u64
    }
}

pub struct DataB<'vm> {
    pub vm: &'vm mut VM,
    pub rs1: u8,
    pub rs2: u8,
    pub raw_imm: i16,
}
impl<'vm> DataB<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r2(&self) -> u64 {
        self.vm.x(self.rs2)
    }

    pub fn r1s(&self) -> i64 {
        self.vm.x(self.rs1) as i64
    }

    pub fn r2s(&self) -> i64 {
        self.vm.x(self.rs2) as i64
    }

    pub fn imm(&self) -> i64 {
        sext((self.raw_imm as u64) << 1, 12)
    }

    pub fn immu(&self) -> u64 {
        self.imm() as u64
    }
}

pub struct DataU<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub raw_imm: i32,
}
impl<'vm> DataU<'vm> {
    pub fn imm(&self) -> i64 {
        ((self.raw_imm as u64) << 12) as i32 as i64
    }

    pub fn immu(&self) -> u64 {
        self.imm() as u64
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

pub struct DataJ<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub raw_imm: i32,
}
impl<'vm> DataJ<'vm> {
    pub fn imm(&self) -> i64 {
        sext((self.raw_imm as u64) << 1, 20)
    }

    pub fn immu(&self) -> u64 {
        self.imm() as u64
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RawFormat {
    R {
        opc: u8,
        f3: u8,
        f7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    I {
        opc: u8,
        f3: u8,
        rd: u8,
        rs1: u8,
        imm: i16,
    },
    S {
        opc: u8,
        f3: u8,
        rs1: u8,
        rs2: u8,
        imm: i16,
    },
    B {
        opc: u8,
        f3: u8,
        rs1: u8,
        rs2: u8,
        imm: i16,
    },
    U {
        opc: u8,
        rd: u8,
        imm: i32,
    },
    J {
        opc: u8,
        rd: u8,
        imm: i32,
    },
}
impl RawFormat {
    pub fn parse(inst: u32) -> Option<Self> {
        let opc = opcode(inst);
        let funct3 = funct3_14_12(inst);
        let funct7 = funct7_31_25(inst);

        let rd = rd_11_7(inst) as u8;
        let rs1 = rs1_19_15(inst) as u8;
        let rs2 = rs2_24_20(inst) as u8;

        let imm_i = || sext_imm_12(ext!(inst, u64; 31;20)) as i16;
        let imm_s = || {
            let imm_4_0 = rd_11_7(inst) as u64;
            let imm_11_5 = funct7_31_25(inst) as u64;
            let imm = sext_imm_12((imm_11_5 << 5) | imm_4_0);
            imm as i16
        };
        let imm_b = || {
            let imm_4_1 = ext!(inst, u64; 11;8);
            let imm_10_5 = ext!(inst, u64; 30;25);
            let imm_11 = ext!(inst, u64; 7;7);
            let imm_12 = ext!(inst, u64; 31;31);
            let imm = sext_imm_12((imm_12 << 11) | (imm_11 << 10) | (imm_10_5 << 4) | imm_4_1);
            imm as i16
        };
        let imm_u = || {
            let imm_31_12 = ext!(inst, u64; 31;12);
            let imm = sext_imm_20(imm_31_12);
            imm as i32
        };
        let imm_j = || {
            let imm_10_1 = ext!(inst, u64; 30;21);
            let imm_11 = ext!(inst, u64; 20;20);
            let imm_19_12 = ext!(inst, u64; 19;12);
            let imm_20 = ext!(inst, u64; 31;31);
            let imm = sext_imm_20((imm_20 << 19) | (imm_19_12 << 11) | (imm_11 << 10) | imm_10_1);
            imm as i32
        };

        Some(match opc {
            0b00000 | 0b00100 | 0b00110 | 0b11001 | 0b11100 => RawFormat::I {
                opc,
                rd,
                rs1,
                imm: imm_i(),
                f3: funct3,
            },
            0b00101 | 0b01101 => RawFormat::U {
                opc,
                rd,
                imm: imm_u(),
            },
            0b01000 => RawFormat::S {
                opc,
                rs1,
                rs2,
                imm: imm_s(),
                f3: funct3,
            },
            0b01100 | 0b01110 => RawFormat::R {
                opc,
                rd,
                rs1,
                rs2,
                f3: funct3,
                f7: funct7,
            },
            0b11000 => RawFormat::B {
                opc,
                rs1,
                rs2,
                imm: imm_b(),
                f3: funct3,
            },
            0b11011 => RawFormat::J {
                opc,
                rd,
                imm: imm_j(),
            },
            _ => return None,
        })
    }
}
