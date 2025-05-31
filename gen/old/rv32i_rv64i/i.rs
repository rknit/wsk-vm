use crate::inst_i;

inst_i!(
    (vm, rd, rs1, imm),
    addi = {
        let rs1_val = vm.x(rs1) as i64;
        let v = rs1_val.wrapping_add(imm) as u64;
        vm.set_x(rd, v);
    },
    slli = {
        let rs1_val = vm.x(rs1);
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, rs1_val << shamt);
    },
    srli = {
        let rs1_val = vm.x(rs1);
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, rs1_val >> shamt);
    },
    srai = {
        let rs1_val = vm.x(rs1) as i64;
        let shamt = ext!(imm, u64; 5;0);
        vm.set_x(rd, (rs1_val >> shamt) as u64);
    },
    slti = {
        let rs1_val = vm.x(rs1) as i64;
        let v = (rs1_val < imm) as u64;
        vm.set_x(rd, v);
    },
    sltiu = {
        let rs1_val = vm.x(rs1);
        let v = (rs1_val < (imm as u64)) as u64;
        vm.set_x(rd, v);
    },
    xori = {
        let rs1_val = vm.x(rs1) as i64;
        let v = (rs1_val ^ imm) as u64;
        vm.set_x(rd, v);
    },
    ori = {
        let rs1_val = vm.x(rs1) as i64;
        let v = (rs1_val | imm) as u64;
        vm.set_x(rd, v);
    },
    andi = {
        let rs1_val = vm.x(rs1) as i64;
        let v = (rs1_val & imm) as u64;
        vm.set_x(rd, v);
    },
    jalr = {
        let rs1_val = vm.x(rs1);
        vm.set_x(rd, (vm.pc + 4) as u64);
        vm.pc = ((rs1_val.wrapping_add_signed(imm) & !1u64) - 4) as usize;
    },
);
