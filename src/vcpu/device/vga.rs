use std::{
    fs,
    process::exit,
    sync::{Arc, Mutex, mpsc::Receiver}, collections::{VecDeque, HashMap},
};

use crate::vcpu::cpu::{DebugInfo, Flags};

use super::{Device, DeviceResponse};
use olc_pixel_game_engine as olc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub const CHAR_WIDTH: i32 = 8;
pub const CHAR_HEIGHT: i32 = 16;
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 25;

pub const DEBUG_WIDTH: i32 = 250;
pub const DEBUG_PADDING: i32 = 5;

pub const VALID_KEYS: [olc::Key; 42] = [
    olc::Key::SPACE, olc::Key::CTRL, olc::Key::SHIFT, olc::Key::PERIOD, olc::Key::ENTER, olc::Key::BACK,
    olc::Key::K0, olc::Key::K1, olc::Key::K2, olc::Key::K3, olc::Key::K4, olc::Key::K5, olc::Key::K6, olc::Key::K7, olc::Key::K8, olc::Key::K9,
    olc::Key::A, olc::Key::B, olc::Key::C, olc::Key::D, olc::Key::E, olc::Key::F, olc::Key::G, olc::Key::H, olc::Key::I, olc::Key::J,
    olc::Key::K, olc::Key::L, olc::Key::M, olc::Key::N, olc::Key::O, olc::Key::P, olc::Key::Q, olc::Key::R, olc::Key::S, olc::Key::T,
    olc::Key::U, olc::Key::V,olc::Key::W, olc::Key::X, olc::Key::Y, olc::Key::Z,
];

pub fn key_to_char(key: olc::Key) -> u8 {
    match key {
        olc::Key::SPACE => 32,
        olc::Key::PERIOD => 46,
        olc::Key::ENTER => 10,
        olc::Key::BACK => 8,
        olc::Key::K0 => 48,
        olc::Key::K1 => 49,
        olc::Key::K2 => 50,
        olc::Key::K3 => 51,
        olc::Key::K4 => 52,
        olc::Key::K5 => 53,
        olc::Key::K6 => 54,
        olc::Key::K7 => 55,
        olc::Key::K8 => 56,
        olc::Key::K9 => 57,
        olc::Key::A => 65,
        olc::Key::B => 66,
        olc::Key::C => 67,
        olc::Key::D => 68,
        olc::Key::E => 69,
        olc::Key::F => 70,
        olc::Key::G => 71,
        olc::Key::H => 72,
        olc::Key::I => 73,
        olc::Key::J => 74,
        olc::Key::K => 75,
        olc::Key::L => 76,
        olc::Key::M => 77,
        olc::Key::N => 78,
        olc::Key::O => 79,
        olc::Key::P => 80,
        olc::Key::Q => 81,
        olc::Key::R => 82,
        olc::Key::S => 83,
        olc::Key::T => 84,
        olc::Key::U => 85,
        olc::Key::V => 86,
        olc::Key::W => 87,
        olc::Key::X => 88,
        olc::Key::Y => 89,
        olc::Key::Z => 90,
        _ => panic!("Invalid key :(")
    }
}

#[derive(Debug)]
pub enum KeyEvent {
    Up(olc::Key),
    Down(olc::Key)
}

pub struct Screen {
    // #[allow(dead_code)]
    vga: Arc<Mutex<VGA>>,
    font: Vec<u8>,
    debug_rx: Receiver<DebugInfo>,
    debug_mode: bool,
    keys: Arc<Mutex<VecDeque<KeyEvent>>>,
}

impl Screen {
    pub fn new(vga: Arc<Mutex<VGA>>, debug_rx: Receiver<DebugInfo>, debug_mode: bool, keys: Arc<Mutex<VecDeque<KeyEvent>>>) -> Self {
        let file = match fs::read("resources/AVGA2_8x16.bin") {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to open rom font.\n{error}");
                exit(1);
            }
        };

        Self { vga, font: file, debug_rx, debug_mode, keys }
    }

    pub fn vga_color_to_pixel(color: VGAColor) -> olc::Pixel {
        match color {
            VGAColor::Black => olc::Pixel::rgb(0x00, 0x00, 0x00),
            VGAColor::Blue => olc::Pixel::rgb(0x00, 0x00, 0xAA),
            VGAColor::Green => olc::Pixel::rgb(0x00, 0xAA, 0x00),
            VGAColor::Cyan => olc::Pixel::rgb(0x00, 0xAA, 0xAA),
            VGAColor::Red => olc::Pixel::rgb(0xAA, 0x00, 0x00),
            VGAColor::Magenta => olc::Pixel::rgb(0xAA, 0x00, 0xAA),
            VGAColor::DarkYellow => olc::Pixel::rgb(0xAA, 0xAA, 0x00),
            VGAColor::LightGray => olc::Pixel::rgb(0xAA, 0xAA, 0xAA),
            VGAColor::DarkGray => olc::Pixel::rgb(0x55, 0x55, 0x55),
            VGAColor::LightBlue => olc::Pixel::rgb(0x55, 0x55, 0xFF),
            VGAColor::LightGreen => olc::Pixel::rgb(0x55, 0xFF, 0x55),
            VGAColor::LightCyan => olc::Pixel::rgb(0x55, 0xFF, 0xFF),
            VGAColor::LightRed => olc::Pixel::rgb(0xFF, 0x55, 0x55),
            VGAColor::LightMagenta => olc::Pixel::rgb(0xFF, 0x55, 0xFF),
            VGAColor::Yellow => olc::Pixel::rgb(0xFF, 0xFF, 0x55),
            VGAColor::White => olc::Pixel::rgb(0xFF, 0xFF, 0xFF),
        }
    }

    pub fn debug_scr(&self, debug_info: DebugInfo) {
        let offset_x = CHAR_WIDTH * SCREEN_WIDTH + DEBUG_PADDING;
        let offset_y = DEBUG_PADDING;

        olc::fill_rect(offset_x - DEBUG_PADDING, 0, DEBUG_WIDTH, CHAR_HEIGHT * SCREEN_HEIGHT, olc::BLUE);

        olc::draw_string(offset_x, offset_y + 00, &format!("R1: 0x{:x}", debug_info.r1), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 10, &format!("R2: 0x{:x}", debug_info.r2), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 20, &format!("R3: 0x{:x}", debug_info.r3), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 30, &format!("R4: 0x{:x}", debug_info.r4), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 40, &format!("R5: 0x{:x}", debug_info.r5), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 50, &format!("R6: 0x{:x}", debug_info.r6), olc::WHITE).unwrap();

        olc::draw_string(offset_x + 100, offset_y, &format!("PC: 0x{:x}", debug_info.pc), olc::WHITE).unwrap();
        olc::draw_string(offset_x + 100, offset_y + 10, &format!("SP: 0x{:x}", debug_info.sp), olc::WHITE).unwrap();
        olc::draw_string(offset_x + 100, offset_y + 20, &format!("BP: 0x{:x}", debug_info.bp), olc::WHITE).unwrap();

        olc::draw_string(offset_x + 100, offset_y + 40, &format!("Pins:"), olc::WHITE).unwrap();
        olc::draw_string(offset_x + 100, offset_y + 50, &format!("IRQ Pins: {:?}", debug_info.pins.irq), olc::WHITE).unwrap();

        olc::draw_string(offset_x, offset_y + 70, "Flags:", olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 80, &format!("D: {}", debug_info.flags.contains(Flags::D)), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 90, &format!("G: {}", debug_info.flags.contains(Flags::G)), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 100, &format!("L: {}", debug_info.flags.contains(Flags::L)), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 110, &format!("O: {}", debug_info.flags.contains(Flags::O)), olc::WHITE).unwrap();
        olc::draw_string(offset_x, offset_y + 120, &format!("Z: {}", debug_info.flags.contains(Flags::Z)), olc::WHITE).unwrap();
    }
}

impl olc::Application for Screen {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        let memory = match self.vga.lock() {
            Ok(lock) => {
                // println!("Got lock!");
                lock
            }
            Err(err) => {
                panic!("Could not get lock. {}", err);
            }
        };

        let mut lock_keys = self.keys.lock().unwrap();

        for key in VALID_KEYS {
            let key_status = olc::get_key(key);

            if key_status.pressed {
                lock_keys.push_back(KeyEvent::Down(key));
            } else if key_status.released {
                lock_keys.push_back(KeyEvent::Up(key));
            }
        }

        olc::clear(olc::BLACK);

        if self.debug_mode {
            let debug_info = self.debug_rx.recv().unwrap();
            self.debug_scr(debug_info);
        }


        // println!("{:?}", memory.read_byte(655361));
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let character = memory.memory[(y * SCREEN_WIDTH + x) as usize];

                let background = Screen::vga_color_to_pixel(character.background);
                let color = Screen::vga_color_to_pixel(character.color);

                let mask: [i32; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
                let glyph: usize = ((character.character as i32) * CHAR_HEIGHT) as usize;

                // println!("Glyph: {}", glyph);

                for cy in 0..character.height {
                    for cx in 0..character.width {
                        if (self.font[glyph + cy as usize] as i32) & mask[cx as usize] > 0 {
                            olc::draw(x * CHAR_WIDTH + cx, y * CHAR_HEIGHT + cy, color);
                        } else {
                            olc::draw(x * CHAR_WIDTH + cx, y * CHAR_HEIGHT + cy, background);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, EnumIter)]
pub enum VGAColor {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    DarkYellow,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

#[derive(Clone, Copy)]
pub struct VGACharacter {
    pub character: u8,
    pub color: VGAColor,
    pub background: VGAColor,
    pub width: i32,
    pub height: i32,
}

impl VGACharacter {
    pub fn default() -> Self {
        Self {
            character: 0,
            color: VGAColor::LightGray,
            background: VGAColor::Black,
            width: CHAR_WIDTH,
            height: CHAR_HEIGHT,
        }
    }

    pub fn bytes(&self) -> [u8; 2] {
        let color_num = self.color as u8;
        let background_num = self.color as u8;
        let attr_byte = (background_num << 4) | color_num;

        [attr_byte, self.character]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct VGA {
    pub memory: Vec<VGACharacter>,
    start: u32,
    end: u32,
}

impl VGA {
    pub fn new(start: u32) -> Self {
        #[allow(unused_mut)]
        let mut mem = vec![VGACharacter::default(); (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
        // for (i, char) in "Hello, world!".chars().into_iter().enumerate() {
        //     mem[i].character = char as u8;
        // }

        Self {
            memory: mem,
            start,
            end: start + (SCREEN_WIDTH * SCREEN_HEIGHT * 2) as u32,
        }
    }

    fn relative(&self, addr: u32) -> usize {
        (addr - self.start) as usize
    }
}

impl Device for VGA {
    fn read(&self, addr: u32) -> DeviceResponse<u16> {
        if addr >= self.start && addr <= self.end {
            let bytes = self.memory[self.relative(addr)].bytes();
            let data1 = (bytes[0] as u16) << 8;
            let data2 = bytes[1] as u16;

            return DeviceResponse::Ok(data1 | data2);
        }

        DeviceResponse::NotMyAddress
    }

    fn read_byte(&self, addr: u32) -> DeviceResponse<u8> {
        if addr >= self.start && addr <= self.end {
            let relative_addr = self.relative(addr);
            let bytes = self.memory[relative_addr].bytes();

            if relative_addr % 2 == 0 {
                return DeviceResponse::Ok(bytes[0]);
            } else {
                return DeviceResponse::Ok(bytes[1]);
            }
        }

        DeviceResponse::NotMyAddress
    }

    fn write(&mut self, addr: u32, value: u16) -> DeviceResponse<()> {
        if addr >= self.start && addr <= self.end {
            let background = ((value & 0xF000) >> 12) as u8;
            let color = ((value & 0x0F00) >> 8) as u8;
            let character = value as u8;
            let relative_addr = self.relative(addr);

            if relative_addr % 2 != 0 {
                return DeviceResponse::InvalidAddress;
            }

            self.memory[relative_addr].background =
                VGAColor::iter().nth(background as usize).unwrap();
            self.memory[relative_addr].color = VGAColor::iter().nth(color as usize).unwrap();
            self.memory[relative_addr].character = character;

            return DeviceResponse::Ok(());
        }

        DeviceResponse::NotMyAddress
    }

    fn write_byte(&mut self, addr: u32, value: u8) -> DeviceResponse<()> {
        if addr >= self.start && addr <= self.end {
            let relative_addr = self.relative(addr);

            // println!("Relative addr: {}", relative_addr / 2);

            if relative_addr % 2 == 0 {
                // We are changing the color(s)

                self.memory[relative_addr / 2].background = VGAColor::iter()
                    .nth(((value & 0xF0) >> 4) as usize)
                    .unwrap();
                self.memory[relative_addr / 2].color =
                    VGAColor::iter().nth((value & 0x0F) as usize).unwrap();

                return DeviceResponse::Ok(());
            } else {
                // We are changing the character

                self.memory[relative_addr / 2].character = value;

                return DeviceResponse::Ok(());
            }
        }

        DeviceResponse::NotMyAddress
    }

    fn get_name(&self) -> String {
        String::from("VGA")
    }

    fn set_name(&mut self, _name: String) {
        panic!("set_name should not be called for VGA.");
    }

    fn get_memory(&self) -> Vec<u8> {
        let mut temp: Vec<u8> = Vec::new();

        for character in &self.memory {
            temp.append(&mut character.bytes().to_vec());
        }

        temp
    }
}
