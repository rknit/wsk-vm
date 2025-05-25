#![allow(dead_code)]

mod exception;
mod insts;
pub mod repl;
mod syscall;
mod vm;

use exception::*;
use insts::*;
pub use vm::*;
