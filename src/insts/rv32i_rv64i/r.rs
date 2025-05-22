use log::trace;

use crate::ext;

use super::super::{Inst, inst};

pub fn add(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("add x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1.wrapping_add(rs2));
        Ok(())
    })
}

pub fn sub(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("sub x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1.wrapping_sub(rs2));
        Ok(())
    })
}

pub fn sll(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("sll x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(vm.x(rs2), u64; 4;0);
        vm.set_x(rd, rs1 << shamt);
        Ok(())
    })
}

pub fn slt(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("slt x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, (rs1 < rs2) as u64);
        Ok(())
    })
}

pub fn sltu(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("sltu x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, (rs1 < rs2) as u64);
        Ok(())
    })
}

pub fn xor(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("xor x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 ^ rs2);
        Ok(())
    })
}

pub fn srl(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("srl x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(vm.x(rs2), u64; 4;0);
        vm.set_x(rd, rs1 >> shamt);
        Ok(())
    })
}

pub fn sra(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("sra x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let shamt = ext!(vm.x(rs2), u64; 4;0);
        vm.set_x(rd, (rs1 >> shamt) as u64);
        Ok(())
    })
}

pub fn or(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("or x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 | rs2);
        Ok(())
    })
}

pub fn and(rd: usize, rs1: usize, rs2: usize) -> Inst {
    trace!("or x{rd}, x{rs1}, x{rs2}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 & rs2);
        Ok(())
    })
}
