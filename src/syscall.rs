use std::io;

use log::warn;
use util::syscalls;

use crate::{VM, VMRunError, VMRunErrorKind};

syscalls!(
    (vm),
    exit(93) = {
        vm.halt = true;
        vm.exit_code = vm.x(ARG0_REG) as u8;
    },
    write(64) = {
        let fd = vm.x(ARG0_REG) as i32;
        let buf_ptr = vm.x(ARG1_REG) as usize;
        let buf_len = vm.x(ARG2_REG) as usize;

        let c_ptr = vm.mem_range(buf_ptr, buf_len)?.as_ptr() as *const _;
        let wr_len = unsafe { libc::write(fd, c_ptr, buf_len) };
        if wr_len < 0 {
            let errno = io::Error::last_os_error().raw_os_error().unwrap();
            warn!(
                "{:x}: errno {errno:x}: failed to write {buf_len} bytes from {buf_ptr:x} to fd({fd})",
                vm.pc
            );
        }

        vm.set_x(RET_REG, wr_len as u64);
    },
    read(63) = {
        let fd = vm.x(ARG0_REG) as i32;
        let buf_ptr = vm.x(ARG1_REG) as usize;
        let buf_len = vm.x(ARG2_REG) as usize;

        let c_ptr = vm.mem_range_mut(buf_ptr, buf_len)?.as_ptr() as *mut _;
        let rd_len = unsafe { libc::read(fd, c_ptr, buf_len) };
        if rd_len < 0 {
            let errno = io::Error::last_os_error().raw_os_error().unwrap();
            warn!(
                "{:x}: errno {errno:x}: failed to read {buf_len} bytes from fd({fd}) to {buf_ptr:x}",
                vm.pc
            );
        }

        vm.set_x(RET_REG, rd_len as u64);
    },
    fstat(80) = {
        vm.set_x(RET_REG, 0); // passthrough

        // let fd = vm.x(ARG0_REG) as i32;
        // let statbuf_ptr = vm.x(ARG1_REG) as usize;

        // let c_ptr = vm
        //     .mem_range_mut(statbuf_ptr, size_of::<libc::stat>())?
        //     .as_ptr() as *mut libc::stat;
        // let r = unsafe { libc::fstat(fd, c_ptr) };
        // if r != 0 {
        //     warn!("{}: failed to query fstat to {statbuf_ptr:x}", vm.pc);
        // }
        //
        // vm.set_x(RET_REG, r as u64);
    },
    brk(214) = {
        vm.set_x(RET_REG, 0); // passthrough
    },
    close(57) = {
        let fd = vm.x(ARG0_REG) as i32;
        let r = unsafe { libc::close(fd) };
        if r < 0 {
            let errno = io::Error::last_os_error().raw_os_error().unwrap();
            warn!("{:x}: errno {errno:x}: failed to close fd({fd})", vm.pc);
        }

        vm.set_x(RET_REG, r as u64);
    },
    lseek(62) = {
        let fd = vm.x(ARG0_REG) as i32;
        let offset = vm.x(ARG1_REG) as i64;
        let whence = vm.x(ARG2_REG) as i32;

        let r = unsafe { libc::lseek(fd, offset, whence) };
        if r < 0 {
            let errno = io::Error::last_os_error().raw_os_error().unwrap();
            warn!(
                "{:x}: errno {errno:x}: failed to lseek fd({fd}) on offset {offset:x} with whence {whence}",
                vm.pc
            );
        }

        vm.set_x(RET_REG, r as u64);
    },
    gettimeofday(169) = {
        let tv = vm.x(ARG0_REG) as usize;
        let tz = vm.x(ARG1_REG) as usize;

        let tv_ptr =
            vm.mem_range_mut(tv, size_of::<libc::timeval>())?.as_ptr() as *mut libc::timeval;
        let tz_ptr =
            vm.mem_range_mut(tz, size_of::<libc::timezone>())?.as_ptr() as *mut libc::timezone;
        let r = unsafe { libc::gettimeofday(tv_ptr, tz_ptr) };
        if r < 0 {
            let errno = io::Error::last_os_error().raw_os_error().unwrap();
            warn!(
                "{:x}: errno {errno:x}: failed to query time of day to tv({tv:x}) and tz({tz:x})",
                vm.pc
            );
        }

        vm.set_x(RET_REG, r as u64);
    },
);

const SYSCALL_REG: u8 = 17;
#[allow(dead_code)]
const RET_REG: u8 = 10;
#[allow(dead_code)]
const ARG0_REG: u8 = 10;
#[allow(dead_code)]
const ARG1_REG: u8 = 11;
#[allow(dead_code)]
const ARG2_REG: u8 = 12;
#[allow(dead_code)]
const ARG3_REG: u8 = 13;
#[allow(dead_code)]
const ARG4_REG: u8 = 14;
#[allow(dead_code)]
const ARG5_REG: u8 = 15;

mod util {
    #[macro_export]
    macro_rules! syscalls {
        (($vm:ident), $( $name:ident($code:literal) = $body:block),* $(,)?) => {
            impl VM {
                pub(crate) fn syscall(&mut self) -> Result<(), VMRunError> {
                    let $vm = self;
                    // $vm.rep.is_syscall = true;

                    let code = $vm.x(SYSCALL_REG) as i16;
                    match code {
                        $(
                        $code => {
                            // $vm.rep.syscall_name = stringify!($name);
                            $body
                        }
                        )*
                        _ => {
                            return Err(VMRunError {
                                err_addr: $vm.pc,
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
