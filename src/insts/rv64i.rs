// $GEN_VERSION 1
// This file is auto-generated.
// This file contains the implementations of the instruction set,
// which requires developers to implement them themselves.
// Please backup this file regularly, as it can be overwritten by `gen_inst.py`.
use crate::{VM, VMRunError};

// $IMPL Addiw i
#[derive(Debug, Clone, Copy)]
pub struct Addiw;
impl Addiw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Addiw please!");
        Ok(())
    }
}
