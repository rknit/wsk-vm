use log::trace;

use crate::{
    ext,
    insts::{Inst, inst},
};

pub fn addw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("addw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let v = rs1.wrapping_add(rs2) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn subw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("subw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let v = rs1.wrapping_sub(rs2) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn sllw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("sllw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let shamt = ext!(rs2, u8; 4;0);
        let v = (rs1 << shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn srlw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("srlw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let shamt = ext!(rs2, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn sraw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("sraw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2);
        let shamt = ext!(rs2, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}
