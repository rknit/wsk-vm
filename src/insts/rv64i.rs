use crate::{VM, VMRunError};

#[derive(Debug, Clone, Copy)]
pub struct Addiw;
impl Addiw {
    pub fn run(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
        todo!("implement Addiw please!");
        Ok(())
    }
}

