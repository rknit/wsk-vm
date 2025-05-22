use log::trace;

use super::{Inst, inst};

pub fn add(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("add x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1.wrapping_add(rs2));
        Ok(())
    })
}
