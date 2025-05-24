use std::{
    fs,
    io::{Read, Write},
    os::fd::FromRawFd,
};

use log::warn;
use util::syscalls;

use crate::{VM, VMRunError, VMRunErrorKind};

const SYSCALL_REG: usize = 17;
const RET_REG: usize = 10;
const ARG0_REG: usize = 10;
const ARG1_REG: usize = 11;
const ARG2_REG: usize = 12;
#[allow(dead_code)]
const ARG3_REG: usize = 13;
#[allow(dead_code)]
const ARG4_REG: usize = 14;
#[allow(dead_code)]
const ARG5_REG: usize = 15;

syscalls!(
    (vm),
    exit(93) = {
        vm.halt = true;
        vm.exit_code = vm.x(ARG0_REG) as u8;
    },
    write(64) = {
        let fd = vm.x(ARG0_REG) as i32;
        let str_ptr = vm.x(ARG1_REG) as usize;
        let str_len = vm.x(ARG2_REG) as usize;

        let str_slice = vm.mem_range(str_ptr, str_len)?;
        let buf = match str::from_utf8(str_slice) {
            Ok(v) => v,
            Err(e) => {
                warn!("{}: failed to read string: {e}", vm.pc);
                vm.set_x(RET_REG, -1i64 as u64);
                return Ok(());
            }
        };

        let mut f = unsafe { fs::File::from_raw_fd(fd) };
        if let Err(e) = write!(&mut f, "{buf}") {
            warn!("{}: failed to write string: {e}", vm.pc);
            vm.set_x(RET_REG, -1i64 as u64);
            return Ok(());
        };

        vm.set_x(RET_REG, str_len as u64);
    },
    read(63) = {
        let fd = vm.x(ARG0_REG) as i32;
        let buf_ptr = vm.x(ARG1_REG) as usize;
        let rd_len = vm.x(ARG2_REG) as usize;

        let buf_slice = vm.mem_range_mut(buf_ptr, rd_len)?;
        let mut f = unsafe { fs::File::from_raw_fd(fd) };
        let actual_rd_len = match f.read(buf_slice) {
            Ok(v) => v,
            Err(e) => {
                warn!("{}: failed to read data: {e}", vm.pc);
                vm.set_x(RET_REG, -1i64 as u64);
                return Ok(());
            }
        };

        vm.set_x(RET_REG, actual_rd_len as u64);
    },
);

mod util {
    #[macro_export]
    macro_rules! syscalls {
        (($vm:ident), $( $name:ident($code:literal) = $body:block),* $(,)?) => {
            impl VM {
                pub(crate) fn syscall(&mut self) -> Result<(), VMRunError> {
                    let $vm = self;
                    $vm.rep.is_syscall = true;

                    let code = $vm.x(SYSCALL_REG) as i16;
                    match code {
                        $(
                        $code => {
                            $vm.rep.syscall_name = stringify!($name);
                            $body
                        }
                        )*
                        _ => {
                            return Err(VMRunError {
                                pc: $vm.rep.pc,
                                kind: VMRunErrorKind::UnknownSyscall(code),
                                info: Default::default(),
                            });
                        }
                    }

                    Ok(())
                }
            }
        };
    }

    pub use syscalls;
}
