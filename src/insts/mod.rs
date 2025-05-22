use crate::{VM, VMRunError, ext};
use log::trace;

mod bits;
use bits::*;
mod format;
use format::*;

mod rv32i_rv64i;
use rv32i_rv64i::*;

pub const INST_LEN: usize = 4;

pub(crate) fn decode_inst(bytes: [u8; INST_LEN]) -> Inst {
    let inst = u32::from_be_bytes(bytes);
    // trace!("decoding: {inst:032b}");

    if inst == 0 {
        return halt();
    }

    match opcode(inst) {
        0b00000 => i(
            match funct3_14_12(inst) {
                0b000 => lb,
                0b001 => lh,
                0b010 => lw,
                0b100 => lbu,
                0b101 => lhu,
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
        0b01000 => s(
            match funct3_14_12(inst) {
                0b000 => sb,
                0b001 => sh,
                0b010 => sw,
                _ => unimplemented!(),
            },
            inst,
        ),
        0b01100 => r(
            match funct3_14_12(inst) {
                0b000 => {
                    if funct7_31_25(inst) > 0 {
                        sub
                    } else {
                        add
                    }
                }
                0b001 => sll,
                0b010 => slt,
                0b011 => sltu,
                0b100 => xor,
                0b101 => {
                    if funct7_31_25(inst) > 0 {
                        sra
                    } else {
                        srl
                    }
                }
                0b110 => or,
                0b111 => and,
                _ => unimplemented!(),
            },
            inst,
        ),
        0b01101 => u(lui, inst),
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
