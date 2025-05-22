use log::trace;

use crate::ext;

use super::{Inst, inst};

pub fn sb(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("sb x{rs2}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let data = ext!(rs2, u8; 7;0);
        let addr = rs1.wrapping_add_signed(imm) as usize;
        vm.set_mem(addr, data)?;
        Ok(())
    })
}

pub fn sh(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("sh x{rs2}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let addr = rs1.wrapping_add_signed(imm) as usize;
        for i in 0..2 {
            let data = ext!(rs2, u8; 7+i*8;i*8);
            vm.set_mem(addr + i, data)?;
        }
        Ok(())
    })
}

pub fn sw(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("sw x{rs2}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let addr = rs1.wrapping_add_signed(imm) as usize;
        for i in 0..4 {
            let data = ext!(rs2, u8; 7+i*8;i*8);
            vm.set_mem(addr + i, data)?;
        }
        Ok(())
    })
}
