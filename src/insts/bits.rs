#[inline]
pub const fn mask(len: usize) -> u64 {
    (!0u64) >> (64 - len)
}

#[inline]
pub const fn sext(v: u64, sign_pos: usize) -> u64 {
    let v_shl = (v << (64 - sign_pos - 1)) as i64;
    let v_shr = v_shl >> (64 - sign_pos - 1);
    v_shr as u64
}

#[inline]
pub const fn sext_imm_12(v: u64) -> i64 {
    sext(v, 11) as i64
}

#[inline]
pub const fn sext_imm_13(v: u64) -> i64 {
    sext(v, 12) as i64
}

#[inline]
pub const fn sext_imm_21(v: u64) -> i64 {
    sext(v, 20) as i64
}

#[inline]
pub const fn opcode(v: u32) -> u8 {
    ext!(v, u8; 6;2)
}

#[inline]
pub const fn funct3_14_12(v: u32) -> u8 {
    ext!(v, u8; 14;12)
}

#[inline]
pub const fn funct7_31_25(v: u32) -> u8 {
    ext!(v, u8; 31;25)
}

#[inline]
pub const fn imm_31_20(v: u32) -> i64 {
    sext_imm_12(ext!(v, u64; 31;20))
}

#[inline]
pub const fn rd_11_7(v: u32) -> usize {
    ext!(v, usize; 11;7)
}

#[inline]
pub const fn rs1_19_15(v: u32) -> usize {
    ext!(v, usize; 19;15)
}

#[inline]
pub const fn rs2_24_20(v: u32) -> usize {
    ext!(v, usize; 24;20)
}

#[macro_export]
macro_rules! ext {
    ($val:expr, $t:ty; $high:expr;$low:expr) => {{
        let s = $val >> $low;
        let m = $crate::insts::bits::mask($high - $low + 1);
        ((s as u64) & m) as $t
    }};
}

use ext;
