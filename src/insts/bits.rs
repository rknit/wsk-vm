pub const fn mask(len: usize) -> u128 {
    let mut v = 0u128;
    let mut i = 0;
    while i < len {
        v <<= 1;
        v |= 1;
        i += 1;
    }
    v
}

pub const fn sext(v: u128, sign_pos: usize) -> u128 {
    let mut sign = ext!(v, u128; sign_pos+1;sign_pos);
    sign <<= sign_pos;
    let mut rv = v;
    let mut i = sign_pos;
    while i < 128 {
        sign <<= 1;
        rv |= sign;
        i += 1;
    }
    rv
}

pub const fn sext_imm_12(v: u64) -> i64 {
    sext(v as u128, 11) as i64
}

pub const fn sext_imm_13(v: u64) -> i64 {
    sext(v as u128, 12) as i64
}

pub const fn sext_imm_21(v: u64) -> i64 {
    sext(v as u128, 20) as i64
}

pub const fn opcode(v: u32) -> u8 {
    ext!(v, u8; 6;2)
}

pub const fn funct3_14_12(v: u32) -> u8 {
    ext!(v, u8; 14;12)
}

pub const fn funct7_31_25(v: u32) -> u8 {
    ext!(v, u8; 31;25)
}

pub const fn imm_31_20(v: u32) -> i64 {
    sext_imm_12(ext!(v, u64; 31;20))
}

pub const fn rd_11_7(v: u32) -> usize {
    ext!(v, usize; 11;7)
}

pub const fn rs1_19_15(v: u32) -> usize {
    ext!(v, usize; 19;15)
}

pub const fn rs2_24_20(v: u32) -> usize {
    ext!(v, usize; 24;20)
}

#[macro_export]
macro_rules! ext {
    ($val:expr, $t:ty; $high:expr;$low:expr) => {{
        let s = $val >> $low;
        let m = $crate::insts::bits::mask($high - $low + 1);
        ((s as u128) & m) as $t
    }};
}

use ext;
