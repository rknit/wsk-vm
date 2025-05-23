use crate::{VM, VMRunError, ext};
use log::trace;

mod bits;
use bits::*;
mod format;
use format::*;

mod rv32i_rv64i;
use rv32i_rv64i::*;

mod rv64i;
use rv64i::*;

mod rv32m_rv64m;
use rv32m_rv64m::*;

mod rv64m;
use rv64m::*;

pub const INST_LEN: usize = 4;

pub(crate) fn decode_inst(bytes: [u8; INST_LEN]) -> Inst {
    let inst = u32::from_le_bytes(bytes);
    trace!("decoding {inst:08x}");

    if inst == 0 {
        return halt();
    }

    match opcode(inst) {
        0b00000 => i(
            match funct3_14_12(inst) {
                0b000 => lb,
                0b001 => lh,
                0b010 => lw,
                0b011 => ld,
                0b100 => lbu,
                0b101 => lhu,
                0b110 => lwu,
                _ => unimplemented!(),
            },
            inst,
        ),
        0b00100 => i(
            match funct3_14_12(inst) {
                0b000 => addi,
                0b001 => slli,
                0b010 => slti,
                0b011 => sltiu,
                0b100 => xori,
                0b101 => {
                    if ext!(inst, u8; 31;27) > 0 {
                        srai
                    } else {
                        srli
                    }
                }
                0b110 => ori,
                0b111 => andi,
                _ => unimplemented!(),
            },
            inst,
        ),
        0b00101 => u(auipc, inst),
        0b00110 => i(
            match funct3_14_12(inst) {
                0b000 => addiw,
                0b001 => slliw,
                0b101 => {
                    if funct7_31_25(inst) > 0 {
                        sraiw
                    } else {
                        srliw
                    }
                }
                _ => unimplemented!(),
            },
            inst,
        ),
        0b01000 => s(
            match funct3_14_12(inst) {
                0b000 => sb,
                0b001 => sh,
                0b010 => sw,
                0b011 => sd,
                _ => unimplemented!(),
            },
            inst,
        ),
        0b01100 => r(
            match funct3_14_12(inst) {
                0b000 => match funct7_31_25(inst) {
                    0b0000000 => add,
                    0b0000001 => mul,
                    0b0100000 => sub,
                    _ => unimplemented!(),
                },
                0b001 => match funct7_31_25(inst) {
                    0b0000000 => sll,
                    0b0000001 => mulh,
                    _ => unimplemented!(),
                },
                0b010 => match funct7_31_25(inst) {
                    0b0000000 => slt,
                    0b0000001 => mulhsu,
                    _ => unimplemented!(),
                },
                0b011 => match funct7_31_25(inst) {
                    0b0000000 => sltu,
                    0b0000001 => mulhu,
                    _ => unimplemented!(),
                },
                0b100 => match funct7_31_25(inst) {
                    0b0000000 => xor,
                    0b0000001 => div,
                    _ => unimplemented!(),
                },
                0b101 => match funct7_31_25(inst) {
                    0b0000000 => srl,
                    0b0000001 => divu,
                    0b0100000 => sra,
                    _ => unimplemented!(),
                },
                0b110 => match funct7_31_25(inst) {
                    0b0000000 => or,
                    0b0000001 => rem,
                    _ => unimplemented!(),
                },
                0b111 => match funct7_31_25(inst) {
                    0b0000000 => and,
                    0b0000001 => remu,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            inst,
        ),
        0b01101 => u(lui, inst),
        0b01110 => r(
            match funct3_14_12(inst) {
                0b000 => match funct7_31_25(inst) {
                    0b0000000 => addw,
                    0b0000001 => mulw,
                    0b0100000 => subw,
                    _ => unimplemented!(),
                },
                0b001 => sllw,
                0b100 => match funct7_31_25(inst) {
                    0b0000001 => divw,
                    _ => unimplemented!(),
                },
                0b101 => match funct7_31_25(inst) {
                    0b0000000 => srlw,
                    0b0000001 => divuw,
                    0b0100000 => sraw,
                    _ => unimplemented!(),
                },
                0b110 => match funct7_31_25(inst) {
                    0b0000001 => remw,
                    _ => unimplemented!(),
                },
                0b111 => match funct7_31_25(inst) {
                    0b0000001 => remuw,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            inst,
        ),
        0b11000 => b(
            match funct3_14_12(inst) {
                0b000 => beq,
                0b001 => bne,
                0b100 => blt,
                0b101 => bge,
                0b110 => bltu,
                0b111 => bgeu,
                _ => unimplemented!(),
            },
            inst,
        ),
        0b11001 => i(jalr, inst),
        0b11011 => j(jal, inst),
        _ => unimplemented!(),
    }
}

fn halt() -> Inst {
    trace!("halt");
    inst!(vm {
        vm.halt = true;
        Ok(())
    })
}

pub(crate) trait RunInst {
    fn run_inst(&self, vm: &mut VM) -> Result<(), VMRunError>;
}

impl<F> RunInst for F
where
    F: Fn(&mut VM) -> Result<(), VMRunError>,
{
    fn run_inst(&self, vm: &mut VM) -> Result<(), VMRunError> {
        self(vm)
    }
}

pub type Inst = Box<dyn RunInst>;

macro_rules! inst {
    ($vm:ident $body:block) => {{ Box::new(move |$vm: &mut $crate::VM| -> Result<(), $crate::VMRunError> { $body }) }};
}
use inst;
