#![allow(unused_assignments)]

use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

use olc_pixel_game_engine as olc;

pub mod cpu;
pub mod device;

use self::{
    cpu::Dump,
    device::{
        vga::{CHAR_HEIGHT, CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
        Device,
    },
};

const SCALE: i32 = 1;

pub fn run(program: Vec<u8>, ivt_bytes: [u8; 510]) {
    // println!("{:?}", program);
    let ivt = Arc::new(Mutex::new(device::ram::Ram::new(0x0000, 0x0400)));
    {
        let mut l = ivt.lock().unwrap();
        l.set_name(String::from("IVT"));
        l.memory = ivt_bytes.to_vec();
    }
    
    let ram = Arc::new(Mutex::new(device::ram::Ram::new(0x0401, 0x4401)));
    let rom = Arc::new(Mutex::new(device::rom::Rom::new(program, 0x4402, 0x400)));
    
    let stack = Arc::new(Mutex::new(device::ram::Ram::new(0x4803, 0x4C03)));
    {
        let mut l = stack.lock().unwrap();
        l.set_name(String::from("Stack"));
    }
    
    let vga = Arc::new(Mutex::new(device::vga::VGA::new(0xA000)));
    let vga_scr = Arc::clone(&vga);
    let mut screen = device::vga::Screen::new(vga_scr);

    let mut map = device::map::DeviceMap::new();

    map.add(vga);
    map.add(ivt);
    map.add(ram);
    map.add(rom);
    map.add(stack);

    let running = Arc::new(AtomicBool::new(true));
    let running_screen = Arc::clone(&running);
    let mut cpu = cpu::CPU::new(map, 0x4402);
    cpu.sp = 0x4803;
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

    cpu.dump(Dump::All);
}
