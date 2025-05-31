use crate::inst_j;

inst_j!(
    (vm, rd, imm),
    jal = {
        vm.set_x(rd, (vm.pc + 4) as u64);
        vm.pc = vm.pc.wrapping_add_signed(imm as isize - 4);
    },
);
