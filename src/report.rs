use std::fmt::Display;

use crate::{bits::sext, format::RawFormat, x_name};

pub struct InstReport {
    pub addr: usize,
    pub raw_inst: u32,
    pub inst_name: &'static str,
    pub format: RawFormat,
}
impl Display for InstReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}\t{:08x}\t{}\t",
            self.addr, self.raw_inst, self.inst_name,
        )?;
        self.fmt_args(f)
    }
}

impl InstReport {
    fn fmt_args(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            RawFormat::R { rd, rs1, rs2, .. } => {
                write!(f, "{}, {}, {}", x_name(rd), x_name(rs1), x_name(rs2))
            }
            RawFormat::I { rd, rs1, imm, .. } => {
                write!(
                    f,
                    "{}, {}, {}",
                    x_name(rd),
                    x_name(rs1),
                    imm_to_str(imm as i32)
                )
            }
            RawFormat::S { rs1, rs2, imm, .. } => {
                write!(
                    f,
                    "{}, {}({})",
                    x_name(rs2),
                    imm_to_str(imm as i32),
                    x_name(rs1)
                )
            }
            RawFormat::B { rs1, rs2, imm, .. } => write!(
                f,
                "{}, {}, {:x}",
                x_name(rs1),
                x_name(rs2),
                self.addr
                    .wrapping_add_signed(sext((imm as u64) << 1, 12) as isize)
            ),
            RawFormat::U { rd, imm, .. } => {
                write!(f, "{}, {}", x_name(rd), imm_to_str(imm))
            }
            RawFormat::J { rd, imm, .. } => {
                write!(
                    f,
                    "{}, {:x}",
                    x_name(rd),
                    self.addr
                        .wrapping_add_signed(sext((imm as u64) << 1, 20) as isize)
                )
            }
        }
    }
}

fn imm_to_str(imm: i32) -> String {
    format!("{imm}")
}
