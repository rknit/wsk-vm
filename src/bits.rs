#[inline]
pub const fn mask(len: UArch) -> UArch {
    !(0 as UArch) >> (8 * 8 - len)
}

#[inline]
pub const fn sext(v: UArch, sign_pos: UArch) -> SArch {
    let v_shl = (v << (64 - sign_pos - 1)) as SArch;
    let v_shr = v_shl >> (64 - sign_pos - 1);
    v_shr
}

#[inline]
pub const fn opcode(v: Word) -> Byte {
    ext!(v, Byte; 6;2)
}

#[inline]
pub const fn funct3_14_12(v: Word) -> Byte {
    ext!(v, Byte; 14;12)
}

#[inline]
pub const fn funct7_31_25(v: Word) -> Byte {
    ext!(v, Byte; 31;25)
}

#[inline]
pub const fn rd_11_7(v: Word) -> Byte {
    ext!(v, Byte; 11;7)
}

#[inline]
pub const fn rs1_19_15(v: Word) -> Byte {
    ext!(v, Byte; 19;15)
}

#[inline]
pub const fn rs2_24_20(v: Word) -> Byte {
    ext!(v, Byte; 24;20)
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

use crate::{Byte, SArch, UArch, Word};
