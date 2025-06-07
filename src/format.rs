use super::bits::*;
use crate::{Byte, DFP, SArch, SWord, UArch, VM, VMRunError, Word, ext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Unknown,
    R,
    I,
    S,
    B,
    U,
    J,

    CR,
    CI,
    CSS,
    CIW,
    CL,
    CS,
    CA,
    CB,
    CJ,
}

pub struct RunData<'vm> {
    pub inst: RawInst,
    pub vm: &'vm mut VM,
}
#[allow(dead_code)]
impl<'vm> RunData<'vm> {
    #[inline(always)]
    pub const fn new(inst: RawInst, vm: &'vm mut VM) -> Self {
        RunData { inst, vm }
    }

    #[inline(always)]
    pub fn f(&self, reg: Byte) -> DFP {
        self.vm.f(reg)
    }

    #[inline(always)]
    pub fn set_f(&mut self, reg: Byte, value: DFP) {
        self.vm.set_f(reg, value);
    }

    #[inline(always)]
    pub fn x(&self, reg: Byte) -> UArch {
        self.vm.x(reg)
    }

    #[inline(always)]
    pub fn xs(&self, reg: Byte) -> SArch {
        self.vm.x(reg) as i64
    }

    #[inline(always)]
    pub fn set_x(&mut self, reg: Byte, value: UArch) {
        self.vm.set_x(reg, value);
    }

    #[inline(always)]
    pub fn set_xs(&mut self, reg: Byte, value: SArch) {
        self.vm.set_x(reg, value as UArch);
    }

    #[inline(always)]
    pub fn mem(&self, addr: UArch) -> Result<Byte, VMRunError> {
        self.vm.mem(addr)
    }

    #[inline(always)]
    pub fn set_mem(&mut self, addr: UArch, value: Byte) -> Result<(), VMRunError> {
        self.vm.set_mem(addr, value)
    }

    #[inline(always)]
    pub fn mem_range(&self, addr: UArch, len: UArch) -> Result<&[Byte], VMRunError> {
        self.vm.mem_range(addr, len)
    }

    #[inline(always)]
    pub fn set_mem_range(&mut self, addr: UArch, data: &[Byte]) -> Result<(), VMRunError> {
        self.vm.set_mem_range(addr, data)
    }

    #[inline(always)]
    pub fn set_rd(&mut self, value: UArch) {
        self.vm.set_x(self.inst.rd(), value);
    }

    #[inline(always)]
    pub fn set_rds(&mut self, value: SArch) {
        self.vm.set_x(self.inst.rd(), value as UArch);
    }

    #[inline(always)]
    pub fn r1(&self) -> UArch {
        self.vm.x(self.inst.rs1())
    }

    #[inline(always)]
    pub fn r1s(&self) -> SArch {
        self.vm.x(self.inst.rs1()) as SArch
    }

    #[inline(always)]
    pub fn r2(&self) -> UArch {
        self.vm.x(self.inst.rs2())
    }

    #[inline(always)]
    pub fn r2s(&self) -> SArch {
        self.vm.x(self.inst.rs2()) as SArch
    }

    #[inline(always)]
    pub const fn immu(&self, hi: UArch, lo: UArch) -> UArch {
        self.inst.immu(hi, lo)
    }

    #[inline(always)]
    pub const fn imm(&self, hi: UArch, lo: UArch) -> SArch {
        self.inst.imm(hi, lo)
    }

    #[inline(always)]
    pub const fn imm_fmt_i(&self) -> SArch {
        self.inst.imm_fmt_i()
    }

    #[inline(always)]
    pub const fn immu_fmt_i(&self) -> UArch {
        self.inst.immu_fmt_i()
    }

    #[inline(always)]
    pub const fn imm_fmt_s(&self) -> SArch {
        self.inst.imm_fmt_s()
    }

    #[inline(always)]
    pub const fn imm_fmt_b(&self) -> SArch {
        self.inst.imm_fmt_b()
    }

    #[inline(always)]
    pub const fn imm_fmt_u(&self) -> SArch {
        self.inst.imm_fmt_u()
    }

    #[inline(always)]
    pub const fn immu_fmt_u(&self) -> UArch {
        self.inst.immu_fmt_u()
    }

    #[inline(always)]
    pub const fn imm_fmt_j(&self) -> SArch {
        self.inst.imm_fmt_j()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawInst(Word);
impl RawInst {
    #[inline(always)]
    pub const fn new(inst: Word) -> Self {
        RawInst(inst)
    }

    #[inline(always)]
    pub const fn raw(&self) -> Word {
        self.0
    }

    #[inline(always)]
    pub const fn opcode(&self) -> Byte {
        opcode(self.0)
    }

    #[inline(always)]
    pub const fn funct3(&self) -> Byte {
        funct3_14_12(self.0)
    }

    #[inline(always)]
    pub const fn funct7(&self) -> Byte {
        funct7_31_25(self.0)
    }

    #[inline(always)]
    pub const fn rd(&self) -> Byte {
        rd_11_7(self.0)
    }

    #[inline(always)]
    pub const fn rs1(&self) -> Byte {
        rs1_19_15(self.0)
    }

    #[inline(always)]
    pub const fn rs2(&self) -> Byte {
        rs2_24_20(self.0)
    }

    #[inline(always)]
    pub const fn immu(&self, hi: UArch, lo: UArch) -> UArch {
        ext!(self.0, UArch; hi; lo)
    }

    #[inline(always)]
    pub const fn imm(&self, hi: UArch, lo: UArch) -> SArch {
        sext(self.immu(hi, lo), hi - lo)
    }

    #[inline(always)]
    pub const fn imm_fmt_i(&self) -> SArch {
        self.imm(31, 20)
    }

    #[inline(always)]
    pub const fn immu_fmt_i(&self) -> UArch {
        self.imm_fmt_i() as UArch
    }

    #[inline(always)]
    pub const fn imm_fmt_s(&self) -> SArch {
        sext(self.immu(31, 25) << 5 | self.immu(11, 7), 11)
    }

    #[inline(always)]
    pub const fn raw_imm_fmt_b(&self) -> SArch {
        sext(
            (self.immu(31, 31) << 11)
                | (self.immu(7, 7) << 10)
                | (self.immu(30, 25) << 4)
                | (self.immu(11, 8)),
            11,
        )
    }

    #[inline(always)]
    pub const fn imm_fmt_b(&self) -> SArch {
        self.raw_imm_fmt_b() << 1
    }

    #[inline(always)]
    pub const fn raw_imm_fmt_u(&self) -> SArch {
        sext(self.immu(31, 12), 19)
    }

    #[inline(always)]
    pub const fn imm_fmt_u(&self) -> SArch {
        (self.raw_imm_fmt_u() << 12) as SWord as SArch
    }

    #[inline(always)]
    pub const fn immu_fmt_u(&self) -> UArch {
        self.imm_fmt_u() as UArch
    }

    #[inline(always)]
    pub const fn raw_imm_fmt_j(&self) -> SArch {
        sext(
            (self.immu(31, 31) << 19)
                | (self.immu(19, 12) << 11)
                | (self.immu(20, 20) << 10)
                | (self.immu(30, 21)),
            19,
        )
    }

    #[inline(always)]
    pub const fn imm_fmt_j(&self) -> SArch {
        self.raw_imm_fmt_j() << 1
    }
}
impl From<Word> for RawInst {
    fn from(inst: Word) -> Self {
        RawInst::new(inst)
    }
}
