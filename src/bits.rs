#[inline]
pub const fn mask(len: uarch) -> uarch {
    !(0 as uarch) >> (8 * 8 - len)
}

#[inline]
pub const fn sext(v: uarch, sign_pos: uarch) -> iarch {
    let v_shl = (v << (64 - sign_pos - 1)) as iarch;
    let v_shr = v_shl >> (64 - sign_pos - 1);
    v_shr
}

#[inline]
pub const fn opcode(v: word) -> byte {
    ext!(v, byte; 6;2)
}

#[inline]
pub const fn funct3_14_12(v: word) -> byte {
    ext!(v, byte; 14;12)
}

#[inline]
pub const fn funct7_31_25(v: word) -> byte {
    ext!(v, byte; 31;25)
}

#[inline]
pub const fn rd_11_7(v: word) -> byte {
    ext!(v, byte; 11;7)
}

#[inline]
pub const fn rs1_19_15(v: word) -> byte {
    ext!(v, byte; 19;15)
}

#[inline]
pub const fn rs2_24_20(v: word) -> byte {
    ext!(v, byte; 24;20)
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

use crate::{byte, iarch, uarch, word};
