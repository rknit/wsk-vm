use crate::{VM, VMRunError};

use super::{bits::sext, format::RawFormat};

pub fn decode(raw: RawFormat) -> Option<(&'static str, Inst)> {
    use RawFormat::*;

    utils::dec!((raw) => {
        match R {
            0b00000 => {
                0b000 => { _ => test }
            }
        },
        match I {
            0b00000 => { _ => test2 }
        },
    })

    // Some(match raw {
    //     utils::dec!(
    //         0b00000 => {
    //             0b000 => { _ => test }
    //         }
    //     ),
    //     I {
    //         opc,
    //         funct3,
    //         rd,
    //         rs1,
    //         imm,
    //     } => todo!(),
    //     S {
    //         opc,
    //         funct3,
    //         rd,
    //         rs1,
    //         rs2,
    //         imm,
    //     } => todo!(),
    //     B {
    //         opc,
    //         funct3,
    //         rs1,
    //         rs2,
    //         imm,
    //     } => todo!(),
    //     U { opc, rd, imm } => todo!(),
    //     J { opc, rd, imm } => todo!(),
    //     Other { opc } => todo!(),
    // })
}

pub fn test(vm: &mut VM, rd: u8, r1: u64, r2: u64) -> Result<(), VMRunError> {
    todo!()
}

pub fn test2(vm: &mut VM, rd: u8, r1: u64, imm: i64) -> Result<(), VMRunError> {
    todo!()
}

pub fn inst_r(
    rd: u8,
    rs1: u8,
    rs2: u8,
    f: impl Fn(&mut VM, u8, u64, u64) -> Result<(), VMRunError>,
) -> Inst {
    move |vm: &mut VM| -> Result<(), VMRunError> {
        let r1 = vm.x(rs1);
        let r2 = vm.x(rs2);
        f(vm, rd, r1, r2)?;
        Ok(())
    }
}

pub fn inst_i(
    rd: u8,
    rs1: u8,
    imm: i16,
    f: impl Fn(&mut VM, u8, u64, i64) -> Result<(), VMRunError>,
) -> Inst {
    move |vm: &mut VM| -> Result<(), VMRunError> {
        let r1 = vm.x(rs1);
        let imm_sext = sext(imm as u64, 11);
        f(vm, rd, r1, imm_sext as i64)?;
        Ok(())
    }
}

pub fn inst_s(
    rd: u8,
    rs1: u8,
    rs2: u8,
    imm: i16,
    f: impl Fn(&mut VM, u8, u64, u64, i64) -> Result<(), VMRunError>,
) -> Inst {
    move |vm: &mut VM| -> Result<(), VMRunError> {
        let r1 = vm.x(rs1);
        let r2 = vm.x(rs2);
        let imm_sext = sext(imm as u64, 11);
        f(vm, rd, r1, r2, imm_sext as i64)?;
        Ok(())
    }
}

pub fn inst_b(
    rs1: u8,
    rs2: u8,
    imm: i16,
    f: impl Fn(&mut VM, u64, u64, i64) -> Result<(), VMRunError>,
) -> Inst {
    move |vm: &mut VM| -> Result<(), VMRunError> {
        let r1 = vm.x(rs1);
        let r2 = vm.x(rs2);
        let imm_sext = sext((imm << 1) as u64, 12);
        f(vm, r1, r2, imm_sext as i64)?;
        Ok(())
    }
}

pub fn inst_u(rd: u8, imm: i32, f: impl Fn(&mut VM, u8, i64) -> Result<(), VMRunError>) -> Inst {
    move |vm: &mut VM| -> Result<(), VMRunError> {
        let imm_shl = sext((imm << 12) as u64, 31);
        f(vm, rd, imm_shl as i64)?;
        Ok(())
    }
}

pub fn inst_j(rd: u8, imm: i32, f: impl Fn(&mut VM, u8, i64) -> Result<(), VMRunError>) -> Inst {
    move |vm: &mut VM| -> Result<(), VMRunError> {
        let imm_sext = sext((imm << 1) as u64, 20);
        f(vm, rd, imm_sext as i64)?;
        Ok(())
    }
}

pub type Inst = *const fn(&mut VM) -> Result<(), VMRunError>;

mod utils {
    macro_rules! dec {
        (
            ($raw:expr) => {
                match R {
                    $($opc:pat => {
                        $($f3:pat => {
                            $($f7:pat => $func:ident),*
                        }),*
                    }),*
                },
                match I {
                    $($opci:pat => {
                        $($f3i:pat => $funci:ident),*
                    }),*
                },
            }
        ) => {
            Some(match $raw {
                R {
                    opc,
                    funct3,
                    funct7,
                    rd,
                    rs1,
                    rs2,
                } => match opc {$(
                    $opc => match funct3 {$(
                        $f3 => match funct7 {$(
                            $f7 => (stringify!($func), inst_r(rd, rs1, rs2, $func)),
                            #[allow(unreachable_patterns)]
                            _ => return None,
                        )*},
                        #[allow(unreachable_patterns)]
                        _ => return None,
                    )*},
                    #[allow(unreachable_patterns)]
                    _ => return None,
                )*},



                I {
                    opc,
                    funct3,
                    rd,
                    rs1,
                    imm,
                } => match opc {$(
                    $opci => match funct3 {$(
                        $f3i => (stringify!($funci), inst_i(rd, rs1, imm, $funci)),
                        #[allow(unreachable_patterns)]
                        _ => return None,
                    )*},
                    #[allow(unreachable_patterns)]
                    _ => return None,
                )*},

                #[allow(unreachable_patterns)]
                _ => return None,
            })
        };
    }

    pub(crate) use dec;
}
