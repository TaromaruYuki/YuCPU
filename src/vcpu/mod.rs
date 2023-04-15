#![allow(unused_assignments)]

use std::{
    fs,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

use crate::vcpu::cpu::Flags;
use olc_pixel_game_engine as olc;

pub mod cpu;
pub mod device;

use self::device::vga::{CHAR_HEIGHT, CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: i32 = 2;

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
    // println!("{:?}", program);
    let rom = Arc::new(Mutex::new(device::rom::Rom::new(program, 0x8000)));
    let ram = Arc::new(Mutex::new(device::ram::Ram::new(0x0000, 0x7FFF)));

    let vga = Arc::new(Mutex::new(device::vga::VGA::new(0xA000)));
    let vga_scr = Arc::clone(&vga);
    let mut screen = device::vga::Screen::new(vga_scr);

    let mut map = device::map::DeviceMap::new();

    map.add(vga);
    map.add(ram);
    map.add(rom);

    let running = Arc::new(AtomicBool::new(true));
    let running_screen = Arc::clone(&running);
    let mut cpu = cpu::CPU::new(map, 0x8000);
    cpu.sp = 0x0000;
    let mut pins = cpu.pins;

    let vga_thread = thread::spawn(move || {
        olc::start(
            "YuCPU PC",
            &mut screen,
            CHAR_WIDTH * SCREEN_WIDTH,
            CHAR_HEIGHT * SCREEN_HEIGHT,
            SCALE,
            SCALE,
        )
        .unwrap();
        running_screen.store(false, std::sync::atomic::Ordering::Release);
    });

    while running.load(std::sync::atomic::Ordering::Acquire) {
        pins = cpu.tick(pins);

        if !cpu.running {
            running.store(false, std::sync::atomic::Ordering::Release);
        }
    }

    vga_thread.join().unwrap();

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
