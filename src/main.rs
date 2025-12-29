use std::fs;
use std::path::Path;
use std::io::{self, BufRead};
use crate::cpu::CPU;
use crate::bus::MemoryBus;
mod Fake_gpu;
mod cpu;
mod bus;
mod instruction;
mod cartride;
mod timer;
mod GPU;
mod InterruptFlags;
mod keypad;
mod bit;
mod Register;

fn main() -> io::Result<()> {
    println!("Running Blargg test: 02-interuppts.gb\n");

    // -----------------------
    // Load ROM
    // -----------------------
    let rom_path = "test_rom/02-interrupts.gb";
    let rom = fs::read(Path::new(rom_path))
        .expect("Failed to read Blargg ROM");

    // -----------------------
    // Read Blargg log
    // -----------------------
    let log_path = Path::new("log_txt/blargg_log_interuppt.txt");
    let file = fs::File::open(log_path)?;
    let blargg_lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    // -----------------------
    // Create CPU
    // -----------------------
    let mut cpu = CPU::new(None, rom);

    // -----------------------
    // Post-boot CPU state
    // -----------------------
    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;
    cpu.register.set_af(0x01B0);
    cpu.register.set_bc(0x0013);
    cpu.register.set_de(0x00D8);
    cpu.register.set_hl(0x014D);
    cpu.interrupts_enabled = false;

    cpu.bus.write_byte(0xFF50, 0x01); // Ensure boot ROM is unmapped
    for i in 0..0x80 {
        cpu.bus.write_byte(0xFF00 + i, i as u8);
    }

    // -----------------------
    // Run and compare
    // -----------------------
    for (step_count, blargg_line) in blargg_lines.iter().enumerate() {
        cpu.step();
        let pc = cpu.pc;
        let opcode = cpu.bus.read_byte(pc);


        // Format our CPU state like Blargg log
        let mut op_bytes = [0u8; 4];
        for i in 0..4 {
            op_bytes[i] = cpu.bus.read_byte(cpu.pc + i as u16);
        }
        let mut my_line: String;
        if cpu.register.f == 0 {
                my_line = format!(
                "A: {:02X} F: 00 B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
                cpu.register.a,
                cpu.register.b,
                cpu.register.c,
                cpu.register.d,
                cpu.register.e,
                cpu.register.h,
                cpu.register.l,
                cpu.sp,
                cpu.pc,
                op_bytes[0],
                op_bytes[1],
                op_bytes[2],
                op_bytes[3],
            );
        } else {
            my_line = format!(
                "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
                cpu.register.a,
                cpu.register.f,
                cpu.register.b,
                cpu.register.c,
                cpu.register.d,
                cpu.register.e,
                cpu.register.h,
                cpu.register.l,
                cpu.sp,
                cpu.pc,
                op_bytes[0],
                op_bytes[1],
                op_bytes[2],
                op_bytes[3],
            );
        }


        // Compare as strings
        if my_line != *blargg_line {
            println!("Mismatch at step {}:", step_count + 1);
            println!("  Expected: {}", blargg_line);
            println!("  Found:    {}", my_line);
            break;
        }
        // Stop if Blargg prints result or hit max steps
        if cpu.bus.serial_control == 0x81 {
            break;
        }
    }

    println!("Blargg 01-special test finished!");
    Ok(())
}
