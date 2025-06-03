// $GEN_VERSION 3
// This auto-generated file provides instruction set implementations.
// While you can customize the behavior, developers are strictly advised to
// modify only the `run` method in each instruction.
// Changes outside of these methods are not checked for compatibility and may be lost upon regeneration.
// Critical '$' comments must remain unaltered.
// Remember to back up this file frequently, as `gen_inst.py` can overwrite it.
use crate::*;

pub struct Mul;
impl Mul {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mul$
        data.set_rd(data.r1s().wrapping_mul(data.r2s()) as UArch);
        Ok(())
        // $IMPL_END Mul$
    }
}

pub struct Mulh;
impl Mulh {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulh$
        let r1 = data.r1s() as SDArch;
        let r2 = data.r2s() as SDArch;
        let result = ((r1 * r2) >> 64) as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Mulh$
    }
}

pub struct Mulhsu;
impl Mulhsu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulhsu$
        let r1 = data.r1s() as SDArch;
        let r2 = data.r2() as SDArch;
        let result = ((r1 * r2) >> 64) as SArch;
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Mulhsu$
    }
}

pub struct Mulhu;
impl Mulhu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulhu$
        let r1 = data.r1() as UDArch;
        let r2 = data.r2() as UDArch;
        let result = ((r1 * r2) >> 64) as UArch;
        data.set_rd(result);
        Ok(())
        // $IMPL_END Mulhu$
    }
}

pub struct Div;
impl Div {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Div$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = data.r1s().wrapping_div(data.r2s());
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Div$
    }
}

pub struct Divu;
impl Divu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Divu$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = data.r1().wrapping_div(data.r2());
        data.set_rd(result);
        Ok(())
        // $IMPL_END Divu$
    }
}

pub struct Rem;
impl Rem {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Rem$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = data.r1s().wrapping_rem(data.r2s());
        data.set_rd(result as UArch);
        Ok(())
        // $IMPL_END Rem$
    }
}

pub struct Remu;
impl Remu {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Remu$
        if data.r2() == 0 {
            return Err(VMRunError {
                err_addr: data.vm.pc,
                kind: VMRunErrorKind::DivisionByZero,
                info: "division by zero",
            });
        }
        let result = data.r1().wrapping_rem(data.r2());
        data.set_rd(result);
        Ok(())
        // $IMPL_END Remu$
    }
}
