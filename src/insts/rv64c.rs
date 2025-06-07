// $GEN_VERSION 3
// This auto-generated file provides instruction set implementations.
// While you can customize the behavior, developers are strictly advised to
// modify only the `run` method in each instruction.
// Changes outside of these methods are not checked for compatibility and may be lost upon regeneration.
// Critical '$' comments must remain unaltered.
// Remember to back up this file frequently, as `gen_inst.py` can overwrite it.
use crate::*;

pub struct CJr;
impl CJr {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CJr$
        todo!("implement CJr please!");
        Ok(())
        // $IMPL_END CJr$
    }
}

pub struct CMv;
impl CMv {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CMv$
        todo!("implement CMv please!");
        Ok(())
        // $IMPL_END CMv$
    }
}

pub struct CEbreak;
impl CEbreak {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CEbreak$
        todo!("implement CEbreak please!");
        Ok(())
        // $IMPL_END CEbreak$
    }
}

pub struct CJalr;
impl CJalr {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CJalr$
        todo!("implement CJalr please!");
        Ok(())
        // $IMPL_END CJalr$
    }
}

pub struct CAdd;
impl CAdd {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAdd$
        todo!("implement CAdd please!");
        Ok(())
        // $IMPL_END CAdd$
    }
}

pub struct CNop;
impl CNop {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CNop$
        Ok(())
        // $IMPL_END CNop$
    }
}

pub struct CAddi;
impl CAddi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAddi$
        let val = data
            .x(data.rd())
            .wrapping_add_signed(data.imm_12t5_6_2t4_0());
        debug_assert!(val != 0);
        data.set_rd(val);
        Ok(())
        // $IMPL_END CAddi$
    }
}

pub struct CAddiw;
impl CAddiw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAddiw$
        let val =
            data.x(data.rd())
                .wrapping_add_signed(data.imm_12t5_6_2t4_0()) as Word as SWord as SArch;
        debug_assert!(val != 0);
        data.set_rd(val as UArch);
        Ok(())
        // $IMPL_END CAddiw$
    }
}

pub struct CLi;
impl CLi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CLi$
        let val = data.imm_12t5_6_2t4_0();
        debug_assert!(val != 0);
        data.set_rd(val as UArch);
        Ok(())
        // $IMPL_END CLi$
    }
}

pub struct CAddi16sp;
impl CAddi16sp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAddi16sp$
        let uimm9 = data.immu(12, 12) << 9;
        let uimm8_7 = data.immu(4, 3) << 7;
        let uimm6 = data.immu(5, 5) << 6;
        let uimm5 = data.immu(2, 2) << 5;
        let uimm4 = data.immu(6, 6) << 4;
        let uimm = uimm9 | uimm8_7 | uimm6 | uimm5 | uimm4;
        let imm = sext(uimm, 9);
        debug_assert!(imm != 0);

        data.set_x(2, data.x(2).wrapping_add_signed(imm));
        Ok(())
        // $IMPL_END CAddi16sp$
    }
}

pub struct CLui;
impl CLui {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CLui$
        let uimm17 = data.immu(12, 12) << 17;
        let uimm16_12 = data.immu(6, 2) << 12;
        let uimm = uimm17 | uimm16_12;
        let imm = sext(uimm, 17);
        debug_assert!(imm != 0);

        data.set_rd(imm as UArch);
        Ok(())
        // $IMPL_END CLui$
    }
}

pub struct CSrli;
impl CSrli {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSrli$
        let val = (data.x(data.c_rs_p97()) as UArch) >> data.uimm_12t5_6_2t4_0();
        data.set_x(data.c_rs_p97(), val);
        Ok(())
        // $IMPL_END CSrli$
    }
}

pub struct CSrai;
impl CSrai {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSrai$
        let val = (data.x(data.c_rs_p97()) as SArch) >> data.uimm_12t5_6_2t4_0();
        data.set_x(data.c_rs_p97(), val as UArch);
        Ok(())
        // $IMPL_END CSrai$
    }
}

pub struct CAndi;
impl CAndi {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAndi$
        let val = data.imm_12t5_6_2t4_0() as UArch;
        let res = data.x(data.c_rs_p97()) & val;
        data.set_x(data.c_rs_p97(), res);
        Ok(())
        // $IMPL_END CAndi$
    }
}

pub struct CSlli;
impl CSlli {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSlli$
        let shamt = data.uimm_12t5_6_2t4_0();
        let res = data.x(data.c_rs_p97()) << shamt;
        data.set_rd(res);
        Ok(())
        // $IMPL_END CSlli$
    }
}

pub struct CFldsp;
impl CFldsp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CFldsp$
        let addr = data.x(2).wrapping_add(data.uimm_12t5_6_5t4_3_4_2t8_6());
        let bytes = data.mem_range(addr, 8)?;
        let value = DFP::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        data.set_frd(value);
        Ok(())
        // $IMPL_END CFldsp$
    }
}

pub struct CLwsp;
impl CLwsp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CLwsp$
        let addr = data.x(2).wrapping_add(data.uimm_12t5_6_4t4_2_3_2t7_6());
        let bytes = data.mem_range(addr, 4)?;
        let value = Word::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as SWord as SArch;
        data.set_rd(value as UArch);
        Ok(())
        // $IMPL_END CLwsp$
    }
}

pub struct CLdsp;
impl CLdsp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CLdsp$
        let addr = data.x(2).wrapping_add(data.uimm_12t5_6_5t4_3_4_2t8_6());
        let bytes = data.mem_range(addr, 8)?;
        let value = Dword::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        data.set_rd(value as UArch);
        Ok(())
        // $IMPL_END CLdsp$
    }
}

pub struct CFsdsp;
impl CFsdsp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CFsdsp$
        todo!("implement CFsdsp please!");
        Ok(())
        // $IMPL_END CFsdsp$
    }
}

pub struct CSwsp;
impl CSwsp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSwsp$
        todo!("implement CSwsp please!");
        Ok(())
        // $IMPL_END CSwsp$
    }
}

pub struct CSdsp;
impl CSdsp {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSdsp$
        todo!("implement CSdsp please!");
        Ok(())
        // $IMPL_END CSdsp$
    }
}

pub struct CAddi4spn;
impl CAddi4spn {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAddi4spn$
        let nzuimm9_6 = data.immu(10, 7) << 6;
        let nzuimm5_4 = data.immu(12, 11) << 4;
        let nzuimm3 = data.immu(5, 5) << 3;
        let nzuimm2 = data.immu(6, 6) << 2;
        let nzuimm = nzuimm9_6 | nzuimm5_4 | nzuimm3 | nzuimm2;
        debug_assert!(nzuimm != 0);

        let rd = data.immu(4, 2) as Byte + 8;
        data.set_x(rd, data.x(2).wrapping_add(nzuimm));
        Ok(())
        // $IMPL_END CAddi4spn$
    }
}

pub struct CFld;
impl CFld {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CFld$
        let addr = data
            .x(data.c_rs_p97())
            .wrapping_add(data.uimm_12_10t5_3_6_5t7_6());
        let bytes = data.mem_range(addr, 8)?;
        let value = DFP::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        data.set_f(data.c_rs_p42(), value);
        Ok(())
        // $IMPL_END CFld$
    }
}

pub struct CLw;
impl CLw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CLw$
        let addr = data
            .x(data.c_rs_p97())
            .wrapping_add(data.uimm_12_10t5_3_6t2_5t6());
        let bytes = data.mem_range(addr, 4)?;
        let value = Word::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        data.set_x(data.c_rs_p42(), value as UArch);
        Ok(())
        // $IMPL_END CLw$
    }
}

pub struct CLd;
impl CLd {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CLd$
        let addr = data
            .x(data.c_rs_p97())
            .wrapping_add(data.uimm_12_10t5_3_6_5t7_6());
        let bytes = data.mem_range(addr, 8)?;
        let value = Dword::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        data.set_x(data.c_rs_p42(), value as UArch);
        Ok(())
        // $IMPL_END CLd$
    }
}

pub struct CFsd;
impl CFsd {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CFsd$
        let addr = data
            .x(data.c_rs_p97())
            .wrapping_add(data.uimm_12_10t5_3_6_5t7_6());
        let val = data.f(data.c_rs_p42()) as DFP;
        data.set_mem_range(addr, &val.to_le_bytes())?;
        Ok(())
        // $IMPL_END CFsd$
    }
}

pub struct CSw;
impl CSw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSw$
        let addr = data
            .x(data.c_rs_p97())
            .wrapping_add(data.uimm_12_10t5_3_6t2_5t6());
        let val = data.x(data.c_rs_p42()) as Word;
        data.set_mem_range(addr, &val.to_le_bytes())?;
        Ok(())
        // $IMPL_END CSw$
    }
}

pub struct CSd;
impl CSd {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSd$
        let addr = data
            .x(data.c_rs_p97())
            .wrapping_add(data.uimm_12_10t5_3_6_5t7_6());
        let val = data.x(data.c_rs_p42()) as Dword;
        data.set_mem_range(addr, &val.to_le_bytes())?;
        Ok(())
        // $IMPL_END CSd$
    }
}

pub struct CSub;
impl CSub {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSub$
        let v1 = data.x(data.c_rs_p97());
        let v2 = data.x(data.c_rs_p42());
        data.set_x(data.c_rs_p97(), v1.wrapping_sub(v2));
        Ok(())
        // $IMPL_END CSub$
    }
}

pub struct CXor;
impl CXor {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CXor$
        let v1 = data.x(data.c_rs_p97());
        let v2 = data.x(data.c_rs_p42());
        data.set_x(data.c_rs_p97(), v1 ^ v2);
        Ok(())
        // $IMPL_END CXor$
    }
}

pub struct COr;
impl COr {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START COr$
        let v1 = data.x(data.c_rs_p97());
        let v2 = data.x(data.c_rs_p42());
        data.set_x(data.c_rs_p97(), v1 | v2);
        Ok(())
        // $IMPL_END COr$
    }
}

pub struct CAnd;
impl CAnd {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAnd$
        let v1 = data.x(data.c_rs_p97());
        let v2 = data.x(data.c_rs_p42());
        data.set_x(data.c_rs_p97(), v1 & v2);
        Ok(())
        // $IMPL_END CAnd$
    }
}

pub struct CSubw;
impl CSubw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CSubw$
        let v1 = data.x(data.c_rs_p97());
        let v2 = data.x(data.c_rs_p42());
        let res = v1.wrapping_sub(v2) as Word as SWord as SArch;
        data.set_x(data.c_rs_p97(), res as UArch);
        Ok(())
        // $IMPL_END CSubw$
    }
}

pub struct CAddw;
impl CAddw {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CAddw$
        let v1 = data.x(data.c_rs_p97());
        let v2 = data.x(data.c_rs_p42());
        let res = v1.wrapping_add(v2) as Word as SWord as SArch;
        data.set_x(data.c_rs_p97(), res as UArch);
        Ok(())
        // $IMPL_END CAddw$
    }
}

pub struct CBeqz;
impl CBeqz {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CBeqz$
        if data.x(data.c_rs_p97()) == 0 {
            data.vm.jump_pc_rel(data.c_br_offset(), true)?;
        }
        Ok(())
        // $IMPL_END CBeqz$
    }
}

pub struct CBnez;
impl CBnez {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CBnez$
        if data.x(data.c_rs_p97()) != 0 {
            data.vm.jump_pc_rel(data.c_br_offset(), true)?;
        }
        Ok(())
        // $IMPL_END CBnez$
    }
}

pub struct CJ;
impl CJ {
    #[inline]
    pub fn run(mut data: RunData) -> Result<(), VMRunError> {
        // $IMPL_START CJ$
        let uimm11 = data.immu(12, 12) << 11;
        let uimm10 = data.immu(8, 8) << 10;
        let uimm9_8 = data.immu(10, 9) << 8;
        let uimm7 = data.immu(6, 6) << 7;
        let uimm6 = data.immu(7, 7) << 6;
        let uimm5 = data.immu(2, 2) << 5;
        let uimm4 = data.immu(11, 11) << 4;
        let uimm3_1 = data.immu(5, 3) << 1;
        let uimm = uimm11 | uimm10 | uimm9_8 | uimm7 | uimm6 | uimm5 | uimm4 | uimm3_1;
        let imm = sext(uimm, 11);

        data.vm.jump_pc_rel(imm, true)?;
        Ok(())
        // $IMPL_END CJ$
    }
}
