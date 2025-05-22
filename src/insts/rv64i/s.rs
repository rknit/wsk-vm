use log::trace;

use crate::ext;

use super::super::{Inst, inst};

pub fn sd(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("sd x{rs2}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let addr = rs1.wrapping_add_signed(imm) as usize;
        for i in 0..8 {
            let data = ext!(rs2, u8; 7+i*8;i*8);
            vm.set_mem(addr + i, data)?;
        }
        Ok(())
    })
}
