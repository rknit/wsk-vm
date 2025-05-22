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

pub fn ld(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("ld x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 8] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, u64::from_le_bytes(bytes));
        Ok(())
    })
}

pub fn lwu(rd: usize, rs1: usize, imm: i64) -> Inst {
    trace!("lwu x{rd}, {imm:#x}(x{rs1})");
    inst!(vm {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 4] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, u32::from_le_bytes(bytes) as u64);
        Ok(())
    })
}
