use log::trace;

use super::super::{Inst, ext, inst, sext};

pub fn lui(rd: usize, imm: i64) -> Inst {
    trace!("lui x{rd}, {imm:#x}");
    inst!(vm {
        vm.set_x(rd, imm as u64);
        Ok(())
    })
}

pub fn auipc(rd: usize, imm: i64) -> Inst {
    trace!("auipc x{rd}, {imm:#x}");
    inst!(vm {
        let pc = vm.pc as u64;
        vm.set_x(rd, pc.wrapping_add_signed(imm));
        Ok(())
    })
}

pub fn addi(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("addi x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = rs1.wrapping_add(imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn slli(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("slli x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, rs1 << shamt);
        Ok(())
    })
}

pub fn srli(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("srli x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, rs1 >> shamt);
        Ok(())
    })
}

pub fn srai(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("srai x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, (rs1 >> shamt) as u64);
        Ok(())
    })
}

pub fn slti(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("slti x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 < imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn sltiu(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("sltiu x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let v = (rs1 < (imm as u64)) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn xori(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("xori x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 ^ imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn ori(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("ori x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 | imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn andi(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("andi x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1) as i64;
        let v = (rs1 & imm) as u64;
        vm.set_x(rd, v);
        Ok(())
    })
}

pub fn lb(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("lb x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let data = vm.mem(addr as usize)?;
        let data = sext(data as u128, 7) as u64;
        vm.set_x(rd, data);
        Ok(())
    })
}

pub fn lh(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("lh x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 2] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, i16::from_le_bytes(bytes) as i64 as u64);
        Ok(())
    })
}

pub fn lw(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("lw x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 4] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, i32::from_le_bytes(bytes) as i64 as u64);
        Ok(())
    })
}

pub fn lbu(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("lbu x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let data = vm.mem(addr as usize)?;
        vm.set_x(rd, data as u64);
        Ok(())
    })
}

pub fn lhu(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("lhu x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 2] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, u16::from_le_bytes(bytes) as u64);
        Ok(())
    })
}

pub fn jalr(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("jalr x{rd}, x{rs1}, {imm:#x}");
    inst!(vm {
        let rs1 = vm.x(rs1);
        vm.set_x(rd, (vm.pc + 4) as u64);
        vm.pc = (rs1.wrapping_add_signed(imm) & !1u64) as usize;
        Ok(())
    })
}
