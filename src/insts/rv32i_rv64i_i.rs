use log::trace;

use crate::ext;

use super::{Inst, inst};

pub fn lui(rd: usize, imm: i64) -> Inst {
    trace!("lui x{rd}, ${imm:x}");
    inst!(vm {
        vm.set_x(rd, imm as u64);
        Ok(())
    })
}

pub fn auipc(rd: usize, imm: i64) -> Inst {
    trace!("auipc x{rd}, ${imm:x}");
    inst!(vm {
        let pc = vm.pc as u64;
        vm.set_x(rd, pc.wrapping_add_signed(imm));
        Ok(())
    })
}

pub fn addi(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("addi x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = rs1.wrapping_add(imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn slli(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("slli x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, rs1 << shamt);
        Ok(())
    })
}

pub fn srli(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("srli x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, rs1 >> shamt);
        Ok(())
    })
}

pub fn srai(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("srai x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, (rs1 >> shamt) as u64);
        Ok(())
    })
}

pub fn slti(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("slti x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 < imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn sltiu(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("sltiu x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let v = (rs1 < (imm as u64)) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn xori(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("xori x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 ^ imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn ori(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("ori x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 | imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn andi(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("andi x{rd}, x{rs1}, ${imm:x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 & imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}
