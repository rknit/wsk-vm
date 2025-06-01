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
    pub fn run(data: DataR) -> Result<(), VMRunError> {
        // $IMPL_START Add$
        todo!("implement Add please!");
        Ok(())
        // $IMPL_END Add$
    }
}

pub struct Addi;
impl Addi {
    pub fn run(data: DataI) -> Result<(), VMRunError> {
        // $IMPL_START Addi$
        todo!("implement Addi please!");
        Ok(())
        // $IMPL_END Addi$
    }
}
