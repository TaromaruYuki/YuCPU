use std::{fs, sync::mpsc::Sender};

use bitflags::bitflags;

use crate::{common::instruction::opcode::InstructionError, vcpu::device::DeviceResponse};

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
        const FLAGS = Self::Z.bits() | Self::O.bits() | Self::L.bits() | Self::G.bits() | Self::D.bits();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadWrite {
    Read,
    #[allow(dead_code)]
    Write,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqPin {
    Off,
    On(u8),
}

#[derive(Clone, Copy)]
pub struct Pins {
    pub data: u16,
    pub address: u32, // We will only use 20 bits out of the 32: 0b0000_00000000_00000000
    pub rw: ReadWrite,
    pub irq: IrqPin,
}

impl Pins {
    pub fn new() -> Pins {
        Pins {
            data: 0,
            address: 0,
            rw: ReadWrite::Read,
            irq: IrqPin::Off,
        }
    }
}

impl Default for Pins {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Dump {
    All,
    Memory,
    Stats,
}

pub struct DebugInfo {
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub pc: u16,
    pub sp: u16,
    pub bp: u16,
    pub flags: Flags,
    pub pins: Pins,
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
    pub debug_mode: bool,
    pub debug_tx: Option<Sender<DebugInfo>>,
}

// Public Code
impl CPU {
    #[allow(dead_code)]
    pub fn new(pc: u16, sp: u16, debug_mode: bool) -> CPU {
        CPU {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            sp,
            pc,
            bp: 0,
            flags: Flags::empty(),
            ir: 0,
            dr: 0,
            ad: 0,
            is: 0,
            pins: Pins::new(),
            map: DeviceMap::new(),
            running: true,
            debug_mode,
            debug_tx: None,
        }
    }
}

impl CPU {
    pub fn tick(&mut self, mut pins: Pins) -> Pins {
        if let IrqPin::On(irq) = pins.irq {
            let jump_addr = match self.map.read((irq * 2) as u32) {
                DeviceMapResult::Ok(value) => value,
                DeviceMapResult::NoDevices => {
                    panic!("No devices attached. Could not read any values.")
                }
                DeviceMapResult::Error(err) => {
                    if err == DeviceResponse::ReadOnly {
                        panic!("Device read only. Could not read value.");
                    } else {
                        panic!("Unknown error. Could not read value.");
                    }
                }
            };

            if jump_addr != 0 {
                self.push_registers();
                self.pc = jump_addr;

                pins.irq = IrqPin::Off;
                return pins;
            } else {
                pins.irq = IrqPin::Off;
            }
        }

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
                InstructionError::InvalidOpcode => {
                    panic!("Invalid opcode: 0x{:x}", &(((mask & self.ir) >> 8) as u8))
                }
            },
        };

        // if res.opcode != Opcode::HLT {
        //     println!("Running {:?} with addr mode {:?}.", res.opcode, res.mode);
        //     println!(
        //         "Opcode {:08b} ir {} dr {} ad {}",
        //         (mask & self.ir),
        //         self.ir,
        //         self.dr,
        //         self.ad,
        //     );

        //     println!("Executing instruction...");
        // }

        (res.exec)(self);

        if self.debug_mode {
            let tx = self.debug_tx.as_ref().unwrap();

            tx.send(DebugInfo {
                r1: self.r1,
                r2: self.r2,
                r3: self.r3,
                r4: self.r4,
                r5: self.r5,
                r6: self.r6,
                pc: self.pc,
                sp: self.sp,
                bp: self.bp,
                flags: self.flags,
                pins,
            })
            .unwrap();
        }

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
            _ => panic!("Invalid register {}", reg),
        };
    }

    pub fn push_registers(&mut self) {
        for reg in 0..6 {
            let decoded_reg = *self.decode_register(reg);

            match self.map.write(self.sp as u32, decoded_reg) {
                DeviceMapResult::Ok(_) => (),
                DeviceMapResult::NoDevices => {
                    panic!("No devices attached. Could not read any values.")
                }
                DeviceMapResult::Error(err) => {
                    if err == DeviceResponse::ReadOnly {
                        panic!("Device read only. Could not read value.");
                    } else {
                        panic!("Unknown error. Could not read value.");
                    }
                }
            };

            self.sp += 2;
        }

        match self.map.write(self.sp as u32, self.flags.bits() as u16) {
            DeviceMapResult::Ok(_) => (),
            DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
            DeviceMapResult::Error(err) => {
                if err == DeviceResponse::ReadOnly {
                    panic!("Device read only. Could not read value.");
                } else {
                    panic!("Unknown error. Could not read value.");
                }
            }
        };

        self.sp += 2;

        match self.map.write(self.sp as u32, self.pc + self.is as u16) {
            DeviceMapResult::Ok(_) => (),
            DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
            DeviceMapResult::Error(err) => {
                if err == DeviceResponse::ReadOnly {
                    panic!("Device read only. Could not read value.");
                } else {
                    panic!("Unknown error. Could not read value.");
                }
            }
        };

        self.sp += 2;
    }

    pub fn pop_registers(&mut self) {
        // self.dump(Dump::All);
        self.sp -= 2;
        self.pc = match self.map.read(self.sp as u32) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
            DeviceMapResult::Error(err) => {
                if err == DeviceResponse::WriteOnly {
                    panic!("Device write only. Could not read value.");
                } else {
                    panic!("Unknown error. Could not read value.");
                }
            }
        };

        self.sp -= 2;
        self.flags = Flags::from_bits(match self.map.read(self.sp as u32) {
            DeviceMapResult::Ok(val) => val as u32,
            DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
            DeviceMapResult::Error(err) => {
                if err == DeviceResponse::ReadOnly {
                    panic!("Device read only. Could not read value.");
                } else {
                    panic!("Unknown error. Could not read value.");
                }
            }
        })
        .unwrap();

        for reg in (0..6).rev() {
            self.sp -= 2;
            *self.decode_register(reg) = match self.map.read(self.sp as u32) {
                DeviceMapResult::Ok(val) => val,
                DeviceMapResult::NoDevices => {
                    panic!("No devices attached. Could not read any values.")
                }
                DeviceMapResult::Error(err) => {
                    if err == DeviceResponse::WriteOnly {
                        panic!("Device write only. Could not read value.");
                    } else {
                        panic!("Unknown error. Could not read value.");
                    }
                }
            };
        }
    }

    pub fn dump(&self, dump_type: Dump) {
        match dump_type {
            Dump::All => {
                self.dump_memory();
                self.dump_stats();
            }
            Dump::Memory => self.dump_memory(),
            Dump::Stats => self.dump_stats(),
        }
    }

    fn dump_stats(&self) {
        macro_rules! flag_value {
            ($flag_val:expr) => {
                if $flag_val {
                    "On"
                } else {
                    "Off"
                }
            };
        }

        fs::write(
            "debug/dump.txt",
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
    DWord: {}",
                self.pc,
                self.sp,
                self.r1,
                self.r2,
                self.r3,
                self.r4,
                self.r5,
                self.r6,
                flag_value!(self.flags.contains(Flags::Z)),
                flag_value!(self.flags.contains(Flags::G)),
                flag_value!(self.flags.contains(Flags::L)),
                flag_value!(self.flags.contains(Flags::O)),
                flag_value!(self.flags.contains(Flags::D)),
            ),
        )
        .unwrap();
    }

    fn dump_memory(&self) {
        for device in &self.map.devices {
            let dev_lock = device.lock().unwrap();
            fs::write(
                format!("debug/memory/{}.bin", dev_lock.get_name()),
                dev_lock.get_memory(),
            )
            .unwrap();
        }
    }
}
