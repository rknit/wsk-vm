use super::bits::*;
use crate::{VM, ext};

pub struct DataR<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}
impl<'vm> DataR<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r2(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

pub struct DataI<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub rs1: u8,
    pub raw_imm: i16,
}
impl<'vm> DataI<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn imm(&self) -> i64 {
        sext(self.raw_imm as u64, 11)
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

pub struct DataS<'vm> {
    pub vm: &'vm mut VM,
    pub rs1: u8,
    pub rs2: u8,
    pub raw_imm: i16,
}
impl<'vm> DataS<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r2(&self) -> u64 {
        self.vm.x(self.rs2)
    }

    pub fn imm(&self) -> i64 {
        sext(self.raw_imm as u64, 11)
    }
}

pub struct DataB<'vm> {
    pub vm: &'vm mut VM,
    pub rs1: u8,
    pub rs2: u8,
    pub raw_imm: i16,
}
impl<'vm> DataB<'vm> {
    pub fn r1(&self) -> u64 {
        self.vm.x(self.rs1)
    }

    pub fn r2(&self) -> u64 {
        self.vm.x(self.rs2)
    }

    pub fn imm(&self) -> i64 {
        sext((self.raw_imm as u64) << 1, 12)
    }
}

pub struct DataU<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub raw_imm: i32,
}
impl<'vm> DataU<'vm> {
    pub fn imm(&self) -> i64 {
        ((self.raw_imm as u64) << 12) as i32 as i64
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

pub struct DataJ<'vm> {
    pub vm: &'vm mut VM,
    pub rd: u8,
    pub raw_imm: i32,
}
impl<'vm> DataJ<'vm> {
    pub fn imm(&self) -> i64 {
        sext((self.raw_imm as u64) << 1, 20)
    }

    pub fn set_rd(&mut self, value: u64) {
        self.vm.set_x(self.rd, value);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RawFormat {
    R {
        opc: u8,
        funct3: u8,
        funct7: u8,
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    I {
        opc: u8,
        funct3: u8,
        rd: u8,
        rs1: u8,
        imm: i16,
    },
    S {
        opc: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i16,
    },
    B {
        opc: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i16,
    },
    U {
        opc: u8,
        rd: u8,
        imm: i32,
    },
    J {
        opc: u8,
        rd: u8,
        imm: i32,
    },
}
impl RawFormat {
    pub fn parse(inst: u32) -> Option<Self> {
        let opc = opcode(inst);
        let funct3 = funct3_14_12(inst);
        let funct7 = funct7_31_25(inst);

        let rd = rd_11_7(inst) as u8;
        let rs1 = rs1_19_15(inst) as u8;
        let rs2 = rs2_24_20(inst) as u8;

        let imm_i = || sext_imm_12(ext!(inst, u64; 31;20)) as i16;
        let imm_s = || {
            let imm_4_0 = rd_11_7(inst) as u64;
            let imm_11_5 = funct7_31_25(inst) as u64;
            let imm = sext_imm_12((imm_11_5 << 5) | imm_4_0);
            imm as i16
        };
        let imm_b = || {
            let imm_4_1 = ext!(inst, u64; 11;8);
            let imm_10_5 = ext!(inst, u64; 30;25);
            let imm_11 = ext!(inst, u64; 7;7);
            let imm_12 = ext!(inst, u64; 31;31);
            let imm = sext_imm_12((imm_12 << 11) | (imm_11 << 10) | (imm_10_5 << 4) | imm_4_1);
            imm as i16
        };
        let imm_u = || {
            let imm_31_12 = ext!(inst, u64; 31;12);
            let imm = sext_imm_20(imm_31_12);
            imm as i32
        };
        let imm_j = || {
            let imm_10_1 = ext!(inst, u64; 30;21);
            let imm_11 = ext!(inst, u64; 20;20);
            let imm_19_12 = ext!(inst, u64; 19;12);
            let imm_20 = ext!(inst, u64; 31;31);
            let imm = sext_imm_20((imm_20 << 19) | (imm_19_12 << 11) | (imm_11 << 10) | imm_10_1);
            imm as i32
        };

        Some(match opc {
            0b00000 | 0b00100 | 0b00110 | 0b11001 | 0b11100 => RawFormat::I {
                opc,
                rd,
                rs1,
                imm: imm_i(),
                funct3,
            },
            0b00101 | 0b01101 => RawFormat::U {
                opc,
                rd,
                imm: imm_u(),
            },
            0b01000 => RawFormat::S {
                opc,
                rs1,
                rs2,
                imm: imm_s(),
                funct3,
            },
            0b01100 | 0b01110 => RawFormat::R {
                opc,
                rd,
                rs1,
                rs2,
                funct3,
                funct7,
            },
            0b11000 => RawFormat::B {
                opc,
                rs1,
                rs2,
                imm: imm_b(),
                funct3,
            },
            0b11011 => RawFormat::J {
                opc,
                rd,
                imm: imm_j(),
            },
            _ => return None,
        })
    }
}
