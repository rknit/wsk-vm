use log::trace;

use super::super::{Inst, inst};

pub fn jal(rd: usize, imm: i64) -> Inst {
    trace!("jal x{rd}, {imm:#x}");
    inst!(vm {
        vm.set_x(rd, (vm.pc + 4) as u64);
        vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        Ok(())
    })
}
