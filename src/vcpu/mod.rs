#![allow(unused_assignments)]

use std::{
    sync::{atomic::AtomicBool, Arc, Mutex, mpsc},
    thread, collections::VecDeque,
};

use olc_pixel_game_engine as olc;

pub mod cpu;
pub mod device;

use crate::vcpu::{cpu::IrqPin, device::bios::KeyboardFlags};

use self::{cpu::{DebugInfo, Pins}, device::vga::KeyEvent};
#[allow(unused_imports)]
use self::{
    cpu::Dump,
    device::{
        vga::{CHAR_HEIGHT, CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, DEBUG_WIDTH, key_to_char},
        Device,
    },
};

const SCALE: i32 = 1;

#[allow(unused_variables)]
pub fn run(program: Vec<u8>, ivt_bytes: [u8; 510], debug_mode: bool) {
    let add_scr_width = if debug_mode { DEBUG_WIDTH } else { 0 };

    let keys: Arc<Mutex<VecDeque<KeyEvent>>> = Arc::new(Mutex::new(VecDeque::new()));
    let keys_scr = Arc::clone(&keys);

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

    let bda = Arc::new(Mutex::new(device::bios::BIOS::new(0x4C04)));
    let bda_map = Arc::clone(&bda);
    
    let vga = Arc::new(Mutex::new(device::vga::VGA::new(0xA000)));
    let vga_scr = Arc::clone(&vga);

    let (debug_tx, debug_rx) = mpsc::channel::<DebugInfo>();
    let mut pins = Pins::new();
    let mut screen = device::vga::Screen::new(vga_scr, debug_rx, debug_mode, keys_scr);

    let mut map = device::map::DeviceMap::new();

    map.add(vga);
    map.add(ivt);
    map.add(ram);
    map.add(rom);
    map.add(stack);
    map.add(bda_map);

    let running = Arc::new(AtomicBool::new(true));
    let running_screen = Arc::clone(&running);
    let mut cpu = cpu::CPU::new(0x4402, 0x4803, debug_mode);
    cpu.map = map;
    
    if debug_mode {
        cpu.debug_tx = Some(debug_tx);
    }
    
    let vga_thread = thread::spawn(move || {
        olc::start(
            "YuCPU PC",
            &mut screen,
            CHAR_WIDTH * SCREEN_WIDTH + add_scr_width,
            CHAR_HEIGHT * SCREEN_HEIGHT,
            SCALE,
            SCALE,
        )
        .unwrap();
        running_screen.store(false, std::sync::atomic::Ordering::Release);
    });

    loop {
        pins = cpu.tick(pins);
        
        if !cpu.running {
            running.store(false, std::sync::atomic::Ordering::Release);
        }

        if vga_thread.is_finished() {
            break;
        }

        let mut lock_keys = keys.lock().unwrap();
        // println!("Keys: {:?}", lock_keys);

        if lock_keys.len() > 0 {
            // We have a new key press! We can unwrap since we know the length is greater than one.
            let key_event = lock_keys.pop_back().unwrap();
            let mut lock_bda = bda.lock().unwrap();
            match key_event {
                KeyEvent::Up(key) => {
                    match key {
                        olc::Key::CTRL => lock_bda.set_keyboard_flag(KeyboardFlags::CONTROL, false).unwrap(),
                        olc::Key::SHIFT => lock_bda.set_keyboard_flag(KeyboardFlags::LSHIFT, false).unwrap(),
                        _ => {
                            lock_bda.set_keyboard_buffer(0).unwrap();
                        }
                    };
                },
                KeyEvent::Down(key) => {
                    match key {
                        olc::Key::CTRL => lock_bda.set_keyboard_flag(KeyboardFlags::CONTROL, true).unwrap(),
                        olc::Key::SHIFT => lock_bda.set_keyboard_flag(KeyboardFlags::LSHIFT, true).unwrap(),
                        _ => {
                            lock_bda.set_keyboard_buffer(key_to_char(key)).unwrap();
                            pins.irq = IrqPin::On(1)
                        }
                    };
                },
            }

            println!("Key event {:?}", key_event);
        }
    }

    vga_thread.join().unwrap();

    cpu.dump(Dump::All);
}
