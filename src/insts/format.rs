use super::bits::*;
use crate::{Inst, ext};

#[inline]
pub fn r(f: impl Fn(usize, usize, usize) -> Inst, inst32: u32) -> Inst {
    f(rd_11_7(inst32), rs1_19_15(inst32), rs2_24_20(inst32))
}

#[inline]
pub fn i(f: impl Fn(usize, usize, i64) -> Inst, inst32: u32) -> Inst {
    f(rd_11_7(inst32), rs1_19_15(inst32), imm_31_20(inst32))
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
    let imm_11 = ext!(inst32, u64; 7;7);
    let imm_12 = ext!(inst32, u64; 31;31);
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
    let imm_10_1 = ext!(inst32, u64; 30;21);
    let imm_11 = ext!(inst32, u64; 20;20);
    let imm_19_12 = ext!(inst32, u64; 19;12);
    let imm_20 = ext!(inst32, u64; 31;31);
    let imm = sext_imm_21((imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1));
    f(rd_11_7(inst32), imm)
}

#[macro_export]
macro_rules! inst_r {
    (($vm:ident, $rd:ident, $rs1:ident, $rs2:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rd: usize, $rs1: usize, $rs2: usize) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.r(stringify!($name), $rd, $rs1, $rs2);
                    $body
                }
            )
        }
        )*
    };
}

#[macro_export]
macro_rules! inst_u {
    (($vm:ident, $rd:ident, $imm:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rd: usize, $imm: i64) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.u(stringify!($name), $rd, $imm);
                    $body
                }
            )
        }
        )*
    };
}

#[macro_export]
macro_rules! inst_i {
    (($vm:ident, $rd:ident, $rs1:ident, $imm:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rd: usize, $rs1: usize, $imm: i64) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.i(stringify!($name), $rd, $rs1, $imm);
                    $body
                }
            )
        }
        )*
    };
}

#[macro_export]
macro_rules! inst_l {
    (($vm:ident, $rd:ident, $rs1:ident, $imm:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rd: usize, $rs1: usize, $imm: i64) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.l(stringify!($name), $rd, $rs1, $imm);
                    $body
                }
            )
        }
        )*
    };
}

#[macro_export]
macro_rules! inst_b {
    (($vm:ident, $rs1:ident, $rs2:ident, $imm:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rs1: usize, $rs2: usize, $imm: i64) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.b(stringify!($name), $rs1, $rs2, $imm);
                    $body
                }
            )
        }
        )*
    };
}

#[macro_export]
macro_rules! inst_j {
    (($vm:ident, $rd:ident, $imm:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rd: usize, $imm: i64) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.j(stringify!($name), $rd, $imm);
                    $body
                }
            )
        }
        )*
    };
}

#[macro_export]
macro_rules! inst_s {
    (($vm:ident, $rs1:ident, $rs2:ident, $imm:ident), $(
        $name:ident = $body:block
    ),* $(,)?) => {
        #[allow(unused_imports)]
        use $crate::{ext, insts::bits::*};

        $(
        pub fn $name($rs1: usize, $rs2: usize, $imm: i64) -> $crate::insts::Inst {
            $crate::inst!(
                $vm = {
                    $vm.rep.s(stringify!($name), $rs1, $rs2, $imm);
                    $body
                }
            )
        }
        )*
    };
}
