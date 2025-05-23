use crate::inst_r;

inst_r!(
    (vm, rd, rs1, rs2),
    add = {
        let rs1_val = vm.x(rs1);
        let rs2_val = vm.x(rs2);
        vm.set_x(rd, rs1_val.wrapping_add(rs2_val));
    },
    sub = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1.wrapping_sub(rs2));
    },
    sll = {
        let rs1 = vm.x(rs1);
        let shamt = ext!(vm.x(rs2), u64; 4;0);
        vm.set_x(rd, rs1 << shamt);
    },
    slt = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        vm.set_x(rd, (rs1 < rs2) as u64);
    },
    sltu = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, (rs1 < rs2) as u64);
    },
    xor = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 ^ rs2);
    },
    srl = {
        let rs1 = vm.x(rs1);
        let shamt = ext!(vm.x(rs2), u64; 4;0);
        vm.set_x(rd, rs1 >> shamt);
    },
    sra = {
        let rs1 = vm.x(rs1) as i64;
        let shamt = ext!(vm.x(rs2), u64; 4;0);
        vm.set_x(rd, (rs1 >> shamt) as u64);
    },
    or = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 | rs2);
    },
    and = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        vm.set_x(rd, rs1 & rs2);
    }
);
