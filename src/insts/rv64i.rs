// $GEN_VERSION 3
// This auto-generated file provides instruction set implementations.
// While you can customize the behavior, developers are strictly advised to
// modify only the `run` method in each instruction.
// Changes outside of these methods are not checked for compatibility and may be lost upon regeneration.
// Critical '$' comments must remain unaltered.
// Remember to back up this file frequently, as `gen_inst.py` can overwrite it.
use crate::*;

pub struct Addw;
impl Addw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Addw$
        let result = data.r1().wrapping_add(data.r2());
        let result = (result & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Addw$
    }
}

pub struct Subw;
impl Subw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Subw$
        let result = data.r1().wrapping_sub(data.r2());
        let result = (result & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Subw$
    }
}

pub struct Sllw;
impl Sllw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sllw$
        let shift = (data.r2() & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let result = ((data.r1() << shift) & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Sllw$
    }
}

pub struct Srlw;
impl Srlw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srlw$
        let shift = (data.r2() & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let val = data.r1() as u32; // Convert to 32 bits
        let result = (val >> shift) as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Srlw$
    }
}

pub struct Sraw;
impl Sraw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sraw$
        let shift = (data.r2() & 0b11111) as u32; // Only the lower 5 bits are used for shift
        let val = data.r1() as i32; // Convert to 32 bits
        let result = (val >> shift) as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Sraw$
    }
}

pub struct Addiw;
impl Addiw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Addiw$
        let result = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let result = (result & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Addiw$
    }
}

pub struct Slliw;
impl Slliw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Slliw$
        let shift = data.immu_fmt_i() & 0b11111; // Only the lower 5 bits are used for shift
        let result = ((data.r1() << shift) & 0xFFFFFFFF) as u32; // Ensure result is 32 bits
        let result = result as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Slliw$
    }
}

pub struct Srliw;
impl Srliw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srliw$
        let shift = data.immu_fmt_i() & 0b11111; // Only the lower 5 bits are used for shift
        let val = data.r1() as u32; // Convert to 32 bits
        let result = (val >> shift) as i32 as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Srliw$
    }
}

pub struct Sraiw;
impl Sraiw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sraiw$
        let shift = data.immu_fmt_i() & 0b11111; // Only the lower 5 bits are used for shift
        let val = data.r1() as i32; // Convert to 32 bits
        let result = (val >> shift) as i64; // Sign-extend to 64 bits
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Sraiw$
    }
}

pub struct Lwu;
impl Lwu {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lwu$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i()) as usize;
        let vals = data.mem_range(addr, 4)?;
        let val = u32::from_le_bytes([vals[0], vals[1], vals[2], vals[3]]) as u64;
        data.set_rd(val);
        Ok(())
        // $IMPL_END Lwu$
    }
}

pub struct Ld;
impl Ld {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Ld$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i()) as usize;
        let vals = data.mem_range(addr, 8)?;
        let val = u64::from_le_bytes([
            vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7],
        ]);
        data.set_rd(val);
        Ok(())
        // $IMPL_END Ld$
    }
}

pub struct Sd;
impl Sd {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sd$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_s()) as usize;
        let bytes = data.r2().to_le_bytes();
        data.set_mem_range(addr, &bytes)?;
        Ok(())
        // $IMPL_END Sd$
    }
}
