use super::bits::*;
use crate::{VM, VMRunError, ext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Unknown,
    R,
    I,
    S,
    B,
    U,
    J,
}

pub struct RunData<'vm> {
    pub inst: RawInst,
    pub vm: &'vm mut VM,
}
#[allow(dead_code)]
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
    #[inline]
    pub const fn new(inst: u32) -> Self {
        RawInst(inst)
    }

    #[inline]
    pub const fn raw(&self) -> u32 {
        self.0
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
        sext(self.immu(hi, lo), hi - lo)
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
