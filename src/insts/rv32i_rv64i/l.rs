use crate::inst_l;

inst_l!(
    (vm, rd, rs1, imm),
    lb = {
        let rs1_val = vm.x(rs1);
        let addr = rs1_val.wrapping_add_signed(imm);
        let data = vm.mem(addr as usize)?;
        let data = sext(data as u64, 7);
        vm.set_x(rd, data);
    },
    lh = {
        let rs1_val = vm.x(rs1);
        let addr = rs1_val.wrapping_add_signed(imm);
        let mut bytes: [u8; 2] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, i16::from_le_bytes(bytes) as i64 as u64);
    },
    lw = {
        let rs1_val = vm.x(rs1);
        let addr = rs1_val.wrapping_add_signed(imm);
        let mut bytes: [u8; 4] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, i32::from_le_bytes(bytes) as i64 as u64);
    },
    lbu = {
        let rs1_val = vm.x(rs1);
        let addr = rs1_val.wrapping_add_signed(imm);
        let data = vm.mem(addr as usize)?;
        vm.set_x(rd, data as u64);
    },
    lhu = {
        let rs1_val = vm.x(rs1);
        let addr = rs1_val.wrapping_add_signed(imm);
        let mut bytes: [u8; 2] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, u16::from_le_bytes(bytes) as u64);
    },
);
