use std::ops::Range;

use super::{Device, DeviceResponse};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct KeyboardFlags: u32 {
        const INSERT  = 0b10000000;
        const CAPS    = 0b01000000;
        const NUM     = 0b00100000;
        const SCROLL  = 0b00010000;
        const ALT     = 0b00001000;
        const CONTROL = 0b00000100;
        const LSHIFT  = 0b00000010;
        const RSHIFT  = 0b00000001;
    }
}

#[derive(Debug)]
enum BDAData {
    KeyboardShiftFlags {
        read_only: bool,
        data: KeyboardFlags,
        range: Range<u8>,
    },
    KeyboardBuffer {
        read_only: bool,
        data: u8,
        range: Range<u8>,
    },
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
struct BDA {
    pub memory: Vec<BDAData>,
}

impl BDA {
    pub fn new() -> Self {
        Self {
            memory: vec![
                BDAData::KeyboardShiftFlags {
                    read_only: true,
                    data: KeyboardFlags::empty(),
                    range: 22..30,
                },
                BDAData::KeyboardBuffer {
                    read_only: true,
                    data: 0,
                    range: 60..68,
                },
            ],
        }
    }
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct BIOS {
    start: u32,
    end: u32,
    bda: BDA,
}

#[derive(Debug)]
pub enum BIOSError {
    NoDataSection,
}

impl BIOS {
    pub fn new(start: u32) -> Self {
        Self {
            start,
            end: start + 0xFF,
            bda: BDA::new(),
        }
    }

    fn relative(&self, addr: u32) -> u8 {
        (addr - self.start) as u8
    }

    fn read_bda_data(data: &BDAData) -> (u16, Range<u8>, bool) {
        match data {
            BDAData::KeyboardShiftFlags {
                read_only,
                data,
                range,
            } => (data.bits() as u16, range.clone(), *read_only),
            BDAData::KeyboardBuffer {
                read_only,
                data,
                range,
            } => (*data as u16, range.clone(), *read_only),
        }
    }

    pub fn set_keyboard_buffer(&mut self, key: u8) -> Result<(), BIOSError> {
        for (i, data) in self.bda.memory.iter().enumerate() {
            match data {
                #[allow(unused_variables)]
                BDAData::KeyboardBuffer {
                    read_only,
                    data,
                    range,
                } => {
                    self.bda.memory[i] = BDAData::KeyboardBuffer {
                        read_only: *read_only,
                        data: key,
                        range: range.clone(),
                    };

                    return Ok(());
                }
                _ => continue,
            }
        }

        Err(BIOSError::NoDataSection)
    }

    pub fn set_keyboard_flag(&mut self, flag: KeyboardFlags, value: bool) -> Result<(), BIOSError> {
        for (i, data) in self.bda.memory.iter().enumerate() {
            match data {
                #[allow(unused_variables)]
                BDAData::KeyboardShiftFlags {
                    read_only,
                    data,
                    range,
                } => {
                    let mut mod_flags = *data;
                    mod_flags.set(flag, value);
                    self.bda.memory[i] = BDAData::KeyboardShiftFlags {
                        read_only: *read_only,
                        data: mod_flags,
                        range: range.clone(),
                    };

                    return Ok(());
                }
                _ => continue,
            }
        }

        Err(BIOSError::NoDataSection)
    }
}

impl Device for BIOS {
    fn read(&self, addr: u32) -> DeviceResponse<u16> {
        if addr >= self.start && addr <= self.end {
            for data in &self.bda.memory {
                let (read_data, read_range, _) = BIOS::read_bda_data(data);

                if read_range.contains(&self.relative(addr)) {
                    return DeviceResponse::Ok(read_data);
                }
            }
        }

        DeviceResponse::NotMyAddress
    }

    fn read_byte(&self, addr: u32) -> DeviceResponse<u8> {
        if addr >= self.start && addr <= self.end {
            let addr = &self.relative(addr);

            for data in &self.bda.memory {
                let (read_data, read_range, _) = BIOS::read_bda_data(data);

                if read_range.contains(addr) {
                    if addr % 2 == 0 {
                        // Upper bytes
                        return DeviceResponse::Ok((read_data >> 4) as u8);
                    } else {
                        // Lower bytes
                        return DeviceResponse::Ok(read_data as u8);
                    }
                }
            }
        }

        DeviceResponse::NotMyAddress
    }

    fn write(&mut self, addr: u32, _value: u16) -> DeviceResponse<()> {
        if addr >= self.start && addr <= self.end {
            return DeviceResponse::ReadOnly;
        }

        DeviceResponse::NotMyAddress
    }

    fn write_byte(&mut self, addr: u32, _value: u8) -> DeviceResponse<()> {
        if addr >= self.start && addr <= self.end {
            return DeviceResponse::ReadOnly;
        }

        DeviceResponse::NotMyAddress
    }

    fn get_name(&self) -> String {
        String::from("BIOS")
    }

    fn set_name(&mut self, _name: String) {
        panic!("Cannot set name for BIOS");
    }

    fn get_memory(&self) -> Vec<u8> {
        Vec::new()
    }
}
