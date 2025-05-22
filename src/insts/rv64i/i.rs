use log::trace;

use crate::{
    ext,
    insts::{Inst, inst},
};

pub fn addiw(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("addiw x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let v = rs1.wrapping_add_signed(imm) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn slliw(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("slliw x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u8; 4;0);
        let v = (rs1 << shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn srliw(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("srliw x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn sraiw(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("sraiw x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let shamt = ext!(imm, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}
