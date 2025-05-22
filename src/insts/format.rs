use super::bits::*;
use crate::{Inst, ext};

#[inline]
pub fn r(f: impl Fn(usize, usize, usize) -> Inst, inst32: u32) -> Inst {
    f(rd_11_7(inst32), rs1_19_15(inst32), rs2_24_20(inst32))
}

#[inline]
pub fn i(f: impl Fn(usize, usize, i64) -> Inst, inst32: u32) -> Inst {
    f(rd_11_7(inst32), rs1_19_15(inst32), imm_32_20(inst32))
}

#[inline]
pub fn s(f: impl Fn(usize, usize, i64) -> Inst, inst32: u32) -> Inst {
    let imm_4_0 = rd_11_7(inst32) as u64;
    let imm_11_5 = funct7_31_25(inst32) as u64;
    let imm = sext_imm_12((imm_11_5 << 5) | imm_4_0);
    f(rs1_19_15(inst32), rs2_24_20(inst32), imm)
}

#[inline]
pub fn b(f: impl Fn(usize, usize, i64) -> Inst, inst32: u32) -> Inst {
    let imm_4_1 = ext!(inst32, u64; 11;8);
    let imm_10_5 = ext!(inst32, u64; 30;25);
    let imm_11 = ext!(inst32, u64; 8;7);
    let imm_12 = ext!(inst32, u64; 31;30);
    let imm = sext_imm_13((imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1));
    f(rs1_19_15(inst32), rs2_24_20(inst32), imm)
}

#[inline]
pub fn u(f: impl Fn(usize, i64) -> Inst, inst32: u32) -> Inst {
    let imm_31_12 = ext!(inst32, u128; 31;12);
    let imm = sext(imm_31_12 << 12, 31) as i64;
    f(rd_11_7(inst32), imm)
}

#[inline]
pub fn j(f: impl Fn(usize, i64) -> Inst, inst32: u32) -> Inst {
    let imm_10_1 = ext!(inst32, u64; 30;24);
    let imm_11 = ext!(inst32, u64; 21;20);
    let imm_19_12 = ext!(inst32, u64; 19;12);
    let imm_20 = ext!(inst32, u64; 31;30);
    let imm = sext_imm_21((imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1));
    f(rd_11_7(inst32), imm)
}
