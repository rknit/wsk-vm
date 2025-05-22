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

pub fn sext(v: impl Into<u128>, sign_pos: usize) -> u128 {
    let v = v.into();
    let mut sign = ext!(v, u128; sign_pos;sign_pos-1);
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

pub fn sext_20_32_to_i64(v: u32) -> i64 {
    sext(ext!(v, u32; 31;20), 11) as i64
}

#[macro_export]
macro_rules! ext {
    ($val:expr, $t:ty; $high:expr;$low:expr) => {{
        let s = $val >> $low;
        let m = $crate::mask($high - $low + 1);
        ((s as u128) & m) as $t
    }};
}

use ext;
