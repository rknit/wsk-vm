mod bits;
mod cache;
mod exception;
mod format;
mod insts;
pub mod repl;
mod report;
mod syscall;
mod vm;

use exception::*;
use format::*;
use insts::*;
pub use report::*;
pub use vm::*;

// common integer types used across the project
pub type Byte = u8;
pub type SByte = i8;
pub type Half = u16;
pub type SHalf = i16;
pub type Word = u32;
pub type SWord = i32;
pub type Dword = u64;
pub type SDword = i64;
pub type Qword = u128;
pub type SQword = i128;

pub type SFP = f32;
pub type DFP = f64;

// architecture-specific integer types of the VM
pub type UArch = Dword;
pub type SArch = SDword;
pub type UDArch = Qword;
pub type SDArch = SQword;

// architecture-specific integer types of the host machine
pub type UHSize = usize;
pub type SHSize = isize;
