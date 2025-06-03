// #![allow(dead_code)]

mod bits;
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

#[allow(non_camel_case_types)]
pub type byte = u8;
#[allow(non_camel_case_types)]
pub type sbyte = i8;
#[allow(non_camel_case_types)]
pub type half = u16;
#[allow(non_camel_case_types)]
pub type shalf = i16;
#[allow(non_camel_case_types)]
pub type word = u32;
#[allow(non_camel_case_types)]
pub type sword = i32;
#[allow(non_camel_case_types)]
pub type dword = u64;
#[allow(non_camel_case_types)]
pub type sdword = i64;
#[allow(non_camel_case_types)]
pub type qword = u128;
#[allow(non_camel_case_types)]
pub type sqword = i128;

#[allow(non_camel_case_types)]
pub type uarch = u64;
#[allow(non_camel_case_types)]
pub type iarch = i64;
#[allow(non_camel_case_types)]
pub type udarch = u128;
#[allow(non_camel_case_types)]
pub type idarch = i128;

#[allow(non_camel_case_types)]
pub type uhsize = usize;
#[allow(non_camel_case_types)]
pub type ihsize = isize;
