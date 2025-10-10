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
            0x00 => Some(Instruction::NOP()),
            0x01 => Some(Instruction::LD(LoadType::Word(WordByteTarget::BC, WordByteSource::U16))),
            0x03 => Some(Instruction::INC(IncTarget::BC)),
            0x04 => Some(Instruction::INC(IncTarget::B)),
            0x05 => Some(Instruction::DEC(IncTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instruction::RRLA()),
            0x08 => Some(Instruction::LD(LoadType::Word(WordByteTarget::U16, WordByteSource::U16))),
            0x09 => Some(Instruction::AddHL(ADDHLTarget::BC)),
            0x0A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::BC))),
            0x0B => Some(Instruction::DEC(IncTarget::BC)),
            0x0C => Some(Instruction::INC(IncTarget::C)),
            0x0D => Some(Instruction::DEC(IncTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            0x14 => Some(Instruction::INC(IncTarget::D)),
            0x15 => Some(Instruction::DEC(IncTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instruction::RLA()),
            0x18 => None, // TODO JR
            0x19 => Some(Instruction::AddHL(ADDHLTarget::DE)),
            0x1A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::DE))),
            0x1B => Some(Instruction::DEC(IncTarget::DE)),
            0x1C => Some(Instruction::INC(IncTarget::E)),
            0x1D => Some(Instruction::DEC(IncTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instruction::RRA()),
            0x20 => None, //Todo JR
            0x21 => Some(Instruction::LD(LoadType::Word(WordByteTarget::HL, WordByteSource::U16))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(IndirectFromA::HLPlus))),
            0x23 => Some(Instruction::INC(IncTarget::HL)),
            0x24 => Some(Instruction::INC(IncTarget::H)),
            0x25 => Some(Instruction::DEC(IncTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => None, // TODO DAA
            0x28 => None, // TODO JR
            0x29 => Some(Instruction::AddHL(ADDHLTarget::HL)),
            0x2A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::HLPlus))),
            0x2B => Some(Instruction::DEC(IncTarget::HL)),
            0x2C => Some(Instruction::INC(IncTarget::L)),
            0x2D => Some(Instruction::DEC(IncTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL()),
            0x30 => None, // TODO JR
            0x31 => Some(Instruction::LD(LoadType::Word(WordByteTarget::SP, WordByteSource::U16))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(IndirectFromA::HLMinus))),
            0x33 => Some(Instruction::INC(IncTarget::SP)),
            0x34 => Some(Instruction::INC(IncTarget::HL)),
            0x35 => Some(Instruction::DEC(IncTarget::HL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF()),
            0x38 => None, // TODO JR
            0x39 => Some(Instruction::AddHL(ADDHLTarget::SP)),
            0x3A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::HLMinus))),
            0x3B => Some(Instruction::DEC(IncTarget::SP)),
            0x3C => Some(Instruction::INC(IncTarget::A)),
            0x3D => Some(Instruction::DEC(IncTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instruction::CCF()),
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))), // I f'd up the opcode so now It's slightly out of order
            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),
            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))),
            0x76 => Some(Instruction::HALT()),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
            0xAF => Some(Instruction::OR(ArithmeticTarget::A)),
            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
            0xC0 => None, // TODO RT
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => None, // TODO JP
            0xC3 => None, // TODO JP
            0xC4 => None, // TODO CALL
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::PC)),
            0xC7 => None, // TODO RST
            0xC8 => None, // TODO REt
            0xC9 => None, // TODO RET
            0xCA => None, // TODO JP
            0xCB => None,
            0xCC => None, // TODO CAll
            0xCD => None, // TODO call u16
            0xCE => Some(Instruction::ADC(ArithmeticTarget::PC)),
            0xCF => None, // TODO RST
            0xD0 => None, // TODO ret
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xD2 => None, // TODO JP
            0xD3 => None,
            0xD4 => None, // TODO CALL
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::PC)),
            0xD7 => None, // TODO RST
            0xD8 => None, // TODO Ret
            0xD9 => None, // TODORETI
            0xDA => None, // TODO JP
            0xDB => None,
            0xDC => None, // TODO CAll
            0xDD => None,
            0xDE => Some(Instruction::SBC(ArithmeticTarget::PC)),
            0xDF => None, // TODO RST
            0xE0 => Some(Instruction::LD(LoadType::ByteAddressFromA(ByteAddressFromA::FF00U8))),
            0xE1 => Some(Instruction::POP(StackTarget::Hl)),
            0xE2 => Some(Instruction::LD(LoadType::ByteAddressFromA(ByteAddressFromA::FFOOC))),
            0xE3 => None,
            0xE4 => None,
            0xE5 => Some(Instruction::PUSH(StackTarget::Hl)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::PC)),
            0xE7 => None, // TODO RST
            0xE8 => Some(Instruction::ADDSP()),
            0xE9 => None, // TODO JP HL
            0xEA => Some(Instruction::LD(LoadType::ByteAddressFromA(ByteAddressFromA::U16))),
            0xEB => None,
            0xEC => None,
            0xED => None,
            0xEE => Some(Instruction::XOR(ArithmeticTarget::PC)),
            0xEF => None, // TODO RST
            0xF0 => Some(Instruction::LD(LoadType::AFromByteAddress(AFromByteAddress::FF00U8))),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::AFromByteAddress(AFromByteAddress::FFOOC))),
            0xF3 => None, // TODO DI
            0xF4 => None,
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::PC)),
            0xF7 => None, //TODO RST
            0xF8 => Some(Instruction::LDHL()),
            0xF9 => Some(Instruction::LDSP()),
            0xFA => Some(Instruction::LD(LoadType::AFromByteAddress(AFromByteAddress::U16))),
            0xFB => None, // TODO EI
            0xFC => None,
            0xFD => None,
            0xFE => Some(Instruction::CP(ArithmeticTarget::PC)),
            0xFF => None, // TODO RST
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        use PrefixTarget::*;
        match byte {
            // 0x00–0x07: RLC r
            0x00 => Some(Instruction::RLC(B)),
            0x01 => Some(Instruction::RLC(C)),
            0x02 => Some(Instruction::RLC(D)),
            0x03 => Some(Instruction::RLC(E)),
            0x04 => Some(Instruction::RLC(H)),
            0x05 => Some(Instruction::RLC(L)),
            0x06 => Some(Instruction::RLC(HL)),
            0x07 => Some(Instruction::RLC(A)),

            // 0x08–0x0F: RRC r
            0x08 => Some(Instruction::RRC(B)),
            0x09 => Some(Instruction::RRC(C)),
            0x0A => Some(Instruction::RRC(D)),
            0x0B => Some(Instruction::RRC(E)),
            0x0C => Some(Instruction::RRC(H)),
            0x0D => Some(Instruction::RRC(L)),
            0x0E => Some(Instruction::RRC(HL)),
            0x0F => Some(Instruction::RRC(A)),

            // 0x10–0x17: RL r
            0x10 => Some(Instruction::RL(B)),
            0x11 => Some(Instruction::RL(C)),
            0x12 => Some(Instruction::RL(D)),
            0x13 => Some(Instruction::RL(E)),
            0x14 => Some(Instruction::RL(H)),
            0x15 => Some(Instruction::RL(L)),
            0x16 => Some(Instruction::RL(HL)),
            0x17 => Some(Instruction::RL(A)),

            // 0x18–0x1F: RR r
            0x18 => Some(Instruction::RR(B)),
            0x19 => Some(Instruction::RR(C)),
            0x1A => Some(Instruction::RR(D)),
            0x1B => Some(Instruction::RR(E)),
            0x1C => Some(Instruction::RR(H)),
            0x1D => Some(Instruction::RR(L)),
            0x1E => Some(Instruction::RR(HL)),
            0x1F => Some(Instruction::RR(A)),

            // 0x20–0x27: SLA r
            0x20 => Some(Instruction::SLA(B)),
            0x21 => Some(Instruction::SLA(C)),
            0x22 => Some(Instruction::SLA(D)),
            0x23 => Some(Instruction::SLA(E)),
            0x24 => Some(Instruction::SLA(H)),
            0x25 => Some(Instruction::SLA(L)),
            0x26 => Some(Instruction::SLA(HL)),
            0x27 => Some(Instruction::SLA(A)),

            // 0x28–0x2F: SRA r
            0x28 => Some(Instruction::SRA(B)),
            0x29 => Some(Instruction::SRA(C)),
            0x2A => Some(Instruction::SRA(D)),
            0x2B => Some(Instruction::SRA(E)),
            0x2C => Some(Instruction::SRA(H)),
            0x2D => Some(Instruction::SRA(L)),
            0x2E => Some(Instruction::SRA(HL)),
            0x2F => Some(Instruction::SRA(A)),

            // 0x30–0x37: SWAP r
            0x30 => Some(Instruction::SWAP(B)),
            0x31 => Some(Instruction::SWAP(C)),
            0x32 => Some(Instruction::SWAP(D)),
            0x33 => Some(Instruction::SWAP(E)),
            0x34 => Some(Instruction::SWAP(H)),
            0x35 => Some(Instruction::SWAP(L)),
            0x36 => Some(Instruction::SWAP(HL)),
            0x37 => Some(Instruction::SWAP(A)),

            // 0x38–0x3F: SRL r
            0x38 => Some(Instruction::SRL(B)),
            0x39 => Some(Instruction::SRL(C)),
            0x3A => Some(Instruction::SRL(D)),
            0x3B => Some(Instruction::SRL(E)),
            0x3C => Some(Instruction::SRL(H)),
            0x3D => Some(Instruction::SRL(L)),
            0x3E => Some(Instruction::SRL(HL)),
            0x3F => Some(Instruction::SRL(A)),

            // 0x40–0x7F: BIT <target>, <bit>
            0x40..=0x7F => {
                let bit = (byte >> 3) & 0b111;
                let reg = byte & 0b111;
                Some(Instruction::BIT(Self::decode_prefix_target(reg), bit))
            }

            // 0x80–0xBF: RESET <target>, <bit>
            0x80..=0xBF => {
                let bit = (byte >> 3) & 0b111;
                let reg = byte & 0b111;
                Some(Instruction::RESET(Self::decode_prefix_target(reg), bit))
            }

            // 0xC0–0xFF: SET <target>, <bit>
            0xC0..=0xFF => {
                let bit = (byte >> 3) & 0b111;
                let reg = byte & 0b111;
                Some(Instruction::SET(Self::decode_prefix_target(reg), bit))
            }

            _ => None,
        }
    }

    fn decode_prefix_target(code: u8) -> PrefixTarget {
        match code {
            0 => PrefixTarget::B,
            1 => PrefixTarget::C,
            2 => PrefixTarget::D,
            3 => PrefixTarget::E,
            4 => PrefixTarget::H,
            5 => PrefixTarget::L,
            6 => PrefixTarget::HL,
            7 => PrefixTarget::A,
            _ => unreachable!(),
        }
    }
}

pub enum Instruction {
    LDSP(),
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget),
    XOR(ArithmeticTarget),
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
    ADDSP(),
    LDHL(),
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
    LD(LoadType),
    POP(StackTarget),
    PUSH(StackTarget),
    NOP(),
    HALT(),
}
pub enum StackTarget {
    AF,
    BC,
    DE,
    Hl
}
//finish TODO
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    PC
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
pub enum WordByteTarget {
    //just like the Byte type except with 16-bit values
    BC,DE,HL,SP,U16
}
pub enum WordByteSource {
    //just like the Byte type except with 16-bit values
    U16,SP
}
pub enum AFromIndirect {
    //load the A register with the contents from a value from a memory location whose address is stored in some location
    BC,HLPlus,HLMinus,DE,
}
pub enum IndirectFromA {
    BC,HLPlus,HLMinus,DE,
}
pub enum AFromByteAddress {
    U16,
    FF00U8,
    FFOOC
    // Just like AFromIndirect except the memory address is some address in the very last byte of memory
}
pub enum ByteAddressFromA {
    //Just like IndirectFromA except the memory address is some address in the very last byte of memory.
    U16,
    FF00U8,
    FFOOC
}
pub enum LoadByteTarget {
    A,B,C,D,E,H,L,HLI
}
pub enum LoadByteSource {
    A,B,C,D,E,H,L,D8,HLI,
}
pub enum LoadType {
    Byte(LoadByteTarget,LoadByteSource),
    Word(WordByteTarget,WordByteSource),
    ByteAddressFromA(ByteAddressFromA),
    AFromByteAddress(AFromByteAddress),
    IndirectFromA(IndirectFromA),
    AFromIndirect(AFromIndirect),
}

