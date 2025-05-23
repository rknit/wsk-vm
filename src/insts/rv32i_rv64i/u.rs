use crate::inst_u;

inst_u!(
    (vm, rd, imm),
    lui = {
        vm.set_x(rd, imm as u64);
    },
    auipc = {
        let pc = vm.pc as u64;
        vm.set_x(rd, pc.wrapping_add_signed(imm));
    },
);
