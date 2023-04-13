use std::sync::{Arc, Mutex};

use super::{Device, DeviceResponse};
use olc_pixel_game_engine as olc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Screen {
    #[allow(dead_code)]
    vga: Arc<Mutex<VGA>>,
}

impl Screen {
    pub fn new(vga: Arc<Mutex<VGA>>) -> Self {
        Self { vga }
    }
}

impl olc::Application for Screen {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
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
    character: u8,
    color: VGAColor,
    background: VGAColor,
}

impl VGACharacter {
    pub fn default() -> Self {
        Self {
            character: 0x00,
            color: VGAColor::DarkGray,
            background: VGAColor::Black,
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
        Self {
            memory: vec![VGACharacter::default(); 0x10000],
            start,
            end: start + 0x10000,
        }
    }

    fn relative(&self, addr: u32) -> usize {
        (addr - self.start) as usize
    }
}

impl Device for VGA {
    fn read(&mut self, addr: u32) -> DeviceResponse<u16> {
        if addr >= self.start && addr <= self.end {
            let bytes = self.memory[self.relative(addr)].bytes();
            let data1 = (bytes[0] as u16) << 8;
            let data2 = bytes[1] as u16;

            return DeviceResponse::Ok(data1 | data2);
        }

        DeviceResponse::NotMyAddress
    }

    fn read_byte(&mut self, addr: u32) -> DeviceResponse<u8> {
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

            if relative_addr % 2 == 0 {
                // We are changing the color(s)

                self.memory[relative_addr].background = VGAColor::iter()
                    .nth(((value & 0xF0) >> 4) as usize)
                    .unwrap();
                self.memory[relative_addr].color =
                    VGAColor::iter().nth((value & 0x0F) as usize).unwrap();

                return DeviceResponse::Ok(());
            } else {
                // We are changing the character

                self.memory[relative_addr].character = value;

                return DeviceResponse::Ok(());
            }
        }

        DeviceResponse::NotMyAddress
    }

    fn get_name(&self) -> String {
        String::from("VGA")
    }

    fn get_memory(&self) -> &Vec<u8> {
        todo!()
    }
}
