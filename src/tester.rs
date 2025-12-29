use std::fs;
use std::path::Path;

use crate::cpu::CPU;
use crate::bus::MemoryBus;


pub(crate) fn test() {
    println!("Running Blargg test: 01-special.gb\n");

    // -----------------------
    // Load Blargg ROM
    // -----------------------
    let rom_path = "01-special.gb";
    let rom = fs::read(Path::new(rom_path))
        .expect("Failed to read Blargg ROM");

    // -----------------------
    // Create Bus + CPU
    // -----------------------
    let mut cpu = CPU::new(None,rom);

    // -----------------------
    // Post-boot CPU state
    // (NO boot ROM)
    // -----------------------
    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;

    cpu.register.set_af(0x01B0);
    cpu.register.set_bc(0x0013);
    cpu.register.set_de(0x00D8);
    cpu.register.set_hl(0x014D);

    cpu.interrupts_enabled = false;

    // Ensure boot ROM is unmapped
    cpu.bus.write_byte(0xFF50, 0x01);

    // -----------------------
    // Run until Blargg finishes
    // -----------------------
    let mut step_count = 0;
    loop {
        step_count += 1;

        let pc = cpu.pc;
        let opcode = cpu.bus.read_byte(pc);

        cpu.step();

        // Print state after execution

        // Stop if Blargg prints result
        if cpu.bus.serial_control == 0x81 || step_count == 1 {
            println!(
                "#{:05} PC:{:04X} OP:{:02X} A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X}",
                step_count,
                pc,
                opcode,
                cpu.register.a,
                cpu.register.f,
                cpu.register.b,
                cpu.register.c,
                cpu.register.d,
                cpu.register.e,
                cpu.register.h,
                cpu.register.l,
                cpu.sp
            );
            break;
        }
    }
}
