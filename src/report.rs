use std::fmt::Display;

use crate::{format::Format, insts::Inst, x_name};

pub struct InstReport {
    pub addr: usize,
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
                    imm_to_str(v.imm_fmt_i() as i32),
                )
            }
            Format::S => {
                write!(
                    f,
                    "{}, {}({})",
                    x_name(v.rs2()),
                    imm_to_str_hex(v.imm_fmt_s() as i32),
                    x_name(v.rs1()),
                )
            }
            Format::B => write!(
                f,
                "{}, {}, 0x{:x}",
                x_name(v.rs1()),
                x_name(v.rs2()),
                self.addr.wrapping_add_signed(v.imm_fmt_b() as isize),
            ),
            Format::U => {
                write!(
                    f,
                    "{}, {}",
                    x_name(v.rd()),
                    imm_to_str_hex(v.raw_imm_fmt_u() as i32)
                )
            }
            Format::J => {
                write!(
                    f,
                    "{}, 0x{:x}",
                    x_name(v.rd()),
                    self.addr.wrapping_add_signed(v.imm_fmt_j() as isize)
                )
            }
            Format::Unknown => write!(f, "<unknown format>"),
        }
    }
}

fn imm_to_str(imm: i32) -> String {
    if imm >= 0 {
        format!("{imm}")
    } else {
        format!("{imm}")
    }
}

fn imm_to_str_hex(imm: i32) -> String {
    if imm >= 0 {
        format!("0x{imm:x}")
    } else {
        format!("{imm}")
    }
}
