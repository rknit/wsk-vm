use log::trace;

use super::super::{Inst, inst};

pub fn mul(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("mul x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, rs1.wrapping_mul(rs2) as u64);
        Ok(())
    })
}

pub fn mulh(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("mulh x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64 as i128;
        let rs2 = vm.x(rs2) as i64 as i128;
        vm.set_x(rd, (rs1.wrapping_mul(rs2) >> 64) as u64);
        Ok(())
    })
}

pub fn mulhsu(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("mulhsu x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64 as i128;
        let rs2 = vm.x(rs2) as i128;
        vm.set_x(rd, (rs1.wrapping_mul(rs2) >> 64) as u64);
        Ok(())
    })
}

pub fn mulhu(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("mulhu x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as u128;
        let rs2 = vm.x(rs2) as u128;
        vm.set_x(rd, (rs1.wrapping_mul(rs2) >> 64) as u64);
        Ok(())
    })
}

pub fn div(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("div x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, (rs1 / rs2) as u64);
        Ok(())
    })
}

pub fn divu(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("divu x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 / rs2);
        Ok(())
    })
}

pub fn rem(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("rem x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, (rs1 % rs2) as u64);
        Ok(())
    })
}

pub fn remu(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("remu x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 % rs2);
        Ok(())
    })
}
