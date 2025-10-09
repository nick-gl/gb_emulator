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
            0x01 => Some(Instruction::LD(LoadType::Word(WordByteTarget::BC,WordByteSource::U16))),
            0x03 => Some(Instruction::INC(IncTarget::BC)),
            0x04 => Some(Instruction::INC(IncTarget::B)),
            0x05 => Some(Instruction::DEC(IncTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::D8))),
            0x07 => Some(Instruction::RRLA()),
            0x08 => Some(Instruction::LD(LoadType::Word(WordByteTarget::U16,WordByteSource::U16))),
            0x09 => Some(Instruction::AddHL(ADDHLTarget::BC)),
            0x0A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::BC))),
            0x0B => Some(Instruction::DEC(IncTarget::BC)),
            0x0C => Some(Instruction::INC(IncTarget::C)),
            0x0D => Some(Instruction::DEC(IncTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::D8))),
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            0x14 => Some(Instruction::INC(IncTarget::D)),
            0x15 => Some(Instruction::DEC(IncTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::D8))),
            0x17 => Some(Instruction::RLA()),
            0x18 => None, // TODO JR
            0x19 => Some(Instruction::AddHL(ADDHLTarget::DE)),
            0x1A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::DE))),
            0x1B => Some(Instruction::DEC(IncTarget::DE)),
            0x1C => Some(Instruction::INC(IncTarget::E)),
            0x1D => Some(Instruction::DEC(IncTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::D8))),
            0x1F => Some(Instruction::RRA()),
            0x20 => None,//Todo JR
            0x21 => Some(Instruction::LD(LoadType::Word(WordByteTarget::HL,WordByteSource::U16))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(IndirectFromA::HLPlus))),
            0x23 => Some(Instruction::INC(IncTarget::HL)),
            0x24 => Some(Instruction::INC(IncTarget::H)),
            0x25 => Some(Instruction::DEC(IncTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::D8))),
            0x27 => None, // TODO DAA
            0x28 => None, // TODO JR
            0x29 => Some(Instruction::AddHL(ADDHLTarget::HL)),
            0x2A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::HLPlus))),
            0x2B => Some(Instruction::DEC(IncTarget::HL)),
            0x2C => Some(Instruction::INC(IncTarget::L)),
            0x2D => Some(Instruction::DEC(IncTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL()),
            0x30 => None,// TODO JR
            0x31 => Some(Instruction::LD(LoadType::Word(WordByteTarget::SP,WordByteSource::U16))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(IndirectFromA::HLMinus))),
            0x33 => Some(Instruction::INC(IncTarget::SP)),
            0x34 => Some(Instruction::INC(IncTarget::HL)),
            0x35 => Some(Instruction::DEC(IncTarget::HL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF()),
            0x38 => None, // TODO JR
            0x39 => Some(Instruction::AddHL(ADDHLTarget::SP)),
            0x3A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirect::HLMinus))),
            0x3B => Some(Instruction::DEC(IncTarget::SP)),
            0x3C => Some(Instruction::INC(IncTarget::A)),
            0x3D => Some(Instruction::DEC(IncTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D8))),
            0x3F => Some(Instruction::CCF()),
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::B))),
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
            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::HLI))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::HLI))),
            0x5F =>  Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::A))),
            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::HLI))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::HLI))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::A))),
            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::L))),
            0x76 =>  Some(Instruction::HALT()),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::HLI))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::A))),
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
            0xC6 => Some(Instruction::ADD(StackTarget::U8)),
            0xC7 => None, // TODO
            0xC8 => None, // TODO
            0xC9 => None, // TODO
            0xCA => None, // TODO
            0xCB => None, // TODO (prefixed opcodes handled elsewhere)
            0xCC => None, // TODO
            0xCD => None, // TODO
            0xCE => None, // TODO
            0xCF => None, // TODO
            0xD0 => None, // TODO
            0xD1 => None, // TODO
            0xD2 => None, // TODO
            0xD3 => None, // TODO (unused)
            0xD4 => None, // TODO
            0xD5 => None, // TODO
            0xD6 => None, // TODO
            0xD7 => None, // TODO
            0xD8 => None, // TODO
            0xD9 => None, // TODO
            0xDA => None, // TODO
            0xDB => None, // TODO (unused)
            0xDC => None, // TODO
            0xDD => None, // TODO (unused)
            0xDE => None, // TODO
            0xDF => None, // TODO
            0xE0 => None, // TODO
            0xE1 => None, // TODO
            0xE2 => None, // TODO
            0xE3 => None, // TODO (unused)
            0xE4 => None, // TODO (unused)
            0xE5 => None, // TODO
            0xE6 => None, // TODO
            0xE7 => None, // TODO
            0xE8 => None, // TODO
            0xE9 => None, // TODO
            0xEA => None, // TODO
            0xEB => None, // TODO (unused)
            0xEC => None, // TODO (unused)
            0xED => None, // TODO (unused)
            0xEE => None, // TODO
            0xEF => None, // TODO
            0xF0 => None, // TODO
            0xF1 => None, // TODO
            0xF2 => None, // TODO
            0xF3 => None, // TODO
            0xF4 => None, // TODO (unused)
            0xF5 => None, // TODO
            0xF6 => None, // TODO
            0xF7 => None, // TODO
            0xF8 => None, // TODO
            0xF9 => None, // TODO
            0xFA => None, // TODO
            0xFB => None, // TODO
            0xFC => None, // TODO (unused)
            0xFD => None, // TODO (unused)
            0xFE => None, // TODO
            0xFF => None, // TODO
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
            match byte {
                0x00 => Some(Instruction::RLC(PrefixTarget::B)),
                0x01 => None, // TODO RLC C
                0x02 => None, // TODO RLC D
                0x03 => None, // TODO RLC E
                0x04 => None, // TODO RLC H
                0x05 => None, // TODO RLC L
                0x06 => None, // TODO RLC (HL)
                0x07 => None, // TODO RLC A

                // RRC r
                0x08 => None, // TODO RRC B
                0x09 => None, // TODO RRC C
                0x0A => None, // TODO RRC D
                0x0B => None, // TODO RRC E
                0x0C => None, // TODO RRC H
                0x0D => None, // TODO RRC L
                0x0E => None, // TODO RRC (HL)
                0x0F => None, // TODO RRC A

                // RL r
                0x10 => None, // TODO RL B
                0x11 => None, // TODO RL C
                0x12 => None, // TODO RL D
                0x13 => None, // TODO RL E
                0x14 => None, // TODO RL H
                0x15 => None, // TODO RL L
                0x16 => None, // TODO RL (HL)
                0x17 => None, // TODO RL A

                // RR r
                0x18 => None, // TODO RR B
                0x19 => None, // TODO RR C
                0x1A => None, // TODO RR D
                0x1B => None, // TODO RR E
                0x1C => None, // TODO RR H
                0x1D => None, // TODO RR L
                0x1E => None, // TODO RR (HL)
                0x1F => None, // TODO RR A

                // SLA r
                0x20 => None, // TODO SLA B
                0x21 => None, // TODO SLA C
                0x22 => None, // TODO SLA D
                0x23 => None, // TODO SLA E
                0x24 => None, // TODO SLA H
                0x25 => None, // TODO SLA L
                0x26 => None, // TODO SLA (HL)
                0x27 => None, // TODO SLA A

                // SRA r
                0x28 => None, // TODO SRA B
                0x29 => None, // TODO SRA C
                0x2A => None, // TODO SRA D
                0x2B => None, // TODO SRA E
                0x2C => None, // TODO SRA H
                0x2D => None, // TODO SRA L
                0x2E => None, // TODO SRA (HL)
                0x2F => None, // TODO SRA A

                // SWAP r
                0x30 => None, // TODO SWAP B
                0x31 => None, // TODO SWAP C
                0x32 => None, // TODO SWAP D
                0x33 => None, // TODO SWAP E
                0x34 => None, // TODO SWAP H
                0x35 => None, // TODO SWAP L
                0x36 => None, // TODO SWAP (HL)
                0x37 => None, // TODO SWAP A

                // SRL r
                0x38 => None, // TODO SRL B
                0x39 => None, // TODO SRL C
                0x3A => None, // TODO SRL D
                0x3B => None, // TODO SRL E
                0x3C => None, // TODO SRL H
                0x3D => None, // TODO SRL L
                0x3E => None, // TODO SRL (HL)
                0x3F => None, // TODO SRL A

                // BIT b, r (0x40–0x7F)
                0x40 => None, // TODO BIT 0,B
                0x41 => None, // TODO BIT 0,C
                0x42 => None, // TODO BIT 0,D
                0x43 => None, // TODO BIT 0,E
                0x44 => None, // TODO BIT 0,H
                0x45 => None, // TODO BIT 0,L
                0x46 => None, // TODO BIT 0,(HL)
                0x47 => None, // TODO BIT 0,A
                0x48 => None, // TODO BIT 1,B
                0x49 => None, // TODO BIT 1,C
                0x4A => None, // TODO BIT 1,D
                0x4B => None, // TODO BIT 1,E
                0x4C => None, // TODO BIT 1,H
                0x4D => None, // TODO BIT 1,L
                0x4E => None, // TODO BIT 1,(HL)
                0x4F => None, // TODO BIT 1,A
                0x50 => None, // TODO BIT 2,B
                0x51 => None, // TODO BIT 2,C
                0x52 => None, // TODO BIT 2,D
                0x53 => None, // TODO BIT 2,E
                0x54 => None, // TODO BIT 2,H
                0x55 => None, // TODO BIT 2,L
                0x56 => None, // TODO BIT 2,(HL)
                0x57 => None, // TODO BIT 2,A
                0x58 => None, // TODO BIT 3,B
                0x59 => None, // TODO BIT 3,C
                0x5A => None, // TODO BIT 3,D
                0x5B => None, // TODO BIT 3,E
                0x5C => None, // TODO BIT 3,H
                0x5D => None, // TODO BIT 3,L
                0x5E => None, // TODO BIT 3,(HL)
                0x5F => None, // TODO BIT 3,A
                0x60 => None, // TODO BIT 4,B
                0x61 => None, // TODO BIT 4,C
                0x62 => None, // TODO BIT 4,D
                0x63 => None, // TODO BIT 4,E
                0x64 => None, // TODO BIT 4,H
                0x65 => None, // TODO BIT 4,L
                0x66 => None, // TODO BIT 4,(HL)
                0x67 => None, // TODO BIT 4,A
                0x68 => None, // TODO BIT 5,B
                0x69 => None, // TODO BIT 5,C
                0x6A => None, // TODO BIT 5,D
                0x6B => None, // TODO BIT 5,E
                0x6C => None, // TODO BIT 5,H
                0x6D => None, // TODO BIT 5,L
                0x6E => None, // TODO BIT 5,(HL)
                0x6F => None, // TODO BIT 5,A
                0x70 => None, // TODO BIT 6,B
                0x71 => None, // TODO BIT 6,C
                0x72 => None, // TODO BIT 6,D
                0x73 => None, // TODO BIT 6,E
                0x74 => None, // TODO BIT 6,H
                0x75 => None, // TODO BIT 6,L
                0x76 => None, // TODO BIT 6,(HL)
                0x77 => None, // TODO BIT 6,A
                0x78 => None, // TODO BIT 7,B
                0x79 => None, // TODO BIT 7,C
                0x7A => None, // TODO BIT 7,D
                0x7B => None, // TODO BIT 7,E
                0x7C => None, // TODO BIT 7,H
                0x7D => None, // TODO BIT 7,L
                0x7E => None, // TODO BIT 7,(HL)
                0x7F => None, // TODO BIT 7,A

                // RES b, r (0x80–0xBF)
                0x80 => None, // TODO RES 0,B
                0x81 => None, // TODO RES 0,C
                0x82 => None, // TODO RES 0,D
                0x83 => None, // TODO RES 0,E
                0x84 => None, // TODO RES 0,H
                0x85 => None, // TODO RES 0,L
                0x86 => None, // TODO RES 0,(HL)
                0x87 => None, // TODO RES 0,A
                // … continue sequentially …
                0xBF => None, // TODO RES 7,A

                // SET b, r (0xC0–0xFF)
                0xC0 => None, // TODO SET 0,B
                0xC1 => None, // TODO SET 0,C
                // … continue sequentially …
                0xFF => None, // TODO SET 7,A
                _ => None, // TODO add rest of map
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