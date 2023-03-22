use bitflags::bitflags;

use super::device::map::{DeviceMap, DeviceMapResult};
use super::instruction::opcode::Instruction;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u32 {
        const Z = 0b00000001;
        const O = 0b00000010;
        const L = 0b01000000;
        const G = 0b00100000;
        const FLAGS = Self::Z.bits() | Self::O.bits() | Self::L.bits() | Self::G.bits();
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
    pub is: u8,
    pub pins: Pins,
    pub map: DeviceMap,
}

// Public Code
impl CPU {
    #[allow(dead_code)]
    pub fn new(map: DeviceMap) -> CPU {
        CPU {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            sp: 0,
            pc: 0x8000,
            bp: 0,
            flags: Flags::empty(),
            ir: 0,
            dr: 0,
            is: 0,
            pins: Pins::new(),
            map,
        }
    }
}

impl CPU {
    pub fn tick(&mut self, mut pins: Pins) -> Pins {
        pins.rw = ReadWrite::Read;

        self.ir = match self.map.read(self.pc as u32) {
            DeviceMapResult::Ok(data) => data,
            DeviceMapResult::NoDevices => {
                panic!("No devices attached. Could not get instruction info.")
            }
        };

        if ((0x8 & self.ir) >> 3) == 1 {
            self.dr = match self.map.read((self.pc + 2) as u32) {
                DeviceMapResult::Ok(data) => data,
                DeviceMapResult::NoDevices => {
                    panic!("No devices attached. Could not get instruction data.")
                }
            };
            self.is = 4;
        } else {
            self.dr = match self.map.read_byte((self.pc + 2) as u32) {
                DeviceMapResult::Ok(data) => data as u16,
                DeviceMapResult::NoDevices => {
                    panic!("No devices attached. Could not get instruction data.")
                }
            };
            self.is = 3;
        }

        let mask = 0xFF00;

        let res = match Instruction::from_opcode(&(((mask & self.ir) >> 8) as u8)) {
            Ok(data) => data,
            Err(error) => panic!("Instruction error: {:?}", error),
        };

        println!("Running {:?} with addr mode {:?}.", res.opcode, res.mode);
        println!(
            "Opcode {:08b} ir {} dr {}",
            (mask & self.ir),
            self.ir,
            self.dr
        );

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
