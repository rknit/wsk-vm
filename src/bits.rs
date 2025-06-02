#[inline]
pub const fn mask(len: usize) -> u64 {
    !(0 as u64) >> (8 * 8 - len)
}

#[inline]
pub const fn sext(v: u64, sign_pos: usize) -> i64 {
    let v_shl = (v << (64 - sign_pos - 1)) as i64;
    let v_shr = v_shl >> (64 - sign_pos - 1);
    v_shr
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
pub const fn rd_11_7(v: u32) -> u8 {
    ext!(v, u8; 11;7)
}

#[inline]
pub const fn rs1_19_15(v: u32) -> u8 {
    ext!(v, u8; 19;15)
}

#[inline]
pub const fn rs2_24_20(v: u32) -> u8 {
    ext!(v, u8; 24;20)
}

mod utils {
    #[macro_export]
    macro_rules! ext {
        ($val:expr, $t:ty; $high:expr;$low:expr) => {{
            let s = $val >> $low;
            let m = $crate::bits::mask($high - $low + 1);
            ((s as u64) & m) as $t
        }};
    }

    pub(crate) use ext;
}

pub(crate) use utils::ext;
