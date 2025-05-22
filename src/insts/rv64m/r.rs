use log::trace;

use super::super::{Inst, inst};

pub fn mulw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("mulw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, rs1.wrapping_mul(rs2) as u32 as i32 as i64 as u64);
        Ok(())
    })
}

pub fn divw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("divw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i32;
        let rs2 = vm.x(rs2) as i32;
        vm.set_x(rd, (rs1 / rs2) as i64 as u64);
        Ok(())
    })
}

pub fn divuw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("divuw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as u32;
        let rs2 = vm.x(rs2) as u32;
        vm.set_x(rd, (rs1 / rs2) as i32 as i64 as u64);
        Ok(())
    })
}

pub fn remw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("remw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i32;
        let rs2 = vm.x(rs2) as i32;
        vm.set_x(rd, (rs1 % rs2) as i64 as u64);
        Ok(())
    })
}

pub fn remuw(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("remuw x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, (rs1 % rs2) as i32 as i64 as u64);
        Ok(())
    })
}
