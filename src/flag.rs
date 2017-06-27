use debug::*;
use cpu::Cpu;

pub const CARRY_FLAG: u32 = 0b1;
pub const ZERO_FLAG: u32 = 0b10;
pub const NEGATIVE_FLAG: u32 = 0b100;
pub const OVERFLOW_FLAG: u32 = 0b1000;
pub const PROTECT_FLAG: u32 = 0b10000;
pub const EXTERNAL_FLAG: u32 = 0b100000;

pub const ARITH_FLAGS_MASK: u32 = 0b1111;

const U32_MASK: u64 = 0xFFFF_FFFF;
const LONG_SIGN_BIT: u32 = 0x8000_0000;
const U8_MASK: u32 = 0xFF;
const SHORT_SIGN_BIT: u8 = 0x80;

pub trait Flag {
    fn flag_get(&self, flag_mask: u32) -> bool;
    fn flag_set(&mut self, flag_mask: u32, set: bool);

    fn set_arith_long(&mut self, a: u32, b: u32, c: u64);
    fn set_arith_short(&mut self, a: u8, b: u8, c: u32);
}

impl Flag for Cpu {
    fn flag_get(&self, flag_mask: u32) -> bool {
        self.rflags & flag_mask != 0
    }

    fn flag_set(&mut self, flag_mask: u32, set: bool) {
        if set {
            self.rflags |= flag_mask;
        } else {
            self.rflags &= !flag_mask;
        }
    }

    fn set_arith_long(&mut self, a: u32, b: u32, c: u64) {
        self.flag_set(CARRY_FLAG, c & !U32_MASK != 0);
        self.flag_set(ZERO_FLAG, c & U32_MASK == 0);
        self.flag_set(NEGATIVE_FLAG, c & LONG_SIGN_BIT as u64 != 0);
        let a7 = a & LONG_SIGN_BIT != 0;
        let b7 = b & LONG_SIGN_BIT != 0;
        let c7 = c & LONG_SIGN_BIT as u64 != 0;
        self.flag_set(OVERFLOW_FLAG, (!a7 && !b7 && c7) || (a7 && b7 && !c7));
    }

    fn set_arith_short(&mut self, a: u8, b: u8, c: u32) {
        self.flag_set(CARRY_FLAG, c & !U8_MASK != 0);
        self.flag_set(ZERO_FLAG, c & U8_MASK == 0);
        self.flag_set(NEGATIVE_FLAG, c & SHORT_SIGN_BIT as u32 != 0);
        let a7 = a & SHORT_SIGN_BIT != 0;
        let b7 = b & SHORT_SIGN_BIT != 0;
        let c7 = c & SHORT_SIGN_BIT as u32 != 0;
        self.flag_set(OVERFLOW_FLAG, (!a7 && !b7 && c7) || (a7 && b7 && !c7));
        debug!("Flags: {:b}", self.rflags);
    }
}
