// $GEN_VERSION 3
// This auto-generated file provides instruction set implementations.
// While you can customize the behavior, developers are strictly advised to
// modify only the `run` method in each instruction.
// Changes outside of these methods are not checked for compatibility and may be lost upon regeneration.
// Critical '$' comments must remain unaltered.
// Remember to back up this file frequently, as `gen_inst.py` can overwrite it.
use crate::*;

pub struct Add;
impl Add {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Add$
        data.set_rd(data.r1().wrapping_add(data.r2()));
        Ok(())
        // $IMPL_END Add$
    }
}

pub struct Sub;
impl Sub {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sub$
        data.set_rd(data.r1().wrapping_sub(data.r2()));
        Ok(())
        // $IMPL_END Sub$
    }
}

pub struct Sll;
impl Sll {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sll$
        data.set_rd(data.r1() << (data.r2() & 0b11111));
        Ok(())
        // $IMPL_END Sll$
    }
}

pub struct Slt;
impl Slt {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Slt$
        if data.r1s() < data.r2s() {
            data.set_rd(1);
        } else {
            data.set_rd(0);
        }
        Ok(())
        // $IMPL_END Slt$
    }
}

pub struct Sltu;
impl Sltu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sltu$
        if data.r1() < data.r2() {
            data.set_rd(1);
        } else {
            data.set_rd(0);
        }
        Ok(())
        // $IMPL_END Sltu$
    }
}

pub struct Xor;
impl Xor {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Xor$
        data.set_rd(data.r1() ^ data.r2());
        Ok(())
        // $IMPL_END Xor$
    }
}

pub struct Srl;
impl Srl {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srl$
        data.set_rd(data.r1() >> (data.r2() & 0b11111));
        Ok(())
        // $IMPL_END Srl$
    }
}

pub struct Sra;
impl Sra {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sra$
        data.set_rd((data.r1s() >> (data.r2() & 0b11111)) as UArch);
        Ok(())
        // $IMPL_END Sra$
    }
}

pub struct Or;
impl Or {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Or$
        data.set_rd(data.r1() | data.r2());
        Ok(())
        // $IMPL_END Or$
    }
}

pub struct And;
impl And {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START And$
        data.set_rd(data.r1() & data.r2());
        Ok(())
        // $IMPL_END And$
    }
}

pub struct SfenceVma;
impl SfenceVma {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START SfenceVma$
        todo!("implement SfenceVma please!");
        Ok(())
        // $IMPL_END SfenceVma$
    }
}

pub struct Addi;
impl Addi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Addi$
        data.set_rd(data.r1().wrapping_add_signed(data.imm_fmt_i()));
        Ok(())
        // $IMPL_END Addi$
    }
}

pub struct Slti;
impl Slti {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Slti$
        if data.r1s() < data.imm_fmt_i() {
            data.set_rd(1);
        } else {
            data.set_rd(0);
        }
        Ok(())
        // $IMPL_END Slti$
    }
}

pub struct Sltiu;
impl Sltiu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sltiu$
        if data.r1() < data.immu_fmt_i() {
            data.set_rd(1);
        } else {
            data.set_rd(0);
        }
        Ok(())
        // $IMPL_END Sltiu$
    }
}

pub struct Xori;
impl Xori {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Xori$
        data.set_rd(data.r1() ^ data.immu_fmt_i());
        Ok(())
        // $IMPL_END Xori$
    }
}

pub struct Ori;
impl Ori {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Ori$
        data.set_rd(data.r1() | data.immu_fmt_i());
        Ok(())
        // $IMPL_END Ori$
    }
}

pub struct Andi;
impl Andi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Andi$
        data.set_rd(data.r1() & data.immu_fmt_i());
        Ok(())
        // $IMPL_END Andi$
    }
}

pub struct Slli;
impl Slli {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Slli$
        data.set_rd(data.r1() << (data.immu_fmt_i() & 0b111111));
        Ok(())
        // $IMPL_END Slli$
    }
}

pub struct Srli;
impl Srli {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srli$
        data.set_rd(data.r1() >> (data.immu_fmt_i() & 0b111111));
        Ok(())
        // $IMPL_END Srli$
    }
}

pub struct Srai;
impl Srai {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srai$
        data.set_rd((data.r1s() >> (data.immu_fmt_i() & 0b111111)) as UArch);
        Ok(())
        // $IMPL_END Srai$
    }
}

pub struct Lb;
impl Lb {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lb$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let val = data.mem(addr)?;
        let val = val as SByte as SArch; // Sign-extend the byte
        data.set_rd(val as UArch);
        Ok(())
        // $IMPL_END Lb$
    }
}

pub struct Lh;
impl Lh {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lh$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let vals = data.vm.mem_range(addr, 2)?;
        let val = Half::from_le_bytes([vals[0], vals[1]]) as SHalf as SArch; // Sign-extend the halfword
        data.set_rd(val as UArch);
        Ok(())
        // $IMPL_END Lh$
    }
}

pub struct Lw;
impl Lw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lw$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let vals = data.mem_range(addr, 4)?;
        let val = Word::from_le_bytes([vals[0], vals[1], vals[2], vals[3]]) as SWord as SArch; // Sign-extend the word
        data.set_rd(val as UArch);
        Ok(())
        // $IMPL_END Lw$
    }
}

pub struct Lbu;
impl Lbu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lbu$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let val = data.mem(addr)?;
        let val = val as UArch; // Zero-extend the byte
        data.set_rd(val);
        Ok(())
        // $IMPL_END Lbu$
    }
}

pub struct Lhu;
impl Lhu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lhu$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let vals = data.mem_range(addr, 2)?;
        let val = Half::from_le_bytes([vals[0], vals[1]]) as UArch; // Zero-extend the halfword
        data.set_rd(val);
        Ok(())
        // $IMPL_END Lhu$
    }
}

pub struct Jalr;
impl Jalr {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Jalr$
        let return_address = data.vm.pc + 4;
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        data.vm.jump(addr & !1, true)?; // Jump to the target address
        data.set_rd(return_address); // Save the return address
        Ok(())
        // $IMPL_END Jalr$
    }
}

pub struct Fence;
impl Fence {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Fence$
        todo!("implement Fence please!");
        Ok(())
        // $IMPL_END Fence$
    }
}

pub struct FenceI;
impl FenceI {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START FenceI$
        todo!("implement FenceI please!");
        Ok(())
        // $IMPL_END FenceI$
    }
}

pub struct Csrrw;
impl Csrrw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Csrrw$
        todo!("implement Csrrw please!");
        Ok(())
        // $IMPL_END Csrrw$
    }
}

pub struct Csrrs;
impl Csrrs {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Csrrs$
        todo!("implement Csrrs please!");
        Ok(())
        // $IMPL_END Csrrs$
    }
}

pub struct Csrrc;
impl Csrrc {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Csrrc$
        todo!("implement Csrrc please!");
        Ok(())
        // $IMPL_END Csrrc$
    }
}

pub struct Csrrwi;
impl Csrrwi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Csrrwi$
        todo!("implement Csrrwi please!");
        Ok(())
        // $IMPL_END Csrrwi$
    }
}

pub struct Csrrsi;
impl Csrrsi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Csrrsi$
        todo!("implement Csrrsi please!");
        Ok(())
        // $IMPL_END Csrrsi$
    }
}

pub struct Csrrci;
impl Csrrci {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Csrrci$
        todo!("implement Csrrci please!");
        Ok(())
        // $IMPL_END Csrrci$
    }
}

pub struct Ecall;
impl Ecall {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Ecall$
        use crate::Exception;
        data.vm.raise(Exception::Ecall)
        // $IMPL_END Ecall$
    }
}

pub struct Ebreak;
impl Ebreak {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Ebreak$
        todo!("implement Ebreak please!");
        Ok(())
        // $IMPL_END Ebreak$
    }
}

pub struct Uret;
impl Uret {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Uret$
        todo!("implement Uret please!");
        Ok(())
        // $IMPL_END Uret$
    }
}

pub struct Sret;
impl Sret {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sret$
        todo!("implement Sret please!");
        Ok(())
        // $IMPL_END Sret$
    }
}

pub struct Mret;
impl Mret {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mret$
        todo!("implement Mret please!");
        Ok(())
        // $IMPL_END Mret$
    }
}

pub struct Wfi;
impl Wfi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Wfi$
        todo!("implement Wfi please!");
        Ok(())
        // $IMPL_END Wfi$
    }
}

pub struct Sb;
impl Sb {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sb$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_s());
        let val = data.r2() as Byte; // Get the least significant byte
        data.set_mem(addr, val)?;
        Ok(())
        // $IMPL_END Sb$
    }
}

pub struct Sh;
impl Sh {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sh$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_s());
        let val = data.r2() as Half; // Get the least significant halfword
        let bytes = val.to_le_bytes();
        data.set_mem_range(addr, &bytes)?;
        Ok(())
        // $IMPL_END Sh$
    }
}

pub struct Sw;
impl Sw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sw$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_s());
        let val = data.r2() as Word; // Get the least significant word
        let bytes = val.to_le_bytes();
        data.set_mem_range(addr, &bytes)?;
        Ok(())
        // $IMPL_END Sw$
    }
}

pub struct Beq;
impl Beq {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Beq$
        if data.r1() == data.r2() {
            data.vm.jump_pc_rel(data.imm_fmt_b(), true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Beq$
    }
}

pub struct Bne;
impl Bne {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Bne$
        if data.r1() != data.r2() {
            data.vm.jump_pc_rel(data.imm_fmt_b(), true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bne$
    }
}

pub struct Blt;
impl Blt {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Blt$
        if data.r1s() < data.r2s() {
            data.vm.jump_pc_rel(data.imm_fmt_b(), true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Blt$
    }
}

pub struct Bge;
impl Bge {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Bge$
        if data.r1s() >= data.r2s() {
            data.vm.jump_pc_rel(data.imm_fmt_b(), true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bge$
    }
}

pub struct Bltu;
impl Bltu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Bltu$
        if data.r1() < data.r2() {
            data.vm.jump_pc_rel(data.imm_fmt_b(), true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bltu$
    }
}

pub struct Bgeu;
impl Bgeu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Bgeu$
        if data.r1() >= data.r2() {
            data.vm.jump_pc_rel(data.imm_fmt_b(), true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bgeu$
    }
}

pub struct Lui;
impl Lui {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lui$
        // Load upper immediate (data.immu() has already performed shift-op and sign-extension)
        data.set_rd(data.immu_fmt_u());
        Ok(())
        // $IMPL_END Lui$
    }
}

pub struct Auipc;
impl Auipc {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Auipc$
        // Add upper immediate to the current PC (data.imm() has already performed shift-op and sign-extension)
        data.set_rd(data.vm.pc.wrapping_add_signed(data.imm_fmt_u()));
        Ok(())
        // $IMPL_END Auipc$
    }
}

pub struct Jal;
impl Jal {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Jal$
        let return_address = data.vm.pc + 4;
        data.set_rd(return_address as u64); // Save the return address
        data.vm.jump_pc_rel(data.imm_fmt_j(), true)?; // Jump to the target address
        Ok(())
        // $IMPL_END Jal$
    }
}
