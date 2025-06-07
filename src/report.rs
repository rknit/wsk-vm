use std::fmt::Display;

use crate::{Byte, SArch, SWord, UArch, format::Format, insts::Inst};

pub struct InstReport {
    pub addr: UArch,
    pub inst: Inst,
}
impl Display for InstReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}\t{:08x}\t{}\t",
            self.addr,
            self.inst.inner().raw(),
            self.inst.name(),
        )?;
        self.fmt_args(f)
    }
}

impl InstReport {
    fn fmt_args(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.inst.inner();
        match self.inst.format() {
            Format::R => {
                write!(
                    f,
                    "{}, {}, {}",
                    x_name(v.rd()),
                    x_name(v.rs1()),
                    x_name(v.rs2()),
                )
            }
            Format::I => {
                write!(
                    f,
                    "{}, {}, {}",
                    x_name(v.rd()),
                    x_name(v.rs1()),
                    imm_to_str(v.imm_fmt_i() as SWord),
                )
            }
            Format::S => {
                write!(
                    f,
                    "{}, {}({})",
                    x_name(v.rs2()),
                    imm_to_str_hex(v.imm_fmt_s() as SWord),
                    x_name(v.rs1()),
                )
            }
            Format::B => write!(
                f,
                "{}, {}, 0x{:x}",
                x_name(v.rs1()),
                x_name(v.rs2()),
                self.addr.wrapping_add_signed(v.imm_fmt_b() as SArch),
            ),
            Format::U => {
                write!(
                    f,
                    "{}, {}",
                    x_name(v.rd()),
                    imm_to_str_hex(v.raw_imm_fmt_u() as SWord)
                )
            }
            Format::J => {
                write!(
                    f,
                    "{}, 0x{:x}",
                    x_name(v.rd()),
                    self.addr.wrapping_add_signed(v.imm_fmt_j() as SArch)
                )
            }
            Format::Unknown => write!(f, "<unknown format>"),
            _ => todo!(),
        }
    }
}

fn imm_to_str(imm: SWord) -> String {
    format!("{imm}")
}

fn imm_to_str_hex(imm: SWord) -> String {
    if imm >= 0 {
        format!("0x{imm:x}")
    } else {
        format!("{imm}")
    }
}

pub fn x_name(i: Byte) -> &'static str {
    match i {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0",
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        _ => panic!("invalid register"),
    }
}

pub fn f_name(i: Byte) -> &'static str {
    match i {
        0 => "ft0",
        1 => "ft1",
        2 => "ft2",
        3 => "ft3",
        4 => "ft4",
        5 => "ft5",
        6 => "ft6",
        7 => "ft7",
        8 => "fs0",
        9 => "fs1",
        10 => "fa0",
        11 => "fa1",
        12 => "fa2",
        13 => "fa3",
        14 => "fa4",
        15 => "fa5",
        16 => "fa6",
        17 => "fa7",
        18 => "fs2",
        19 => "fs3",
        20 => "fs4",
        21 => "fs5",
        22 => "fs6",
        23 => "fs7",
        24 => "fs8",
        25 => "fs9",
        26 => "fs10",
        27 => "fs11",
        28 => "ft8",
        29 => "ft9",
        30 => "ft10",
        31 => "ft11",
        _ => panic!("invalid register"),
    }
}
