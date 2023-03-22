use super::{Device, DeviceResponse};

pub struct Ram {
    pub memory: Vec<u8>,
    start: u32,
    end: u32,
}

impl Ram {
    pub fn new(start: u32, end: u32) -> Ram {
        Ram {
            memory: vec![0; (end - start) as usize],
            start,
            end,
        }
    }

    fn relative(&self, addr: u32) -> usize {
        (addr - self.start) as usize
    }
}

impl Device for Ram {
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

    fn write(&mut self, addr: u32, value: u16) -> DeviceResponse<u16> {
        if addr >= self.start && addr <= self.end {
            let val1 = (value >> 8) as u8;
            let val2 = value as u8;
            let addr1 = self.relative(addr);
            let addr2 = self.relative(addr + 1);

            self.memory[addr1] = val1;
            self.memory[addr2] = val2;

            return DeviceResponse::Ok(value);
        }

        DeviceResponse::NotMyAddress
    }

    fn get_name(&self) -> String {
        String::from("Ram")
    }
}
