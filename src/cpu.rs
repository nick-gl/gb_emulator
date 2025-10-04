use std::collections::hash_map::Values;
use crate::instruction::{Instruction, ADDHLTarget, ArithmeticTarget, IncTarget, PrefixTarget, JumpTest,LoadType,LoadByteTarget,LoadByteSource};
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;
struct CPU {
    register: Register,
    pc: u16,
    sp:u16,
    bus: MemoryBus,
}
struct MemoryBus {
    memory: [u8; 0xFFFF],
}
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}
struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}
impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCb;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte,prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
        self.pc = next_pc;
    }
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.register.a,
                            LoadByteSource::D8 => self.bus.read_byte(self.pc+1),
                            LoadByteSource::HLI => self.bus.read_byte(self.register.get_hl()),
                            _ => { panic!("TODO: implement other sources") }
                        };
                        match target {
                            LoadByteTarget::A => self.register.a = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.register.get_hl(), source_value),
                            _ => { panic!("TODO: implement other targets") }
                        };
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    _ => { panic!("TODO: implement other load types") }
                }
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.register.f.zero,
                    JumpTest::NotCarry => !self.register.f.carry,
                    JumpTest::Zero => self.register.f.zero,
                    JumpTest::Carry => self.register.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
            Instruction::SWAP(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.swap(self.register.a);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.swap(self.register.b);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.swap(self.register.c);
                        self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.swap(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.swap(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.swap(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.swap(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.swap(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }

                }

            }
            Instruction::SLA(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.sla(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.sla(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.sla(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.sla(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.sla(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.sla(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.sla(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.sla(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }
                }

            }
            Instruction::SRA(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.sra(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.sra(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.sra(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.sra(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.sra(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.sra(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.sra(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.sra(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }
                }

            }
            Instruction::RLC(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rlc(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rlc(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rlc(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rlc(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rlc(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rlc(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rlc(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rlc(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }
                }

            }
            Instruction::RRC(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rrc(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rrc(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rrc(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rrc(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rrc(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rrc(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rrc(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rrc(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }
                }

            }
            Instruction::RL(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rl(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rl(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rl(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rl(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rl(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rl(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rl(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rl(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }

                }
            }
            Instruction::RR(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rr(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rr(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rr(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rr(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rr(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rr(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rr(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rr(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SRL(target) => {
                match target {
                    PrefixTarget::A => {
                         self.register.a = self.srl(self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.srl(self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.srl(self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.srl(self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.srl(self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.srl(self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.srl(self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.srl(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SET(target,bit) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.set(bit,self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.set(bit,self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.set(bit,self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.set(bit,self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.set(bit,self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.set(bit,self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.set(bit,self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.set(bit, value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }

                }
            }
            Instruction::BIT(target,bit) => {
                match target {
                    PrefixTarget::A => {
                        self.bit(self.register.a,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.bit(self.register.b,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.bit(self.register.c,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.bit(self.register.d,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.bit(self.register.e,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.bit(self.register.h,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.bit(self.register.l,bit);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let value = self.register.get_hl();
                        let new = self.bus.read_byte(value);
                        self.bit(bit,new);
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::RESET(target,bit) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.reset(bit,self.register.a);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::B => {
                        self.register.b = self.reset(bit,self.register.b);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::C => {
                        self.register.c = self.reset(bit,self.register.c);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::D => {
                        self.register.d = self.reset(bit,self.register.d);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::E => {
                        self.register.e = self.reset(bit,self.register.e);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::H => {
                        self.register.h = self.reset(bit,self.register.h);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::L => {
                        self.register.l = self.reset(bit,self.register.l);self.pc.wrapping_add(1)
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.reset(bit, value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);self.pc.wrapping_add(1)
                    }

                }
            }
            Instruction::RRA() => {
                self.rra();self.pc.wrapping_add(1)
            }
            Instruction::SCF() => { self.scf();self.pc.wrapping_add(1)
            }
            Instruction:: CCF() => {
                self.ccf();self.pc.wrapping_add(1)
            }
            Instruction::CPL() => {
                    self.cpl();self.pc.wrapping_add(1)
                }
            Instruction::DEC(target) => {
                match target {
                    IncTarget::A => {
                        self.register.a = self.dec8(self.register.a);self.pc.wrapping_add(1)
                    }
                    IncTarget::B => {
                        self.register.b = self.dec8(self.register.b);self.pc.wrapping_add(1)
                    }
                    IncTarget::C => {
                        self.register.c = self.dec8(self.register.c);self.pc.wrapping_add(1)
                    }
                    IncTarget::D => {
                        self.register.d = self.dec8(self.register.d);self.pc.wrapping_add(1)
                    }
                    IncTarget::E => {
                        self.register.e = self.dec8(self.register.e);self.pc.wrapping_add(1)
                    }
                    IncTarget::H => {
                        self.register.h = self.dec8(self.register.h);self.pc.wrapping_add(1)
                    }
                    IncTarget::L => {
                        self.register.l = self.dec8(self.register.l);self.pc.wrapping_add(1)
                    }
                    IncTarget::BC => {
                        let copy = self.dec16(self.register.get_bc());
                        self.register.set_bc(copy);self.pc.wrapping_add(1)
                    }
                    IncTarget::HL => {
                        let copy = self.dec16(self.register.get_hl());
                        self.register.set_hl(copy);self.pc.wrapping_add(1)
                    }
                    IncTarget::DE => {
                        let copy = self.dec16(self.register.get_de());
                        self.register.set_de(copy);self.pc.wrapping_add(1)
                    }
                    IncTarget::SP => {
                        let copy = self.dec16(self.register.get_sp());
                        self.register.set_sp(copy);self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::INC(target) => {
                match target {
                    IncTarget::A => {
                        self.register.a = self.inc8(self.register.a);self.pc.wrapping_add(1)
                    }
                    IncTarget::B => {
                        self.register.b = self.inc8(self.register.b);self.pc.wrapping_add(1)
                    }
                    IncTarget::C => {
                        self.register.c = self.inc8(self.register.c);self.pc.wrapping_add(1)
                    }
                    IncTarget::D => {
                        self.register.d = self.inc8(self.register.d);self.pc.wrapping_add(1)
                    }
                    IncTarget::E => {
                        self.register.e = self.inc8(self.register.e);self.pc.wrapping_add(1)
                    }
                    IncTarget::H => {
                        self.register.h = self.inc8(self.register.h);self.pc.wrapping_add(1)
                    }
                    IncTarget::L => {
                        self.register.l = self.inc8(self.register.l);self.pc.wrapping_add(1)
                    }
                    IncTarget::BC => {
                        let copy = self.inc16(self.register.get_bc());
                        self.register.set_bc(copy);self.pc.wrapping_add(1)
                    }
                    IncTarget::HL => {
                        let copy = self.inc16(self.register.get_hl());
                        self.register.set_hl(copy);self.pc.wrapping_add(1)
                    }
                    IncTarget::DE => {
                        let copy = self.inc16(self.register.get_de());
                        self.register.set_de(copy);self.pc.wrapping_add(1)
                    }
                    IncTarget::SP => {
                        // TODO this looks wrong
                        let copy = self.inc16(self.register.get_bc());
                        self.register.set_bc(copy);self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.or(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                }

            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.and(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.adc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.sub(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A=> {
                        let value = self.register.a;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.add(value);
                        self.register.a = new_value;self.pc.wrapping_add(1)
                    }

                }

            }
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.sub(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.sub(value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.sub(value);self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.sub(value);self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.sub(value);self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.sub(value);self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.sub(value);self.pc.wrapping_add(1)
                    }
                }
            }

            _=> {
                self.pc.wrapping_add(1)
            }
        }
    }
    fn add_hl(&mut self, value: u16) {
        let hl = self.register.get_hl();
        let sum = hl.wrapping_add(value);

        self.register.f.subtract = false;
        self.register.f.half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
        self.register.f.carry = (hl as u32 + value as u32) > 0xFFFF;

        self.register.set_hl(sum);
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.register.a & value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = true;
        new_value
    }
    fn jump(&self, jump: bool) -> u16 {
        if jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }
    fn ccf(&mut self) {
        self.register.f.carry = !self.register.f.carry;
    }
    fn scf(&mut self) {
        self.register.f.carry = true;
    }
    fn rra(&mut self) {
        let carry_in = if self.register.f.carry { 0x80 } else { 0x00 };
        let bit0 = self.register.a & 0x01;

        self.register.a = (self.register.a >> 1) | carry_in;

        // flags
        self.register.f.zero = self.register.a == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit0 == 1;
    }
    fn rr(&mut self, value: u8) -> u8 {
        let carry_in = if self.register.f.carry { 0x80 } else { 0x00 };
        let bit0 =  value & 0x01;

        let new_value = (value >> 1) | carry_in;

        // flags
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit0 == 1;
        new_value
    }
    fn rla(&mut self) {
        let carry_in = if self.register.f.carry { 0x01 } else { 0x00 };
        let bit7 = (self.register.a & 0x80) != 0;

        self.register.a = (self.register.a << 1) | carry_in;

        // flags
        self.register.f.zero = self.register.a == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit7;
    }
    fn rl(&mut self, value: u8) -> u8 {
        let carry_in = if self.register.f.carry { 0x01 } else { 0x00 };
        let bit7 = (value & 0x80) != 0;

        let new_value = (value << 1) | carry_in;

        // flags
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit7;
        new_value
    }
    fn rrca(&mut self) {
        let bit0 = self.register.a & 0x01;
        self.register.f.carry = bit0 != 0;
        self.register.a = (self.register.a >> 1) | (bit0 << 7) ;
        // flags
        self.register.f.zero = self.register.a == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
    }
    fn rrc(&mut self, value: u8) -> u8 {
        let bit0 = value & 0x01;
        self.register.f.carry = bit0 != 0;
        let new_value = (value >> 1) | (bit0 << 7) ;
        // flags
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        new_value
    }
    fn rrla(&mut self) {
        let bit7 = self.register.a & 0x80;
        self.register.f.carry = bit7 != 0;
        self.register.a = (self.register.a << 1) | (bit7 >> 7);

        // flags
        self.register.f.zero = self.register.a == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
    }
    fn rlc(&mut self, value: u8) -> u8 {
        let bit7 = value & 0x80;
        self.register.f.carry = bit7 != 0;
        let new_value = (value << 1) | (bit7 >> 7);

        // flags
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        new_value
    }
    fn cpl(&mut self) {
        self.register.a = !self.register.a;
        self.register.f.subtract = true;
        self.register.f.half_carry = true;
    }
    fn bit(&mut self, bit: u8, value: u8)  {
        let bit_set = (value & (1 << bit)) != 0;

        self.register.f.zero = !bit_set;
        self.register.f.subtract = false;
        self.register.f.half_carry = true;
    }
    fn reset(&mut self, bit: u8, value: u8) -> u8 {
         value & !(1 << bit)
    }
    fn set(&mut self, bit: u8, value: u8) -> u8 {
        value | (1 << bit)
    }
    fn srl(&mut self, values: u8) -> u8 {
        let bit0 = values & 0x01;
        let new_value = (values >> 1);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = bit0 != 0;
        self.register.f.half_carry = false;
        new_value
    }
    fn sra(&mut self, values: u8) -> u8 {
        let bit0 = values & 0x01;
        let bit8 = values & 0x80;
        let mut new_value = (values >> 1);
        new_value =  new_value | bit8;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = bit0 != 0;
        self.register.f.half_carry = false;
        new_value
    }
    fn sla(&mut self, values: u8) -> u8 {
        let bit7 = values & 0x80;
        let new_value = values << 1;

        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit7 != 0;

        new_value
    }
    fn dec16(&mut self, values: u16) -> u16 {
        values.wrapping_sub(1)
    }
    fn inc16(&mut self, values: u16) -> u16 {
        values.wrapping_add(1)
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.register.a | value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        new_value
    }
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.register.a ^ value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = false;
        new_value
    }
    fn dec8(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);

        self.register.f.zero = new_value == 0;
        self.register.f.subtract = true;
        self.register.f.half_carry =0 == (value & 0xF);
        new_value
    }
    fn inc8(&mut self, value: u8) -> u8 {
            let new_value = value.wrapping_add(1);

            self.register.f.zero = new_value == 0;
            self.register.f.subtract = false;
            self.register.f.half_carry = (value & 0xF) + 1 > 0xF;
            new_value
        }
    fn adc(&mut self, value: u8) -> u8 {
        let carry =  if self.register.f.carry { 1 } else  { 0 };
        let value_to_add = {
            value + carry
        };
        let(new_value, did_overflow) = self.register.a.overflowing_add(value_to_add);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = did_overflow;
        self.register.f.half_carry = (self.register.a & 0xF) + ((value + carry) & 0xF) > 0xF;
        new_value
    }
    fn sbc(&mut self, value: u8) -> u8 {
        let carry =  if self.register.f.carry { 1 } else  { 0 };
        let value_to_sub = value + carry;
        let (new_value, did_overflow) = self.register.a.overflowing_sub(value_to_sub);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = true;
        self.register.f.carry = did_overflow;
        self.register.f.half_carry = (self.register.a & 0xF)  < ((value + carry) & 0xF);
        new_value
    }
    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.register.a.overflowing_sub(value);
            self.register.f.zero = new_value == 0;
            self.register.f.subtract = true;
            self.register.f.carry = did_overflow;
            self.register.f.half_carry = (self.register.a & 0xF) < (value & 0xF) ;
            new_value
    }
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.register.a.overflowing_add(value);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = did_overflow;

        self.register.f.half_carry = (self.register.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    fn swap(&mut self, value: u8) -> u8 {
        let new_value = (value << 4) | (value >> 4);

        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = false;

        new_value
    }

}
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}
impl std::convert::From<u8> for FlagsRegister {
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
impl Register {
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value >> 8) & 0xFF) as u8;
        self.l = (value & 0xFF) as u8;
    }
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 |
            self.c as u16
    }
    fn set_bc(&mut self, value: u16) {
        self.b = ((value >> 8) & 0xFF) as u8;
        self.c = (value & 0xFF) as u8;
    }
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 |
            self.e as u16
    }
    fn set_de(&mut self, value: u16) {
        self.d = ((value >> 8) & 0xFF) as u8;
        self.e = (value & 0xFF) as u8;
    }
    //TODO
    fn get_sp(&self) -> u16 {
        12
    }
    fn set_sp(&mut self, value: u16) {

    }
}
