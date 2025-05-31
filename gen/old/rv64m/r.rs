use crate::inst_r;

inst_r!(
    (vm, rd, rs1, rs2),
    mulw = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, rs1.wrapping_mul(rs2) as u32 as i32 as i64 as u64);
    },
    divw = {
        let rs1 = vm.x(rs1) as i32;
        let rs2 = vm.x(rs2) as i32;
        vm.set_x(rd, (rs1 / rs2) as i64 as u64);
    },
    divuw = {
        let rs1 = vm.x(rs1) as u32;
        let rs2 = vm.x(rs2) as u32;
        vm.set_x(rd, (rs1 / rs2) as i32 as i64 as u64);
    },
    remw = {
        let rs1 = vm.x(rs1) as i32;
        let rs2 = vm.x(rs2) as i32;
        vm.set_x(rd, (rs1 % rs2) as i64 as u64);
    },
    remuw = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, (rs1 % rs2) as i32 as i64 as u64);
    },
);
