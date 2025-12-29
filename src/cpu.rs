use std::cmp::PartialEq;
use std::collections::hash_map::Values;
use crate::instruction::{Instruction, ADDHLTarget, ArithmeticTarget, IncTarget, ByteAddressFromA, AFromByteAddress, IndirectFromA, AFromIndirect, WordByteSource, WordByteTarget, PrefixTarget, JumpTest, LoadType, LoadByteTarget, LoadByteSource, StackTarget};
use crate::bus::{MemoryBus, LCDSTAT_VECTOR, TIMER_VECTOR, VBLANK_VECTOR};
use crate::GPU::gpu::Interrupt;
use crate::Register::Register;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;
pub struct CPU {
    pub register: Register,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub is_halted: bool,
    pub interrupts_enabled: bool,
}
impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU {
        CPU {
            register: Register::new(),
            pc: 0x0100,
            sp: 0xFFFE,
            bus: MemoryBus::new(boot_rom, game_rom),
            is_halted: false,
            interrupts_enabled: true,
        }
    }
    pub fn new_test(test: Vec<u8>) -> CPU {
        CPU {
            register: Register::new(),
            pc: 0x0100,
            sp: 0xFFFE,
            bus: MemoryBus::new_test(test),
            is_halted: false,
            interrupts_enabled: true,
        }
    }
    pub fn step(&mut self) -> u8 {

        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.read_next_byte();
        }

        let (next_pc, mut cycles) =
            if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
                self.execute(instruction)
            } else {
                let description = format!(
                    "0x{}{:x}",
                    if prefixed { "cb" } else { "" },
                    instruction_byte
                );
                panic!(
                    "0x{:x}: Unknown - {}",
                    self.pc, description
                )
            };

        self.bus.step(cycles);

        if self.bus.has_interrupt() {
            self.is_halted = false;
        }
        if !self.is_halted {
            self.pc = next_pc;
        }

        let mut interrupted = false;
        if self.interrupts_enabled {
            if self.bus.interrupt_enable.vblank && self.bus.interrupt_flag.vblank {
                interrupted = true;
                self.bus.interrupt_flag.vblank = false;
                self.interrupt(VBLANK_VECTOR)
            }
            if self.bus.interrupt_enable.lcdstat && self.bus.interrupt_flag.lcdstat {
                interrupted = true;
                self.bus.interrupt_flag.lcdstat = false;
                self.interrupt(LCDSTAT_VECTOR)
            }
            if self.bus.interrupt_enable.timer && self.bus.interrupt_flag.timer {
                interrupted = true;
                self.bus.interrupt_flag.timer = false;
                self.interrupt(TIMER_VECTOR)
            }
        }
        if interrupted {
            cycles += 12;
        }
        cycles
    }

    fn interrupt(&mut self, location: u16) {
        self.interrupts_enabled = false;
        self.push(self.pc);
        self.pc = location;
        self.bus.step(12);
    }
    fn execute(&mut self, instruction: Instruction) -> (u16,u8) {
        if self.is_halted {return (self.pc,4)}
        let mut return_pc= 0;

        match instruction {
            Instruction::NOP() => {
                return_pc = self.pc.wrapping_add(1);
            }
            Instruction::HALT() => {
                self.is_halted = true;

                // If interrupts are enabled and any interrupt is pending, increment PC
                    self.pc = self.pc.wrapping_add(1);
                return_pc = self.pc;
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.register.a,
                            LoadByteSource::D8 => self.bus.read_byte(self.pc+1),
                            LoadByteSource::HLI => self.bus.read_byte(self.register.get_hl()),
                            LoadByteSource::B => self.register.b,
                            LoadByteSource::C => self.register.c,
                            LoadByteSource::L => self.register.l,
                            LoadByteSource::H => self.register.h,
                            LoadByteSource::E => self.register.e,
                            LoadByteSource::D => self.register.d,
                        };
                        match target {
                            LoadByteTarget::A => self.register.a = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.register.get_hl(), source_value),
                            LoadByteTarget::B => self.register.b = source_value,
                            LoadByteTarget::C => self.register.c = source_value,
                            LoadByteTarget::L => self.register.l = source_value,
                            LoadByteTarget::E => self.register.e = source_value,
                            LoadByteTarget::D => self.register.d = source_value,
                            LoadByteTarget::H => self.register.h = source_value,
                        };
                        match source {
                            LoadByteSource::D8 => return_pc = self.pc.wrapping_add(2),
                            _ => return_pc = self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Word(target,source) => {
                        let source_value = match source {
                            WordByteSource::SP => self.sp,
                            WordByteSource::U16 => {let low = self.bus.read_byte(self.pc.wrapping_add(1));
                                let high = self.bus.read_byte(self.pc.wrapping_add(2));
                              ((high as u16) << 8) | (low as u16)}
                        };
                        match target {
                            WordByteTarget::U16 => {let low = self.bus.read_byte(self.pc.wrapping_add(1));
                                let high = self.bus.read_byte( self.pc.wrapping_add(2));
                                let addr = ((high as u16) << 8) | (low as u16);
                                self.bus.write_byte(addr, (source_value & 0xFF) as u8);
                                self.bus.write_byte(addr.wrapping_add(1), (source_value >> 8) as u8);}
                            WordByteTarget::SP => self.sp = source_value,
                            WordByteTarget::BC => self.register.set_bc(source_value),
                            WordByteTarget::DE => self.register.set_de(source_value),
                            WordByteTarget::HL => self.register.set_hl(source_value),
                        }
                        let add = match source {
                            WordByteSource::U16 | WordByteSource::SP => 3,
                            _=> 1
                        };
                         return_pc = self.pc.wrapping_add(add);
                    }
                    LoadType::AFromIndirect(source) => {
                        match source {
                            AFromIndirect::BC => {
                                let addr = self.register.get_bc();
                                self.register.a = self.bus.read_byte(addr);
                            }
                            AFromIndirect::DE => {
                                let addr = self.register.get_de();
                                self.register.a = self.bus.read_byte(addr);
                            }
                            AFromIndirect::HLMinus => {
                                let addr = self.register.get_hl();
                                self.register.a = self.bus.read_byte(addr);
                                self.register.set_hl(addr.wrapping_sub(1));
                            }
                            AFromIndirect::HLPlus => {
                                let addr = self.register.get_hl();
                                self.register.a = self.bus.read_byte(addr);
                                self.register.set_hl(addr.wrapping_add(1));
                            }
                        }

                        // increment the program counter and return it
                        self.pc = self.pc.wrapping_add(1);
                        return_pc = self.pc;
                    }

                    LoadType::IndirectFromA(indirect) => {
                        match indirect {
                            IndirectFromA::HLPlus => {
                                let addr = self.register.get_hl();
                                self.bus.write_byte(addr, self.register.a);
                                self.register.set_hl(addr.wrapping_add(1));
                            }
                            IndirectFromA::HLMinus => {
                                let addr = self.register.get_hl();
                                self.bus.write_byte(addr, self.register.a);
                                self.register.set_hl(addr.wrapping_sub(1));
                            }
                            IndirectFromA::BC => {
                                let addr = self.register.get_bc();
                                self.bus.write_byte(addr, self.register.a);
                            }
                            IndirectFromA::DE => {
                                let addr = self.register.get_de();
                                self.bus.write_byte(addr, self.register.a);
                            }
                        }
                        return_pc = self.pc.wrapping_add(1);
                    }
                    LoadType::AFromByteAddress(target) => {
                        match target {
                            AFromByteAddress::U16 => {
                                let low = self.bus.read_byte(self.pc.wrapping_add(1));
                                let high = self.bus.read_byte(self.pc.wrapping_add(2));
                                let addr = ((high as u16) << 8) | (low as u16);
                                self.register.a = self.bus.read_byte(addr);
                                return_pc = self.pc.wrapping_add(3); // opcode + 16-bit address
                            }
                            AFromByteAddress::FF00U8 => {
                                let offset = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
                                let addr = 0xFF00u16.wrapping_add(offset);
                                self.register.a = self.bus.read_byte(addr);
                                return_pc = self.pc.wrapping_add(2);
                            }
                            AFromByteAddress::FFOOC => {
                                let addr = 0xFF00 + self.register.c as u16;
                                self.register.a = self.bus.read_byte(addr);
                                return_pc = self.pc.wrapping_add(1); // only opcode, no extra byte
                            }
                        }
                    }
                    LoadType::ByteAddressFromA(store_type) => {
                        match store_type {
                            ByteAddressFromA::U16 => {
                                let low = self.bus.read_byte(self.pc.wrapping_add(1));
                                let high = self.bus.read_byte(self.pc.wrapping_add(2));
                                let addr = ((high as u16) << 8) | (low as u16);
                                self.bus.write_byte(addr, self.register.a);
                                return_pc = self.pc.wrapping_add(3);
                            }
                            ByteAddressFromA::FF00U8 => {
                                let offset = self.bus.read_byte(self.pc.wrapping_add(1));
                                let addr = 0xFF00u16 + offset as u16;
                                self.bus.write_byte(addr, self.register.a);
                                return_pc = self.pc.wrapping_add(2);
                            }
                            ByteAddressFromA::FFOOC => {
                                let addr = 0xFF00u16 + self.register.c as u16;
                                self.bus.write_byte(addr, self.register.a);
                                return_pc = self.pc.wrapping_add(1);
                            }
                        }
                    }
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.swap(self.register.a);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.swap(self.register.b);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.swap(self.register.c);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.swap(self.register.d);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.swap(self.register.e);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.swap(self.register.h);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.swap(self.register.l);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.swap(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(1);
                    }

                }

            }
            Instruction::SLA(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.sla(self.register.a);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.sla(self.register.b);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.sla(self.register.c);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.sla(self.register.d);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.sla(self.register.e);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.sla(self.register.h);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.sla(self.register.l);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.sla(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(1);
                    }
                }

            }
            Instruction::SRA(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.sra(self.register.a);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.sra(self.register.b);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.sra(self.register.c);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.sra(self.register.d);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.sra(self.register.e);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.sra(self.register.h);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.sra(self.register.l);return_pc = self.pc.wrapping_add(1);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.sra(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(1);
                    }
                }

            }
            Instruction::RLC(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rlc(self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rlc(self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rlc(self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rlc(self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rlc(self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rlc(self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rlc(self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rlc(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }
                }

            }
            Instruction::RRC(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rrc(self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rrc(self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rrc(self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rrc(self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rrc(self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rrc(self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rrc(self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rrc(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }
                }

            }
            Instruction::RL(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rl(self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rl(self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rl(self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rl(self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rl(self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rl(self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rl(self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rl(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }

                }
            }
            Instruction::RR(target) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.rr(self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.rr(self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.rr(self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.rr(self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.rr(self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.rr(self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.rr(self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.rr(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }
                }
            }
            Instruction::SRL(target) => {
                match target {
                    PrefixTarget::A => {
                         self.register.a = self.srl(self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.srl(self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.srl(self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.srl(self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.srl(self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.srl(self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.srl(self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.srl(value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }
                }
            }
            Instruction::SET(target,bit) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.set(bit,self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.set(bit,self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.set(bit,self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.set(bit,self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.set(bit,self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.set(bit,self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.set(bit,self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.set(bit, value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }

                }
            }
            Instruction::BIT(target,bit) => {
                match target {
                    PrefixTarget::A => {
                        self.bit(self.register.a,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.bit(self.register.b,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.bit(self.register.c,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.bit(self.register.d,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.bit(self.register.e,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.bit(self.register.h,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.bit(self.register.l,bit);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let value = self.register.get_hl();
                        let new = self.bus.read_byte(value);
                        self.bit(bit,new);
                        return_pc = self.pc.wrapping_add(2);
                    }
                }
            }
            Instruction::RESET(target,bit) => {
                match target {
                    PrefixTarget::A => {
                        self.register.a = self.reset(bit,self.register.a);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::B => {
                        self.register.b = self.reset(bit,self.register.b);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::C => {
                        self.register.c = self.reset(bit,self.register.c);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::D => {
                        self.register.d = self.reset(bit,self.register.d);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::E => {
                        self.register.e = self.reset(bit,self.register.e);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::H => {
                        self.register.h = self.reset(bit,self.register.h);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::L => {
                        self.register.l = self.reset(bit,self.register.l);return_pc = self.pc.wrapping_add(2);
                    }
                    PrefixTarget::HL => {
                        let addr = self.register.get_hl();
                        let value = self.bus.read_byte(addr);   // load from memory
                        let new_value = self.reset(bit, value);    // clear bit in that byte
                        self.bus.write_byte(addr, new_value);return_pc = self.pc.wrapping_add(2);
                    }

                }
            }
            Instruction::RRA() => {
                self.rra();return_pc = self.pc.wrapping_add(1);
            }
            Instruction::SCF() => { self.scf();return_pc = self.pc.wrapping_add(1);
            }
            Instruction:: CCF() => {
                self.ccf();return_pc = self.pc.wrapping_add(1);
            }
            Instruction::CPL() => {
                    self.cpl();return_pc = self.pc.wrapping_add(1);
                }
            Instruction::DEC(target) => {
                match target {
                    IncTarget::A => {
                        self.register.a = self.dec8(self.register.a);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::B => {
                        self.register.b = self.dec8(self.register.b);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::C => {
                        self.register.c = self.dec8(self.register.c);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::D => {
                        self.register.d = self.dec8(self.register.d);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::E => {
                        self.register.e = self.dec8(self.register.e);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::H => {
                        self.register.h = self.dec8(self.register.h);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::L => {
                        self.register.l = self.dec8(self.register.l);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::BC => {
                        let addr = self.register.get_bc();
                        self.register.a = self.bus.read_byte(addr);
                        let copy = self.dec16(addr);
                        self.register.set_hl(copy);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::HL => {
                        let addr = self.register.get_hl();
                        self.register.a = self.bus.read_byte(addr);
                        let copy = self.dec16(addr);
                        self.register.set_hl(copy);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::DE => {
                        let copy = self.dec16(self.register.get_de());
                        self.register.set_de(copy);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::SP => {
                        let copy = self.dec16(self.sp);
                        self.sp = copy;return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::HLI => {
                        let hl = self.register.get_hl();
                        let amount = self.bus.read_byte(hl);
                        let result = self.dec8(amount);
                        self.bus.write_byte(hl, result);
                        return_pc = self.pc.wrapping_add(1);

                    }
                }
            }
            Instruction::INC(target) => {
                match target {
                    IncTarget::A => {
                        self.register.a = self.inc8(self.register.a);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::B => {
                        self.register.b = self.inc8(self.register.b);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::C => {
                        self.register.c = self.inc8(self.register.c);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::D => {
                        self.register.d = self.inc8(self.register.d);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::E => {
                        self.register.e = self.inc8(self.register.e);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::H => {
                        self.register.h = self.inc8(self.register.h);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::L => {
                        self.register.l = self.inc8(self.register.l);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::BC => {
                        let copy = self.inc16(self.register.get_bc());
                        self.register.set_bc(copy);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::HL => {
                        let copy = self.inc16(self.register.get_hl());
                        self.register.set_hl(copy);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::DE => {
                        let copy = self.inc16(self.register.get_de());
                        self.register.set_de(copy);return_pc = self.pc.wrapping_add(1);
                    }
                    IncTarget::HLI => {
                        let hl = self.register.get_hl();
                        let amount = self.bus.read_byte(hl);
                        let result = self.inc8(amount);
                        self.bus.write_byte(hl, result);
                        return_pc = self.pc.wrapping_add(1);

                    }
                    IncTarget::SP => {
                        // TODO this looks wrong
                        self.sp = self.inc16(self.sp);
                        return_pc = self.pc.wrapping_add(1);
                    }
                }
            }

            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_hl());
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.or(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                }

            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.xor(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(2);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_hl());
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        self.register.a = self.xor(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(2);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_hl());
                        let new_value = self.or(value);
                        self.register.a = value;
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.and(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(2);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_hl());
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.adc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(2);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_hl());
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(2);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_bc());
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.sbc(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.sub(value);
                        self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::ADD(target) => match target {
                ArithmeticTarget::PC => {
                    let value = self.bus.read_byte(self.pc+1);
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(2);
                }
                ArithmeticTarget::HL => {
                    self.register.a = self.add(self.bus.read_byte(self.register.get_hl()));
                    return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::A=> {
                    let value = self.register.a;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::B => {
                    let value = self.register.b;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::C => {
                    let value = self.register.c;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::D => {
                    let value = self.register.d;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::E => {
                    let value = self.register.e;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::H => {
                    let value = self.register.h;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }
                ArithmeticTarget::L => {
                    let value = self.register.l;
                    let new_value = self.add(value);
                    self.register.a = new_value;return_pc = self.pc.wrapping_add(1);
                }

            },
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::PC => {
                        let value = self.bus.read_byte(self.pc+1);
                        let new_value = self.sub(value);
                        return_pc = self.pc.wrapping_add(2);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.bus.read_byte(self.register.get_hl());
                        self.sub(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::A => {
                        let value = self.register.a;
                        let new_value = self.sub(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::B => {
                        let value = self.register.b;
                        let new_value = self.sub(value);
                        return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::C => {
                        let value = self.register.c;
                        let new_value = self.sub(value);return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        let value = self.register.d;
                        let new_value = self.sub(value);return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::E => {
                        let value = self.register.e;
                        let new_value = self.sub(value);return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::H => {
                        let value = self.register.h;
                        let new_value = self.sub(value);return_pc = self.pc.wrapping_add(1);
                    }
                    ArithmeticTarget::L => {
                        let value = self.register.l;
                        let new_value = self.sub(value);return_pc = self.pc.wrapping_add(1);
                    }
                }
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.register.set_bc(result),
                    StackTarget::DE => self.register.set_de(result),
                    StackTarget::Hl => self.register.set_hl(result),
                    StackTarget::AF => self.register.set_af(result),

                };
                return_pc = self.pc.wrapping_add(1);
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.register.get_bc(),
                    StackTarget::DE => self.register.get_de(),
                    StackTarget::Hl => self.register.get_hl(),
                    StackTarget::AF => self.register.get_af(),
                };
                self.push(value);
                return_pc = self.pc.wrapping_add(1);
            }
            Instruction::ADDSP() => {
                let value = self.bus.read_byte(self.pc) as i8 as i16;
                self.pc = self.pc.wrapping_add(1);
                self.pc = self.pc.wrapping_add(1);
                return_pc = self.pc.wrapping_add(1);

                let sp = self.sp;
                let result = sp.wrapping_add(value as u16);

                // Update flags according to Game Boy behavior
                self.register.f.zero = false;
                self.register.f.subtract = false;

                // Half-carry occurs if lower nibble overflowed from bit 3 to 4
                self.register.f.half_carry = ((sp & 0xF) + (value as u16 & 0xF)) > 0xF;

                // Carry occurs if low byte overflowed from bit 7 to 8
                self.register.f.carry = ((sp & 0xFF) + (value as u16 & 0xFF)) > 0xFF;

                // Update SP (writes both lower and upper bytes)
                self.sp = result;
                return_pc = self.pc.wrapping_add(2);
            }
            Instruction::LDHL() => {
                let value = self.bus.read_byte(self.pc+1) as i8 as i16;
                self.pc = self.pc.wrapping_add(1);
                return_pc = self.pc.wrapping_add(1);

                let sp = self.sp;
                let result = sp.wrapping_add(value as u16);

                // Update flags according to Game Boy behavior
                self.register.f.zero = false;
                self.register.f.subtract = false;

                // Half-carry occurs if lower nibble overflowed from bit 3 to 4
                self.register.f.half_carry = ((sp & 0xF) + (value as u16 & 0xF)) > 0xF;

                // Carry occurs if low byte overflowed from bit 7 to 8
                self.register.f.carry = ((sp & 0xFF) + (value as u16 & 0xFF)) > 0xFF;
                self.register.set_hl(result);
                return_pc = self.pc.wrapping_add(1);
            }
     Instruction::LDSP() => {
         self.sp = self.register.get_hl();
         return_pc = self.pc.wrapping_add(1);
     }
            Instruction::CALL(test) => {
                // DESCRIPTION: Conditionally PUSH the would be instruction on to the
                // stack and then jump to a specific address
                // PC:?/+3
                // Cycles: 24/12
                // Z:- N:- H:- C:-
                let jump_condition = match test {
                    JumpTest::NotZero => !self.register.f.zero,
                    JumpTest::NotCarry => !self.register.f.carry,
                    JumpTest::Zero => self.register.f.zero,
                    JumpTest::Carry => self.register.f.carry,
                    JumpTest::Always => true,
                };
                return self.call(jump_condition);
            }
            Instruction::RET(test) => {
                // DESCRIPTION: Conditionally POP two bytes from the stack and jump to that address
                // PC:?/+1
                // WHEN: condition is 'always'
                // Cycles: 16/8
                // ELSE:
                // Cycles: 20/8
                // Z:- N:- H:- C:-
                let jump_condition = match test {
                    JumpTest::NotZero => !self.register.f.zero,
                    JumpTest::NotCarry => !self.register.f.carry,
                    JumpTest::Zero => self.register.f.zero,
                    JumpTest::Carry => self.register.f.carry,
                    JumpTest::Always => true,
                };
                let next_pc = self.return_(jump_condition);

                let cycles = if jump_condition && test == JumpTest::Always {
                    16
                } else if jump_condition {
                    20
                } else {
                    8
                };
                return (next_pc, cycles);
            }
            Instruction::RETI => {
                // PC:?
                // Cycles: 16
                // Z:- N:- H:- C:-
                self.interrupts_enabled = true;
             return   (self.pop(), 16)
            }
            Instruction::RST(loc) => {
                // PC:?
                // Cycles: 24
                // Z:- N:- H:- C:-
                self.rst();
                return (loc.to_hex(), 24)
            }
            Instruction::DI => {
                // PC:+1
                // Cycles: 4
                // Z:- N:- H:- C:-
                self.interrupts_enabled = false;
               return (self.pc.wrapping_add(1), 4)
            }
            Instruction::EI => {
                // PC:+1
                // Cycles: 4
                // Z:- N:- H:- C:-
                self.interrupts_enabled = true;
                return (self.pc.wrapping_add(1), 4)
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.register.f.zero,
                    JumpTest::NotCarry => !self.register.f.carry,
                    JumpTest::Zero => self.register.f.zero,
                    JumpTest::Carry => self.register.f.carry,
                    JumpTest::Always => true
                };
                return self.jump(jump_condition);
            }
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.register.f.zero,
                    JumpTest::NotCarry => !self.register.f.carry,
                    JumpTest::Zero => self.register.f.zero,
                    JumpTest::Carry => self.register.f.carry,
                    JumpTest::Always => true
                };
                return self.jump_relative(jump_condition);
            }
            Instruction::STOP() => {
                self.bus.switch_speed();
                return_pc = self.pc.wrapping_add(1);
            }
            Instruction::AddHL(register) => {
                let value = match register {
                    ADDHLTarget::BC => self.register.get_bc(),
                    ADDHLTarget::DE => self.register.get_de(),
                    ADDHLTarget::HL => self.register.get_hl(),
                    ADDHLTarget::SP => self.sp,
                };
                self.add_hl(value);
                return (self.pc.wrapping_add(1), 8)
            }
            Instruction::JPI => {
              return  (self.register.get_hl(), 4)
            }
            Instruction::DAA => {
                self.daa();
                return (self.pc.wrapping_add(1), 4)
            }
            _=> {
            panic!("Forgot to add {:?}",instruction)
            }
        }
   (return_pc,self.find_cycle(instruction))
    }
    pub(crate) fn find_cycle(&mut self, instruction: Instruction) -> u8 {
        use Instruction::*;

        match instruction {
            // -------- Control / misc --------
            NOP() => 1,
            HALT() => 1,
            DI | EI => 1,
            DAA => 1,
            CPL() | CCF() | SCF() => 1,

            // -------- 8-bit INC / DEC --------
            INC(target) | DEC(target) => match target {
                IncTarget::HL => 3, // (HL)
                IncTarget::BC | IncTarget::DE | IncTarget::SP => 2,
                _ => 1,
            },

            // -------- Loads --------
            LD(load) => match load {
                // LD r, r
                LoadType::Byte(dst, src) => match (dst, src) {
                    // memory involved
                    (LoadByteTarget::HLI, _) | (_, LoadByteSource::HLI) => 2,
                    (_, LoadByteSource::D8) => 2,
                    _ => 1,
                },

                // LD rr, d16
                LoadType::Word(_, WordByteSource::U16) => 3,
                LoadType::Word(_, WordByteSource::SP) => 2,

                // LD (nn),A or LD A,(nn)
                LoadType::ByteAddressFromA(_) |
                LoadType::AFromByteAddress(_) => 3,

                // LD (rr),A or LD A,(rr)
                LoadType::IndirectFromA(_) |
                LoadType::AFromIndirect(_) => 2,
            },

            // -------- 8-bit ALU --------
            ADD(t) | ADC(t) | SUB(t) | SBC(t)
            | AND(t) | OR(t) | XOR(t) | CP(t) => match t {
                ArithmeticTarget::HL => 2,
                ArithmeticTarget::PC => 2, // immediate
                _ => 1,
            },

            // -------- 16-bit ALU --------
            AddHL(_) => 2,
            ADDSP() => 4,
            LDHL() => 3,
            LDSP() => 2,

            // -------- Jumps --------
            JR(_) => 2,   // +1 if taken (handle elsewhere)
            JP(_) => 4,
            JPI => 1,

            CALL(_) => 6, // +1 if conditional taken
            RET(_) => 4,  // +1 if conditional taken
            RETI => 4,
            RST(_) => 4,

            // -------- Stack --------
            PUSH(_) => 4,
            POP(_) => 3,

            // -------- Rotates / shifts --------
            RRA() | RLA() | RRCA() | RRLA() => 1,

            RLC(t) | RRC(t) | RL(t) | RR(t)
            | SLA(t) | SRA(t) | SRL(t) | SWAP(t) => match t {
                PrefixTarget::HL => 4,
                _ => 2,
            },

            BIT(t, _) => match t {
                PrefixTarget::HL => 3,
                _ => 2,
            },

            RESET(t, _) | SET(t, _) => match t {
                PrefixTarget::HL => 4,
                _ => 2,
            },
            STOP() => 1,
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
    fn any_interrupt_pending(&self) -> bool {
        let ie = self.interrupts_enabled as u8;
        let iflags = self.bus.interrupt_flag.to_byte();
        (ie & iflags & 0x1F) != 0
    }
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }
    fn jump(&self, should_jump: bool) -> (u16, u8) {
        if should_jump {
            (self.read_next_word(), 16)
        } else {
            (self.pc.wrapping_add(3), 12)
        }
    }
    fn read_next_word(&self) -> u16 {
        // Gameboy is little endian so read pc + 2 as most significant bit
        // and pc + 1 as least significant bit
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc + 1) as u16)
    }
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    #[inline(always)]
    fn jump_relative(&self, should_jump: bool) -> (u16, u8) {
        let next_step  = self.pc.wrapping_add(2);
        if should_jump {
            let offset = self.read_next_byte() as i8;
            let pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };
            (pc, 16)
        } else {
            (next_step, 12)
        }
    }

    #[inline(always)]
    fn call(&mut self, should_jump: bool) -> (u16, u8) {
        let next_pc = self.pc.wrapping_add(3);       // instruction after CALL
        let target = self.read_next_word();         // read 16-bit immediate
        if should_jump {
            self.push(next_pc);                     // push return address (PC after CALL)
            (target, 24)                            // return target PC and cycles
        } else {
            (next_pc, 12)                           // skip over operand
        }
    }

#[inline(always)]
pub(crate) fn return_(&mut self, should_jump: bool) -> u16 {
            if should_jump {
                self.pop()
            } else {
                self.pc.wrapping_add(1)
        }
    }

    #[inline(always)]
    fn rst(&mut self) {
        self.push(self.pc.wrapping_add(1));
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.register.a & value;
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.carry = false;
        self.register.f.half_carry = true;
        new_value
    }
    fn ccf(&mut self) {
        self.register.f.carry = !self.register.f.carry;
    }
    fn scf(&mut self) {
        self.register.f.carry = true;
    }
    fn rr_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_in = if self.register.f.carry { 0x80 } else { 0x00 };
        let new_value = carry_in | (value >> 1);
        self.register.f.zero = set_zero && new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = value & 0x01 != 0;
        new_value
    }

    #[inline(always)]
    fn rra(&mut self) {
        self.register.a = self.rr_through_carry(self.register.a, false);
    }

    #[inline(always)]
    fn rr(&mut self, value: u8) -> u8 {
        self.rr_through_carry(value, true)
    }

    // ====== Rotate Left Through Carry ======
    #[inline(always)]
    fn rl_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_in = if self.register.f.carry { 0x01 } else { 0x00 };
        let new_value = (value << 1) | carry_in;
        self.register.f.zero = set_zero && new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = value & 0x80 != 0;
        new_value
    }

    #[inline(always)]
    fn rla(&mut self) {
        self.register.a = self.rl_through_carry(self.register.a, false);
    }

    #[inline(always)]
    fn rl(&mut self, value: u8) -> u8 {
        self.rl_through_carry(value, true)
    }

    // ====== Rotate Right (circular) ======
    #[inline(always)]
    fn rrc(&mut self, value: u8) -> u8 {
        let bit0 = value & 0x01;
        let new_value = (value >> 1) | (bit0 << 7);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit0 != 0;
        new_value
    }

    #[inline(always)]
    fn rrca(&mut self) {
        self.register.a = self.rrc(self.register.a);
    }

    // ====== Rotate Left (circular) ======
    #[inline(always)]
    fn rlc(&mut self, value: u8) -> u8 {
        let bit7 = value & 0x80;
        let new_value = (value << 1) | (bit7 >> 7);
        self.register.f.zero = new_value == 0;
        self.register.f.subtract = false;
        self.register.f.half_carry = false;
        self.register.f.carry = bit7 != 0;
        new_value
    }

    #[inline(always)]
    fn rlca(&mut self) {
        self.register.a = self.rlc(self.register.a);
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
    pub fn daa(&mut self) {
        let mut a = self.register.a;
        let mut adjust = 0u8;
        let mut carry = false;

        if !self.register.f.subtract {
            // After addition
            if self.register.f.half_carry || (a & 0x0F) > 0x09 {
                adjust |= 0x06;
            }
            if self.register.f.carry || a > 0x99 {
                adjust |= 0x60;
                carry = true;
            }
            a = a.wrapping_add(adjust);
        } else {
            // After subtraction
            if self.register.f.half_carry {
                adjust |= 0x06;
            }
            if self.register.f.carry {
                adjust |= 0x60;
            }
            a = a.wrapping_sub(adjust);
        }

        self.register.a = a;

        // Flags
        self.register.f.zero = a == 0;
        self.register.f.half_carry = false;
        self.register.f.carry = carry || self.register.f.carry;
    }

}
