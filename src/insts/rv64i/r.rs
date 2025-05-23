use crate::inst_r;

inst_r!(
    (vm, rd, rs1, rs2),
    addw = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let v = rs1.wrapping_add(rs2) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    subw = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let v = rs1.wrapping_sub(rs2) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    sllw = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let shamt = ext!(rs2, u8; 4;0);
        let v = (rs1 << shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    srlw = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        let shamt = ext!(rs2, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    sraw = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2);
        let shamt = ext!(rs2, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
);
