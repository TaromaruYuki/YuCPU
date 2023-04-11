use bitflags::bitflags;

use crate::{vcpu::device::DeviceResponse, common::instruction::opcode::InstructionError};

use super::device::map::{DeviceMap, DeviceMapResult};
use crate::common::instruction::opcode::Instruction;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u32 {
        const Z = 0b00000001;
        const O = 0b00000010;
        const L = 0b01000000;
        const G = 0b00100000;
        const D = 0b10000000;
        const FLAGS = Self::Z.bits() | Self::O.bits() | Self::L.bits() | Self::G.bits() | Self::G.bits();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadWrite {
    Read,
    #[allow(dead_code)]
    Write,
}

#[derive(Clone, Copy)]
pub struct Pins {
    pub data: u16,
    pub address: u32, // We will only use 20 bits out of the 32: 0b0000_00000000_00000000
    pub rw: ReadWrite,
}

impl Pins {
    pub fn new() -> Pins {
        Pins {
            data: 0,
            address: 0,
            rw: ReadWrite::Read,
        }
    }
}

impl Default for Pins {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub sp: u16,
    pub pc: u16,
    pub bp: u16,
    pub flags: Flags,
    pub ir: u16,
    pub dr: u16,
    pub ad: u8,
    pub is: u8,
    pub pins: Pins,
    pub map: DeviceMap,
    pub running: bool,
}

// Public Code
impl CPU {
    #[allow(dead_code)]
    pub fn new(map: DeviceMap, pc: u16) -> CPU {
        CPU {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            sp: 0,
            pc,
            bp: 0,
            flags: Flags::empty(),
            ir: 0,
            dr: 0,
            ad: 0,
            is: 0,
            pins: Pins::new(),
            map,
            running: true,
        }
    }
}

impl CPU {
    pub fn tick(&mut self, mut pins: Pins) -> Pins {
        self.flags.set(Flags::D, false);
        pins.rw = ReadWrite::Read;

        self.ir = match self.map.read(self.pc as u32) {
            DeviceMapResult::Ok(data) => data,
            DeviceMapResult::NoDevices => {
                panic!("No devices attached. Could not get instruction info.")
            }
            DeviceMapResult::Error(err) => {
                if err == DeviceResponse::WriteOnly {
                    panic!("Device write only. Could not read value.");
                } else {
                    panic!("Unknown error. Could not read value.");
                }
            }
        };

        match (0xC & self.ir) >> 2 {
            0b00 => {
                self.dr = match self.map.read_byte((self.pc + 2) as u32) {
                    DeviceMapResult::Ok(data) => data as u16,
                    DeviceMapResult::NoDevices => {
                        panic!("No devices attached. Could not get instruction data.")
                    }
                    DeviceMapResult::Error(err) => {
                        if err == DeviceResponse::WriteOnly {
                            panic!("Device write only. Could not read value.");
                        } else {
                            panic!("Unknown error. Could not read value.");
                        }
                    }
                };
                self.is = 3;
            }
            0b01 => {
                self.dr = match self.map.read((self.pc + 2) as u32) {
                    DeviceMapResult::Ok(data) => data,
                    DeviceMapResult::NoDevices => {
                        panic!("No devices attached. Could not get instruction data.")
                    }
                    DeviceMapResult::Error(err) => {
                        if err == DeviceResponse::WriteOnly {
                            panic!("Device write only. Could not read value.");
                        } else {
                            panic!("Unknown error. Could not read value.");
                        }
                    }
                };
                self.is = 4;
            }
            0b10 => {
                self.ad = match self.map.read_byte((self.pc + 2) as u32) {
                    DeviceMapResult::Ok(data) => data & 0xF,
                    DeviceMapResult::NoDevices => {
                        panic!("No devices attached. Could not get instruction data.")
                    }
                    DeviceMapResult::Error(err) => {
                        if err == DeviceResponse::WriteOnly {
                            panic!("Device write only. Could not read value.");
                        } else {
                            panic!("Unknown error. Could not read value.");
                        }
                    }
                };

                self.dr = match self.map.read((self.pc + 3) as u32) {
                    DeviceMapResult::Ok(data) => data,
                    DeviceMapResult::NoDevices => {
                        panic!("No devices attached. Could not get instruction data.")
                    }
                    DeviceMapResult::Error(err) => {
                        if err == DeviceResponse::WriteOnly {
                            panic!("Device write only. Could not read value.");
                        } else {
                            panic!("Unknown error. Could not read value.");
                        }
                    }
                };

                self.flags.set(Flags::D, true);

                self.is = 5;
            }
            0b11 => {
                self.is = 2;
            }
            _ => panic!("Data match was over 2 bytes..."),
        };

        let mask = 0xFF00;

        let res = match Instruction::from_opcode(&(((mask & self.ir) >> 8) as u8)) {
            Ok(data) => data,
            Err(error) => match error {
                InstructionError::InvalidOpcode => panic!("Invalid opcode: {}", &(((mask & self.ir) >> 8) as u8))
            }
        };

        println!("Running {:?} with addr mode {:?}.", res.opcode, res.mode);
        println!(
            "Opcode {:08b} ir {} dr {} ad {}",
            (mask & self.ir),
            self.ir,
            self.dr,
            self.ad,
        );

        println!("Executing instruction...");
        (res.exec)(self);

        self.is = 0;
        self.ir = 0;
        self.dr = 0;

        pins
    }

    pub fn advance(&mut self) {
        self.pc += self.is as u16;
    }

    #[allow(unused_variables, clippy::needless_return)]
    pub fn decode_register(&mut self, reg: u8) -> &mut u16 {
        return match reg {
            0 => &mut self.r1,
            1 => &mut self.r2,
            2 => &mut self.r3,
            3 => &mut self.r4,
            4 => &mut self.r5,
            5 => &mut self.r6,
            6 => &mut self.pc,
            7 => &mut self.sp,
            8 => &mut self.bp,
            _ => panic!("Invalid register"),
        };
    }
}
