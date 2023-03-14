#![allow(unused)]

/*
 *  +--------------------------------------------+
 *  |               Memory Layout                |
 *  +--------------------------------------------+
 *  | 0x00 - 0x1F40        | Stack               |
 *  | 0x1F41 - 0xDAC0      | General Purpose     |
 *  | 0xDAC1 - 0xFFDD      | Program Memory      |
 *  +--------------------------------------------+
 */

use std::{fs, process::exit};

use crate::common::hex::{Hex, *};

pub enum DataBusError {
    OutOfRange,
}

type DataBusResult = Result<Option<u8>, DataBusError>;
type DataBusShortResult = Result<Option<u16>, DataBusError>;

#[derive(Clone)]
pub struct DataBus {
    pub memory: Vec<u8>,
}

impl DataBus {
    pub fn new() -> DataBus {
        DataBus {
            memory: vec![0; 0xFFDD],
        }
    }
}

impl DataBus {
    pub fn read(&self, addr: u16) -> DataBusResult {
        if addr > 0xFFDD {
            return Err(DataBusError::OutOfRange);
        }

        Ok(Some(self.memory[addr as usize]))
    }

    pub fn read_panic(&self, addr: u16) -> u8 {
        match self.read(addr) {
            Ok(val) => val.unwrap(),
            Err(_) => {
                panic!("Address out of range")
            }
        }
    }

    pub fn read_short(&self, addr: u16) -> DataBusShortResult {
        if addr > 0xFFDD {
            return Err(DataBusError::OutOfRange);
        }

        let data_1 = self.memory[addr as usize];
        let data_2 = self.memory[(addr + 1) as usize];

        Ok(Some(((data_1 as u16) << 8) | data_2 as u16))
    }

    pub fn read_short_panic(&self, addr: u16) -> u16 {
        match self.read_short(addr) {
            Ok(val) => val.unwrap(),
            Err(_) => {
                panic!("Address out of range")
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: &u8) -> DataBusResult {
        if addr > 0xFFDD {
            return Err(DataBusError::OutOfRange);
        }

        self.memory[addr as usize] = *value;

        Ok(None)
    }

    pub fn write_panic(&mut self, addr: u16, value: &u8) {
        match self.write(addr, value) {
            Ok(val) => (),
            Err(_) => {
                panic!("Address out of range")
            }
        };
    }

    pub fn reset(&mut self) {
        self.memory = vec![0; 0xFFDD]
    }

    pub fn mem_copy(&mut self, start_addr: u16, mem: &[u8]) {
        // TODO: Find a more efficient way to do this
        for (i, byte) in mem.iter().enumerate() {
            self.memory[(start_addr as usize) + i] = *byte;
        }
    }

    pub fn dump(&self) {
        // self.memory.to_hex_string();

        match fs::write("memory.bin", &self.memory) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to write output file.\n{error}");
                exit(1);
            }
        };
    }
}
