use crate::{VM, VMRunError};

pub const INST_LEN: usize = 4;

pub type Inst = fn(&mut VM) -> Result<(), VMRunError>;

pub fn decode_inst(bytes: [u8; INST_LEN]) -> Inst {
    |_| unimplemented!()
}
