impl Instruction {
    pub fn from_byte(byte: u8, prefix: bool) -> Option<Instruction> {
        if prefix {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }
    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => None,//TODO NOP
            0x01 => None, //TODO ld
            0x03 => Some(Instruction::INC(IncTarget::BC)),
            0x04 => Some(Instruction::INC(IncTarget::B)),
            0x05 => Some(Instruction::DEC(IncTarget::B)),
            0x06 => None, //TODO
            0x07 => Some(Instruction::RRLA()),
            0x08 => None, //TODO
            0x09 => Some(Instruction::AddHL(ADDHLTarget::BC)),
            0x0A => None, //TODO
            0x0B => Some(Instruction::DEC(IncTarget::BC)),
            Ox0C => Some(Instruction::INC(IncTarget::C)),
            0x0D => Some(Instruction::DEC(IncTarget::C)),
            0x0E => None, //TODO
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
            match byte {
                0x00 => Some(Instruction::RLC(PrefixTarget::B)),
                _ => None, // TODO add rest of map
            }
        }
    }
}
pub enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget),
    //fix need to find out what to do with sp
    INC(IncTarget),
    DEC(IncTarget),
    AddHL(ADDHLTarget),
    CCF(),
    SCF(),
    RRA(),
    RLA(),
    RRCA(),
    RRLA(),
    CPL(),
    //fix target
    BIT(PrefixTarget, u8),
    RESET(PrefixTarget, u8),
    SET(PrefixTarget, u8),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),
    JP(JumpTest),
}
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}
pub enum PrefixTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL
}
pub enum IncTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    BC,
    DE,
    SP
}
pub enum ADDHLTarget {
    HL,
    BC,
    DE,
    SP
}
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}
