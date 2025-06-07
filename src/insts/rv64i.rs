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
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Addw$
        let result = data.r1().wrapping_add(data.r2());
        let result = (result & 0xFFFFFFFF) as Word; // Ensure result is 32 bits
        let result = result as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Addw$
    }
}

pub struct Subw;
impl Subw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Subw$
        let result = data.r1().wrapping_sub(data.r2());
        let result = (result & 0xFFFFFFFF) as Word; // Ensure result is 32 bits
        let result = result as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Subw$
    }
}

pub struct Sllw;
impl Sllw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sllw$
        let shift = (data.r2() & 0b11111) as Word; // Only the lower 5 bits are used for shift
        let result = ((data.r1() << shift) & 0xFFFFFFFF) as Word; // Ensure result is 32 bits
        let result = result as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Sllw$
    }
}

pub struct Srlw;
impl Srlw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srlw$
        let shift = (data.r2() & 0b11111) as Word; // Only the lower 5 bits are used for shift
        let val = data.r1() as Word; // Convert to 32 bits
        let result = (val >> shift) as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Srlw$
    }
}

pub struct Sraw;
impl Sraw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sraw$
        let shift = (data.r2() & 0b11111) as Word; // Only the lower 5 bits are used for shift
        let val = data.r1() as SWord; // Convert to 32 bits
        let result = (val >> shift) as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Sraw$
    }
}

pub struct Addiw;
impl Addiw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Addiw$
        let result = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let result = (result & 0xFFFFFFFF) as Word; // Ensure result is 32 bits
        let result = result as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Addiw$
    }
}

pub struct Slliw;
impl Slliw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Slliw$
        let shift = data.immu_fmt_i() & 0b11111; // Only the lower 5 bits are used for shift
        let result = ((data.r1() << shift) & 0xFFFFFFFF) as Word; // Ensure result is 32 bits
        let result = result as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Slliw$
    }
}

pub struct Srliw;
impl Srliw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Srliw$
        let shift = data.immu_fmt_i() & 0b11111; // Only the lower 5 bits are used for shift
        let val = data.r1() as Word; // Convert to 32 bits
        let result = (val >> shift) as SWord as SArch; // Sign-extend to architecture bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Srliw$
    }
}

pub struct Sraiw;
impl Sraiw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sraiw$
        let shift = data.immu_fmt_i() & 0b11111; // Only the lower 5 bits are used for shift
        let val = data.r1() as Word; // Convert to 32 bits
        let result = (val >> shift) as SArch; // Sign-extend to architecture64 bits
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Sraiw$
    }
}

pub struct Lwu;
impl Lwu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Lwu$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let vals = data.mem_range(addr, 4)?;
        let val = Word::from_le_bytes([vals[0], vals[1], vals[2], vals[3]]) as UArch;
        data.set_rd(val);
        Ok(())
        // $IMPL_END Lwu$
    }
}

pub struct Ld;
impl Ld {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Ld$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_i());
        let vals = data.mem_range(addr, 8)?;
        let val = Dword::from_le_bytes([
            vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7],
        ]);
        data.set_rd(val as UArch);
        Ok(())
        // $IMPL_END Ld$
    }
}

pub struct Sd;
impl Sd {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Sd$
        let addr = data.r1().wrapping_add_signed(data.imm_fmt_s());
        let bytes = (data.r2() as Dword).to_le_bytes();
        data.set_mem_range(addr, &bytes)?;
        Ok(())
        // $IMPL_END Sd$
    }
}
