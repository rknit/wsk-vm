use crate::inst_l;

inst_l!(
    (vm, rd, rs1, imm),
    ld = {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 8] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, u64::from_le_bytes(bytes));
    },
    lwu = {
        let rs1 = vm.x(rs1);
        let addr = rs1.wrapping_add_signed(imm);
        let mut bytes: [u8; 4] = Default::default();
        for (i, byte) in bytes.iter_mut().enumerate() {
            let v = vm.mem((addr as usize) + i)?;
            *byte = v;
        }
        vm.set_x(rd, u32::from_le_bytes(bytes) as u64);
    },
);
