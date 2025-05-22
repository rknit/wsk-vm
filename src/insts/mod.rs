use crate::{
    VM, VMRunError,
    bits::{funct3_14_12, funct7_31_25},
    ext, i, opcode, r, u,
};
use log::trace;

mod rv32i_rv64i_i;
mod rv32i_rv64i_r;

use rv32i_rv64i_i::*;
use rv32i_rv64i_r::*;

pub const INST_LEN: usize = 4;

pub(crate) fn decode_inst(bytes: [u8; INST_LEN]) -> Inst {
    let inst = u32::from_be_bytes(bytes);
    trace!("decoding: {inst:032b}");

    if inst == 0 {
        return halt();
    }

    match opcode(inst) {
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
