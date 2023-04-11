#![allow(unused_assignments)]

use std::fs;

use crate::vcpu::cpu::Flags;

pub mod cpu;
pub mod device;

macro_rules! flag_value {
    ($flag_val:expr) => {
        if $flag_val {
            "On"
        } else {
            "Off"
        }
    };
}

pub fn run(program: Vec<u8>) {
    println!("{:?}", program);
    let rom = device::rom::Rom::new(program, 0x0000);
    let ram = device::ram::Ram::new(0x8000, 0xFFFF);
    let mut map = device::map::DeviceMap::new();

    map.add(ram);
    map.add(rom);

    let mut cpu = cpu::CPU::new(map, 0x0000);
    cpu.sp = 0x8000;
    let mut pins = cpu.pins;

    while cpu.running {
        pins = cpu.tick(pins);
    }

    fs::write(
        "cpu_dump.txt",
        format!(
            "Program Counter: 0x{:04x}
Stack Pointer: 0x{:04x}

Register:
    R1: 0x{:02x}
    R2: 0x{:02x}
    R3: 0x{:02x}
    R4: 0x{:02x}
    R5: 0x{:02x}
    R6: 0x{:02x}

Flags:
    Zero: {}
    GT  : {}
    LT  : {}
    OvrF: {}
    DWord: {}

Memory dump: 'memory.bin'",
            cpu.pc,
            cpu.sp,
            cpu.r1,
            cpu.r2,
            cpu.r3,
            cpu.r4,
            cpu.r5,
            cpu.r6,
            flag_value!(cpu.flags.contains(Flags::Z)),
            flag_value!(cpu.flags.contains(Flags::G)),
            flag_value!(cpu.flags.contains(Flags::L)),
            flag_value!(cpu.flags.contains(Flags::O)),
            flag_value!(cpu.flags.contains(Flags::D)),
        ),
    )
    .unwrap();
}
