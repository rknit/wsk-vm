// $GEN_VERSION 3
// This auto-generated file provides instruction set implementations.
// While you can customize the behavior, developers are strictly advised to
// modify only the `run` method in each instruction.
// Changes outside of these methods are not checked for compatibility and may be lost upon regeneration.
// Critical '$' comments must remain unaltered.
// Remember to back up this file frequently, as `gen_inst.py` can overwrite it.
use crate::*;

pub struct Mulw;
impl Mulw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulw$
        let result = data.r1().wrapping_mul(data.r2()) as SWord as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Mulw$
    }
}

pub struct Divw;
impl Divw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Divw$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let r1 = data.r1s() as SWord;
        let r2 = data.r2s() as SWord;
        let result = r1.wrapping_div(r2) as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Divw$
    }
}

pub struct Divuw;
impl Divuw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Divuw$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let r1 = data.r1() as Word;
        let r2 = data.r2() as Word;
        let result = r1.wrapping_div(r2) as SWord as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Divuw$
    }
}

pub struct Remw;
impl Remw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Remw$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let r1 = data.r1s() as SWord;
        let r2 = data.r2s() as SWord;
        let result = r1.wrapping_rem(r2) as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Remw$
    }
}

pub struct Remuw;
impl Remuw {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Remuw$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let r1 = data.r1s() as Word;
        let r2 = data.r2s() as Word;
        let result = r1.wrapping_rem(r2) as SWord as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Remuw$
    }
}
