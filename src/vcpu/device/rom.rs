use super::{Device, DeviceResponse};

pub struct Rom {
    pub memory: Vec<u8>,
    start: u32,
    end: u32,
}

impl Rom {
    pub fn new(memory: Vec<u8>, start: u32) -> Rom {
        Rom {
            start,
            end: (memory.len() as u32) + start - 1,
            memory,
        }
    }

    fn relative(&self, addr: u32) -> usize {
        (addr - self.start) as usize
    }
}

impl Device for Rom {
    fn read(&mut self, addr: u32) -> DeviceResponse<u16> {
        if addr >= self.start && addr <= self.end {
            let data1 = (self.memory[self.relative(addr)] as u16) << 8;
            let data2 = self.memory[self.relative(addr + 1)] as u16;
            return DeviceResponse::Ok(data1 | data2);
        }

        DeviceResponse::NotMyAddress
    }

    fn read_byte(&mut self, addr: u32) -> DeviceResponse<u8> {
        if addr >= self.start && addr <= self.end {
            return DeviceResponse::Ok(self.memory[self.relative(addr)]);
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
        String::from("ROM")
    }

    fn get_memory(&self) -> &Vec<u8> {
        &self.memory
    }
}
