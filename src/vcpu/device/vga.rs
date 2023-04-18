use std::{
    fs,
    process::exit,
    sync::{Arc, Mutex},
};

use super::{Device, DeviceResponse};
use olc_pixel_game_engine as olc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub const CHAR_WIDTH: i32 = 8;
pub const CHAR_HEIGHT: i32 = 16;
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 25;

pub struct Screen {
    // #[allow(dead_code)]
    vga: Arc<Mutex<VGA>>,
    font: Vec<u8>,
}

impl Screen {
    pub fn new(vga: Arc<Mutex<VGA>>) -> Self {
        let file = match fs::read("resources/AVGA2_8x16.bin") {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to open rom font.\n{error}");
                exit(1);
            }
        };

        Self { vga, font: file }
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

        olc::clear(olc::BLACK);

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
