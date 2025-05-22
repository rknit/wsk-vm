use log::trace;

use crate::{VM, VMRunError, bits::sext_20_32_to_i64, ext};

pub const INST_LEN: usize = 4;

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

pub(crate) fn decode_inst(bytes: [u8; INST_LEN]) -> Box<dyn RunInst> {
    let inst32 = u32::from_be_bytes(bytes);
    trace!("decoding: {:032b}", inst32);

    if inst32 == 0 {
        return halt();
    }

    match ext!(inst32, u8; 6;2) {
        0b00100 => {
            let rd = ext!(inst32, usize; 11;7);
            let rs1 = ext!(inst32, usize; 19;15);
            let imm = sext_20_32_to_i64(inst32);
            addi(rd, rs1, imm)
        }
        0b01100 => {
            let rd = ext!(inst32, usize; 11;7);
            let rs1 = ext!(inst32, usize; 19;15);
            let rs2 = ext!(inst32, usize; 24;20);
            add(rd, rs1, rs2)
        }
        _ => unimplemented!(),
    }
}

fn halt() -> Box<dyn RunInst> {
    Box::new(move |vm: &mut VM| {
        vm.halt();
        Ok(())
    })
}

fn add(rd: usize, rs1: usize, rs2: usize) -> Box<dyn RunInst> {
    trace!("add x{rd}, x{rs1}, x{rs2}");
    Box::new(move |vm: &mut VM| {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1.wrapping_add(rs2));
        Ok(())
    })
}

fn addi(rd: usize, rs1: usize, imm: i64) -> Box<dyn RunInst> {
    trace!("addi x{rd}, x{rs1}, {imm:012b}");
    Box::new(move |vm: &mut VM| {
        let rs1 = vm.x(rs1) as i64;
        let v = rs1.wrapping_add(imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}
