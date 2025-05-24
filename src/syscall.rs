use util::syscalls;

use crate::{VM, VMRunError, VMRunErrorKind};

pub const SYSCALL_REG: usize = 17;

syscalls! {
    (vm),
    exit: 93 = {
        vm.halt = true;
        vm.exit_code = vm.x(10) as u8;
    },
}

mod util {
    #[macro_export]
    macro_rules! syscalls {
        (($vm:ident), $( $name:ident: $code:literal = $body:block ),* $(,)?) => {
            impl VM {
                pub(crate) fn syscall(&mut self) -> Result<(), VMRunError> {
                    let $vm = self;
                    $vm.rep.is_syscall = true;

                    let code = $vm.x(SYSCALL_REG) as u8;
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
                                info: "ecall syscall",
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
