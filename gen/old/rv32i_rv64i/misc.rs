use crate::{exception::Exception, inst_misc};

inst_misc!(
    (vm),
    ecall = {
        vm.raise(Exception::EnvCall)?;
    },
);
