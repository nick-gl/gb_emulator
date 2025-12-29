use std::fmt::{UpperHex, Formatter, Result};
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;
impl From<FlagsRegister> for u8 {
     fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
            (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
            (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
            (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}
impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }

}

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}
impl Register {
    pub fn new() -> Register {
        Register {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: FlagsRegister::new(),
            h: 0x01,
            l: 0x4D,
        }
    }
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
#[derive(Clone, Copy)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}
pub struct Register {
    pub(crate) a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub(crate) f: FlagsRegister,
    pub  h: u8,
    pub l: u8,
}
impl PartialEq<u8> for FlagsRegister {
    fn eq(&self, other: &u8) -> bool {
        u8::from(*self) == *other
    }
}
impl UpperHex for FlagsRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let value: u8 = (*self).into(); // convert your FlagsRegister to u8
        write!(f, "{:X}", value)
    }
}