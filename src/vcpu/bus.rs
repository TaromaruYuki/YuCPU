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

pub enum DataBusError {
    OutOfRange,
}

type DataBusResult = Result<u8, DataBusError>;

pub struct DataBus {
    memory: Vec<u8>,
}

impl DataBus {
    pub fn new() -> DataBus {
        DataBus {
            memory: vec![0; 0xFFDD],
        }
    }
}

impl DataBus {
    fn read(self, addr: u16) -> DataBusResult {
        if addr > 0xFFDD {
            return Err(DataBusError::OutOfRange);
        }

        Ok(self.memory[addr as usize])
    }

    fn write(mut self, addr: u16, value: u8) -> DataBusResult {
        if addr > 0xFFDD {
            return Err(DataBusError::OutOfRange);
        }

        self.memory[addr as usize] = value;

        Ok(0)
    }

    fn reset(mut self) {
        self.memory = vec![0; 0xFFDD]
    }

    fn mem_copy(mut self, start_addr: u16, mem: &[u8]) {
        // TODO: Find a more efficient way to do this
        for (i, byte) in mem.iter().enumerate() {
            self.memory[(start_addr as usize) + i] = *byte;
        }
    }
}
