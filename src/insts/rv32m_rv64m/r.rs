use crate::inst_r;

inst_r!(
    (vm, rd, rs1, rs2),
    mul = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, rs1.wrapping_mul(rs2) as u64);
    },
    mulh = {
        let rs1 = vm.x(rs1) as i64 as i128;
        let rs2 = vm.x(rs2) as i64 as i128;
        vm.set_x(rd, (rs1.wrapping_mul(rs2) >> 64) as u64);
    },
    mulhsu = {
        let rs1 = vm.x(rs1) as i64 as i128;
        let rs2 = vm.x(rs2) as i128;
        vm.set_x(rd, (rs1.wrapping_mul(rs2) >> 64) as u64);
    },
    mulhu = {
        let rs1 = vm.x(rs1) as u128;
        let rs2 = vm.x(rs2) as u128;
        vm.set_x(rd, (rs1.wrapping_mul(rs2) >> 64) as u64);
    },
    div = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, (rs1 / rs2) as u64);
    },
    divu = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 / rs2);
    },
    rem = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, (rs1 % rs2) as u64);
    },
    remu = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 % rs2);
    },
);
