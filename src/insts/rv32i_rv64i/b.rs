use crate::inst_b;

inst_b!(
    (vm, rs1, rs2, imm),
    beq = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 == rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
    },
    bne = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 != rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
    },
    blt = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        if rs1 < rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
    },
    bge = {
        let rs1 = vm.x(rs1) as i64;
        let rs2 = vm.x(rs2) as i64;
        if rs1 >= rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
    },
    bltu = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 < rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
    },
    bgeu = {
        let rs1 = vm.x(rs1);
        let rs2 = vm.x(rs2);
        if rs1 >= rs2 {
            vm.pc = vm.pc.wrapping_add_signed(imm as isize);
        }
    },
);
