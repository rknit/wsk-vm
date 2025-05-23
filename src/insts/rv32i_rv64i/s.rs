use crate::inst_s;

inst_s!(
    (vm, rs1, rs2, imm),
    sb = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let data = ext!(rs2, u8; 7;0);
        let addr = rs1.wrapping_add_signed(imm) as usize;
        vm.set_mem(addr, data)?;
    },
    sh = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let addr = rs1.wrapping_add_signed(imm) as usize;
        for i in 0..2 {
            let data = ext!(rs2, u8; 7+i*8;i*8);
            vm.set_mem(addr + i, data)?;
        }
    },
    sw = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);

        let addr = rs1.wrapping_add_signed(imm) as usize;
        for i in 0..4 {
            let data = ext!(rs2, u8; 7+i*8;i*8);
            vm.set_mem(addr + i, data)?;
        }
    },
);
