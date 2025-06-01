// $GEN_VERSION 2
// This auto-generated file provides instruction set implementations.
// While you can customize the behavior, developers are strictly advised to
// modify only the `run` method in each instruction.
// Changes outside of these methods are not checked for compatibility and may be lost upon regeneration.
// Critical '$' comments must remain unaltered.
// Remember to back up this file frequently, as `gen_inst.py` can overwrite it.
use crate::*;

pub struct Add;
impl Add {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Add$
        data.set_rd(data.r1().wrapping_add(data.r2()));
        Ok(())
        // $IMPL_END Add$
    }
}

pub struct Sub;
impl Sub {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Sub$
        data.set_rd(data.r1().wrapping_sub(data.r2()));
        Ok(())
        // $IMPL_END Sub$
    }
}

pub struct Sll;
impl Sll {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Sll$
        data.set_rd(data.r1() << (data.r2() & 0b11111));
        Ok(())
        // $IMPL_END Sll$
    }
}

pub struct Slt;
impl Slt {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
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
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
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
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Xor$
        data.set_rd(data.r1() ^ data.r2());
        Ok(())
        // $IMPL_END Xor$
    }
}

pub struct Srl;
impl Srl {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Srl$
        data.set_rd(data.r1() >> (data.r2() & 0b11111));
        Ok(())
        // $IMPL_END Srl$
    }
}

pub struct Sra;
impl Sra {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Sra$
        data.set_rd((data.r1s() >> (data.r2() & 0b11111)) as u64);
        Ok(())
        // $IMPL_END Sra$
    }
}

pub struct Or;
impl Or {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Or$
        data.set_rd(data.r1() | data.r2());
        Ok(())
        // $IMPL_END Or$
    }
}

pub struct And;
impl And {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START And$
        data.set_rd(data.r1() & data.r2());
        Ok(())
        // $IMPL_END And$
    }
}

pub struct SfenceVma;
impl SfenceVma {
    pub fn run(mut data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START SfenceVma$
        todo!("implement SfenceVma please!");
        Ok(())
        // $IMPL_END SfenceVma$
    }
}

pub struct Addi;
impl Addi {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Addi$
        data.set_rd(data.r1().wrapping_add_signed(data.imm()));
        Ok(())
        // $IMPL_END Addi$
    }
}

pub struct Slti;
impl Slti {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Slti$
        if data.r1s() < data.imm() {
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
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Sltiu$
        if data.r1() < (data.imm() as u64) {
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
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Xori$
        data.set_rd(data.r1() ^ data.immu());
        Ok(())
        // $IMPL_END Xori$
    }
}

pub struct Ori;
impl Ori {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Ori$
        data.set_rd(data.r1() | data.immu());
        Ok(())
        // $IMPL_END Ori$
    }
}

pub struct Andi;
impl Andi {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Andi$
        data.set_rd(data.r1() & data.immu());
        Ok(())
        // $IMPL_END Andi$
    }
}

pub struct Slli;
impl Slli {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Slli$
        data.set_rd(data.r1() << (data.immu() & 0b111111));
        Ok(())
        // $IMPL_END Slli$
    }
}

pub struct Srli;
impl Srli {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Srli$
        data.set_rd(data.r1() >> (data.immu() & 0b111111));
        Ok(())
        // $IMPL_END Srli$
    }
}

pub struct Srai;
impl Srai {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Srai$
        data.set_rd((data.r1s() >> (data.immu() & 0b111111)) as u64);
        Ok(())
        // $IMPL_END Srai$
    }
}

pub struct Lb;
impl Lb {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Lb$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let val = data.vm.mem(addr)?;
        let val = val as i8 as i64; // Sign-extend the byte
        data.set_rd(val as u64);
        Ok(())
        // $IMPL_END Lb$
    }
}

pub struct Lh;
impl Lh {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Lh$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let vals = data.vm.mem_range(addr, 2)?;
        let val = u16::from_le_bytes([vals[0], vals[1]]) as i16 as i64; // Sign-extend the halfword
        data.set_rd(val as u64);
        Ok(())
        // $IMPL_END Lh$
    }
}

pub struct Lw;
impl Lw {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Lw$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let vals = data.vm.mem_range(addr, 4)?;
        let val = u32::from_le_bytes([vals[0], vals[1], vals[2], vals[3]]) as i32 as i64; // Sign-extend the word
        data.set_rd(val as u64);
        Ok(())
        // $IMPL_END Lw$
    }
}

pub struct Lbu;
impl Lbu {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Lbu$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let val = data.vm.mem(addr)?;
        let val = val as u64; // Zero-extend the byte
        data.set_rd(val);
        Ok(())
        // $IMPL_END Lbu$
    }
}

pub struct Lhu;
impl Lhu {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Lhu$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let vals = data.vm.mem_range(addr, 2)?;
        let val = u16::from_le_bytes([vals[0], vals[1]]) as u64; // Zero-extend the halfword
        data.set_rd(val);
        Ok(())
        // $IMPL_END Lhu$
    }
}

pub struct Jalr;
impl Jalr {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Jalr$
        let return_address = data.vm.pc + 4;
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        data.vm.jump(addr & !1, true)?; // Jump to the target address
        data.set_rd(return_address as u64); // Save the return address
        Ok(())
        // $IMPL_END Jalr$
    }
}

pub struct Fence;
impl Fence {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Fence$
        todo!("implement Fence please!");
        Ok(())
        // $IMPL_END Fence$
    }
}

pub struct FenceI;
impl FenceI {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START FenceI$
        todo!("implement FenceI please!");
        Ok(())
        // $IMPL_END FenceI$
    }
}

pub struct Csrrw;
impl Csrrw {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Csrrw$
        todo!("implement Csrrw please!");
        Ok(())
        // $IMPL_END Csrrw$
    }
}

pub struct Csrrs;
impl Csrrs {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Csrrs$
        todo!("implement Csrrs please!");
        Ok(())
        // $IMPL_END Csrrs$
    }
}

pub struct Csrrc;
impl Csrrc {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Csrrc$
        todo!("implement Csrrc please!");
        Ok(())
        // $IMPL_END Csrrc$
    }
}

pub struct Csrrwi;
impl Csrrwi {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Csrrwi$
        todo!("implement Csrrwi please!");
        Ok(())
        // $IMPL_END Csrrwi$
    }
}

pub struct Csrrsi;
impl Csrrsi {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Csrrsi$
        todo!("implement Csrrsi please!");
        Ok(())
        // $IMPL_END Csrrsi$
    }
}

pub struct Csrrci;
impl Csrrci {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Csrrci$
        todo!("implement Csrrci please!");
        Ok(())
        // $IMPL_END Csrrci$
    }
}

pub struct Ecall;
impl Ecall {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Ecall$
        use crate::Exception;
        data.vm.raise(Exception::Ecall)
        // $IMPL_END Ecall$
    }
}

pub struct Ebreak;
impl Ebreak {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Ebreak$
        todo!("implement Ebreak please!");
        Ok(())
        // $IMPL_END Ebreak$
    }
}

pub struct Uret;
impl Uret {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Uret$
        todo!("implement Uret please!");
        Ok(())
        // $IMPL_END Uret$
    }
}

pub struct Sret;
impl Sret {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Sret$
        todo!("implement Sret please!");
        Ok(())
        // $IMPL_END Sret$
    }
}

pub struct Mret;
impl Mret {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Mret$
        todo!("implement Mret please!");
        Ok(())
        // $IMPL_END Mret$
    }
}

pub struct Wfi;
impl Wfi {
    pub fn run(mut data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Wfi$
        todo!("implement Wfi please!");
        Ok(())
        // $IMPL_END Wfi$
    }
}

pub struct Sb;
impl Sb {
    pub fn run(mut data: DataS) -> Result<(), VMRunError> {
        // $IMPL_START Sb$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let val = data.r2() as u8; // Get the least significant byte
        data.vm.set_mem(addr, val)?;
        Ok(())
        // $IMPL_END Sb$
    }
}

pub struct Sh;
impl Sh {
    pub fn run(mut data: DataS) -> Result<(), VMRunError> {
        // $IMPL_START Sh$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let val = data.r2() as u16; // Get the least significant halfword
        let bytes = val.to_le_bytes();
        data.vm.set_mem_range(addr, &bytes)?;
        Ok(())
        // $IMPL_END Sh$
    }
}

pub struct Sw;
impl Sw {
    pub fn run(mut data: DataS) -> Result<(), VMRunError> {
        // $IMPL_START Sw$
        let addr = data.r1().wrapping_add_signed(data.imm()) as usize;
        let val = data.r2() as u32; // Get the least significant word
        let bytes = val.to_le_bytes();
        data.vm.set_mem_range(addr, &bytes)?;
        Ok(())
        // $IMPL_END Sw$
    }
}

pub struct Beq;
impl Beq {
    pub fn run(mut data: DataB) -> Result<(), VMRunError> {
        // $IMPL_START Beq$
        if data.r1() == data.r2() {
            data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Beq$
    }
}

pub struct Bne;
impl Bne {
    pub fn run(mut data: DataB) -> Result<(), VMRunError> {
        // $IMPL_START Bne$
        if data.r1() != data.r2() {
            data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bne$
    }
}

pub struct Blt;
impl Blt {
    pub fn run(mut data: DataB) -> Result<(), VMRunError> {
        // $IMPL_START Blt$
        if data.r1s() < data.r2s() {
            data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Blt$
    }
}

pub struct Bge;
impl Bge {
    pub fn run(mut data: DataB) -> Result<(), VMRunError> {
        // $IMPL_START Bge$
        if data.r1s() >= data.r2s() {
            data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bge$
    }
}

pub struct Bltu;
impl Bltu {
    pub fn run(mut data: DataB) -> Result<(), VMRunError> {
        // $IMPL_START Bltu$
        if data.r1() < data.r2() {
            data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bltu$
    }
}

pub struct Bgeu;
impl Bgeu {
    pub fn run(mut data: DataB) -> Result<(), VMRunError> {
        // $IMPL_START Bgeu$
        if data.r1() >= data.r2() {
            data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        }
        Ok(())
        // $IMPL_END Bgeu$
    }
}

pub struct Lui;
impl Lui {
    pub fn run(mut data: DataU) -> Result<(), VMRunError> {
        // $IMPL_START Lui$
        // Load upper immediate (data.immu() has already performed shift-op and sign-extension)
        data.set_rd(data.immu());
        Ok(())
        // $IMPL_END Lui$
    }
}

pub struct Auipc;
impl Auipc {
    pub fn run(mut data: DataU) -> Result<(), VMRunError> {
        // $IMPL_START Auipc$
        // Add upper immediate to the current PC (data.imm() has already performed shift-op and sign-extension)
        data.set_rd(data.vm.pc.wrapping_add_signed(data.imm() as isize) as u64);
        Ok(())
        // $IMPL_END Auipc$
    }
}

pub struct Jal;
impl Jal {
    pub fn run(mut data: DataJ) -> Result<(), VMRunError> {
        // $IMPL_START Jal$
        let return_address = data.vm.pc + 4;
        data.set_rd(return_address as u64); // Save the return address
        data.vm.jump_pc_rel(data.imm() as isize, true)?; // Jump to the target address
        Ok(())
        // $IMPL_END Jal$
    }
}
