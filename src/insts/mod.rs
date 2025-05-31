use crate::{VM, VMRunError, VMRunErrorKind};

mod bits;
use bits::*;

mod format;
use format::*;

mod rv32i_rv64i;
use rv32i_rv64i::*;

mod rv64i;
use rv64i::*;

#[derive(Debug, Clone, Copy)]
pub enum Inst {
    // RV32I_RV64I
    Lui { rd: u8, imm: i64 },
    Addi { rd: u8, r1: u64, imm: i64 },
    Add { rd: u8, r1: u64, r2: u64 },
    Ecall {},
    Sb { r1: u64, r2: u64, offset: i64 },
    Beq { r1: u64, r2: u64, offset: i64 },

    // RV64I
    Addiw { rd: u8, r1: u64, imm: i64 },
}

impl Inst {
    pub fn decode(fmt: RawFormat) -> Option<Self> {
        use RawFormat::*;
        Some(match fmt {
            R {
                opc,
                funct3,
                funct7,
                rd,
                rs1,
                rs2,
            } => match (opc, funct3, funct7) {
                (0b01100, 0b000, 0b0000000) => Inst::Add { rd, rs1, rs2 },

                #[allow(unreachable_patterns)]
                _ => return None,
            },
            I {
                opc,
                funct3,
                rd,
                rs1,
                imm,
            } => match (opc, funct3) {
                (0b00100, 0b000) => Inst::Addi { rd, rs1, imm },
                (0b00110, 0b000) => Inst::Addiw { rd, rs1, imm },

                #[allow(unreachable_patterns)]
                _ => return None,
            },
            S {
                opc,
                funct3,
                rs1,
                rs2,
                imm,
            } => match (opc, funct3) {
                (0b01000, 0b000) => Inst::Sb { rs1, rs2, offset },

                #[allow(unreachable_patterns)]
                _ => return None,
            },
            B {
                opc,
                funct3,
                rs1,
                rs2,
                imm,
            } => match (opc, funct3) {
                (0b11000, 0b000) => Inst::Beq { rs1, rs2, offset },

                #[allow(unreachable_patterns)]
                _ => return None,
            },
            U { opc, rd, imm } => match opc {
                0b01101 => Inst::Lui { rd, imm },

                #[allow(unreachable_patterns)]
                _ => return None,
            },
            J { opc, rd, imm } => match opc {
                #[allow(unreachable_patterns)]
                _ => return None,
            },
            Other { opc } => match opc {
                0b11100 => Inst::Ecall {},

                #[allow(unreachable_patterns)]
                _ => return None,
            },
        })
    }

    pub fn run(self, vm: &mut VM) -> Result<(), VMRunError> {
        match self {
            Inst::Lui { rd, imm } => Lui::run(vm, rd, imm),
            Inst::Addi { rd, rs1, imm } => Addi::run(vm, rd, rs1, imm),
            Inst::Add { rd, rs1, rs2 } => Add::run(vm, rd, rs1, rs2),
            Inst::Ecall {} => Ecall::run(vm),
            Inst::Sb { rs1, rs2, offset } => Sb::run(vm, rs1, rs2, offset),
            Inst::Beq { rs1, rs2, offset } => Beq::run(vm, rs1, rs2, offset),
            Inst::Addiw { rd, rs1, imm } => Addiw::run(vm, rd, rs1, imm),

            #[allow(unreachable_patterns)]
            _ => {
                return Err(VMRunError {
                    err_addr: vm.pc,
                    kind: VMRunErrorKind::Other(format!("{:?}", self)),
                    info: "unimplemented inst",
                });
            }
        }
    }
}
