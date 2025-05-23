use crate::inst_i;

inst_i!(
    (vm, rd, rs1, imm),
    addiw = {
        let rs1 = vm.x(rs1);
        let v = rs1.wrapping_add_signed(imm) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    slliw = {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u8; 4;0);
        let v = (rs1 << shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    srliw = {
        let rs1 = vm.x(rs1);
        let shamt = ext!(imm, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
    sraiw = {
        let rs1 = vm.x(rs1) as i64;
        let shamt = ext!(imm, u8; 4;0);
        let v = (rs1 >> shamt) as u32 as i32 as i64 as u64;
        vm.set_x(rd, v);
    },
);
