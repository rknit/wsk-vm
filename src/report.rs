use std::fmt::Display;

use crate::{byte, format::Format, iarch, insts::Inst, sword, uarch};

pub struct InstReport {
    pub addr: uarch,
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
                    imm_to_str(v.imm_fmt_i() as sword),
                )
            }
            Format::S => {
                write!(
                    f,
                    "{}, {}({})",
                    x_name(v.rs2()),
                    imm_to_str_hex(v.imm_fmt_s() as sword),
                    x_name(v.rs1()),
                )
            }
            Format::B => write!(
                f,
                "{}, {}, 0x{:x}",
                x_name(v.rs1()),
                x_name(v.rs2()),
                self.addr.wrapping_add_signed(v.imm_fmt_b() as iarch),
            ),
            Format::U => {
                write!(
                    f,
                    "{}, {}",
                    x_name(v.rd()),
                    imm_to_str_hex(v.raw_imm_fmt_u() as sword)
                )
            }
            Format::J => {
                write!(
                    f,
                    "{}, 0x{:x}",
                    x_name(v.rd()),
                    self.addr.wrapping_add_signed(v.imm_fmt_j() as iarch)
                )
            }
            Format::Unknown => write!(f, "<unknown format>"),
        }
    }
}

fn imm_to_str(imm: sword) -> String {
    format!("{imm}")
}

fn imm_to_str_hex(imm: sword) -> String {
    if imm >= 0 {
        format!("0x{imm:x}")
    } else {
        format!("{imm}")
    }
}

pub fn x_name(i: byte) -> &'static str {
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
