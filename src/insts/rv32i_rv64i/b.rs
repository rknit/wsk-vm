use log::trace;

use super::super::{Inst, inst};

pub fn beq(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("beq x{rs1}, x{rs2}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 == rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
        Ok(())
    })
}

pub fn bne(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("bne x{rs1}, x{rs2}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 != rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
        Ok(())
    })
}

pub fn blt(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("blt x{rs1}, x{rs2}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        if rs1 < rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
        Ok(())
    })
}

pub fn bge(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("bge x{rs1}, x{rs2}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        if rs1 >= rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
        Ok(())
    })
}

pub fn bltu(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("bltu x{rs1}, x{rs2}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 < rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
        Ok(())
    })
}

pub fn bgeu(rs1: usize, rs2: usize, imm: i64) -> Inst {
    trace!("bgeu x{rs1}, x{rs2}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 >= rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
        Ok(())
    })
}
