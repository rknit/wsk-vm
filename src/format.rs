use super::bits::*;
use crate::{VM, VMRunError, byte, ext, iarch, sword, uarch, word};

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
    pub fn x(&self, reg: byte) -> uarch {
        self.vm.x(reg)
    }

    #[inline]
    pub fn xs(&self, reg: byte) -> iarch {
        self.vm.x(reg) as i64
    }

    #[inline]
    pub fn set_x(&mut self, reg: byte, value: uarch) {
        self.vm.set_x(reg, value);
    }

    #[inline]
    pub fn set_xs(&mut self, reg: byte, value: iarch) {
        self.vm.set_x(reg, value as uarch);
    }

    #[inline]
    pub fn mem(&self, addr: uarch) -> Result<byte, VMRunError> {
        self.vm.mem(addr)
    }

    #[inline]
    pub fn set_mem(&mut self, addr: uarch, value: byte) -> Result<(), VMRunError> {
        self.vm.set_mem(addr, value)
    }

    #[inline]
    pub fn mem_range(&self, addr: uarch, len: uarch) -> Result<&[byte], VMRunError> {
        self.vm.mem_range(addr, len)
    }

    #[inline]
    pub fn set_mem_range(&mut self, addr: uarch, data: &[byte]) -> Result<(), VMRunError> {
        self.vm.set_mem_range(addr, data)
    }

    #[inline]
    pub fn set_rd(&mut self, value: uarch) {
        self.vm.set_x(self.inst.rd(), value);
    }

    #[inline]
    pub fn set_rds(&mut self, value: iarch) {
        self.vm.set_x(self.inst.rd(), value as uarch);
    }

    #[inline]
    pub fn r1(&self) -> uarch {
        self.vm.x(self.inst.rs1())
    }

    #[inline]
    pub fn r1s(&self) -> iarch {
        self.vm.x(self.inst.rs1()) as iarch
    }

    #[inline]
    pub fn r2(&self) -> uarch {
        self.vm.x(self.inst.rs2())
    }

    #[inline]
    pub fn r2s(&self) -> iarch {
        self.vm.x(self.inst.rs2()) as iarch
    }

    #[inline]
    pub const fn immu(&self, hi: uarch, lo: uarch) -> uarch {
        self.inst.immu(hi, lo)
    }

    #[inline]
    pub const fn imm(&self, hi: uarch, lo: uarch) -> iarch {
        self.inst.imm(hi, lo)
    }

    #[inline]
    pub const fn imm_fmt_i(&self) -> iarch {
        self.inst.imm_fmt_i()
    }

    #[inline]
    pub const fn immu_fmt_i(&self) -> uarch {
        self.inst.immu_fmt_i()
    }

    #[inline]
    pub const fn imm_fmt_s(&self) -> iarch {
        self.inst.imm_fmt_s()
    }

    #[inline]
    pub const fn imm_fmt_b(&self) -> iarch {
        self.inst.imm_fmt_b()
    }

    #[inline]
    pub const fn imm_fmt_u(&self) -> iarch {
        self.inst.imm_fmt_u()
    }

    #[inline]
    pub const fn immu_fmt_u(&self) -> uarch {
        self.inst.immu_fmt_u()
    }

    #[inline]
    pub const fn imm_fmt_j(&self) -> iarch {
        self.inst.imm_fmt_j()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawInst(word);
impl RawInst {
    #[inline]
    pub const fn new(inst: word) -> Self {
        RawInst(inst)
    }

    #[inline]
    pub const fn raw(&self) -> word {
        self.0
    }

    #[inline]
    pub const fn opcode(&self) -> byte {
        opcode(self.0)
    }

    #[inline]
    pub const fn funct3(&self) -> byte {
        funct3_14_12(self.0)
    }

    #[inline]
    pub const fn funct7(&self) -> byte {
        funct7_31_25(self.0)
    }

    #[inline]
    pub const fn rd(&self) -> byte {
        rd_11_7(self.0)
    }

    #[inline]
    pub const fn rs1(&self) -> byte {
        rs1_19_15(self.0)
    }

    #[inline]
    pub const fn rs2(&self) -> byte {
        rs2_24_20(self.0)
    }

    #[inline]
    pub const fn immu(&self, hi: uarch, lo: uarch) -> uarch {
        ext!(self.0, uarch; hi; lo)
    }

    #[inline]
    pub const fn imm(&self, hi: uarch, lo: uarch) -> iarch {
        sext(self.immu(hi, lo), hi - lo)
    }

    #[inline]
    pub const fn imm_fmt_i(&self) -> iarch {
        self.imm(31, 20)
    }

    #[inline]
    pub const fn immu_fmt_i(&self) -> uarch {
        self.imm_fmt_i() as uarch
    }

    #[inline]
    pub const fn imm_fmt_s(&self) -> iarch {
        sext(self.immu(31, 25) << 5 | self.immu(11, 7), 11)
    }

    #[inline]
    pub const fn raw_imm_fmt_b(&self) -> iarch {
        sext(
            (self.immu(31, 31) << 11)
                | (self.immu(7, 7) << 10)
                | (self.immu(30, 25) << 4)
                | (self.immu(11, 8)),
            11,
        )
    }

    #[inline]
    pub const fn imm_fmt_b(&self) -> iarch {
        self.raw_imm_fmt_b() << 1
    }

    #[inline]
    pub const fn raw_imm_fmt_u(&self) -> iarch {
        sext(self.immu(31, 12), 19)
    }

    #[inline]
    pub const fn imm_fmt_u(&self) -> iarch {
        (self.raw_imm_fmt_u() << 12) as sword as iarch
    }

    #[inline]
    pub const fn immu_fmt_u(&self) -> uarch {
        self.imm_fmt_u() as uarch
    }

    #[inline]
    pub const fn raw_imm_fmt_j(&self) -> iarch {
        sext(
            (self.immu(31, 31) << 19)
                | (self.immu(19, 12) << 11)
                | (self.immu(20, 20) << 10)
                | (self.immu(30, 21)),
            19,
        )
    }

    #[inline]
    pub const fn imm_fmt_j(&self) -> iarch {
        self.raw_imm_fmt_j() << 1
    }
}
impl From<word> for RawInst {
    fn from(inst: word) -> Self {
        RawInst::new(inst)
    }
}
