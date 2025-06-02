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
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mul$
        data.set_rd(data.r1s().wrapping_mul(data.r2s()) as u64);
        Ok(())
        // $IMPL_END Mul$
    }
}

pub struct Mulh;
impl Mulh {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulh$
        let r1 = data.r1s() as i128;
        let r2 = data.r2s() as i128;
        let result = ((r1 * r2) >> 64) as i64;
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Mulh$
    }
}

pub struct Mulhsu;
impl Mulhsu {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulhsu$
        let r1 = data.r1s() as i128;
        let r2 = data.r2() as i128;
        let result = ((r1 * r2) >> 64) as i64;
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Mulhsu$
    }
}

pub struct Mulhu;
impl Mulhu {
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START Mulhu$
        let r1 = data.r1() as u128;
        let r2 = data.r2() as u128;
        let result = ((r1 * r2) >> 64) as u64;
        data.set_rd(result);
        Ok(())
        // $IMPL_END Mulhu$
    }
}

pub struct Div;
impl Div {
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
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Div$
    }
}

pub struct Divu;
impl Divu {
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
        data.set_rd(result as u64);
        Ok(())
        // $IMPL_END Rem$
    }
}

pub struct Remu;
impl Remu {
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
