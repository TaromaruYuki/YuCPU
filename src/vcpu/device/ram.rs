use super::{Device, DeviceResponse};

pub struct Ram {
    pub memory: Vec<u8>,
    start: u32,
    end: u32,
    name: String
}

impl Ram {
    pub fn new(start: u32, end: u32) -> Ram {
        Ram {
            memory: vec![0; (end - start) as usize],
            start,
            end,
            name: String::from("RAM"),
        }
    }

    fn relative(&self, addr: u32) -> usize {
        // println!("{:x} is {:x}", addr, (addr - self.start));
        (addr - self.start) as usize
    }
}

impl Device for Ram {
    fn read(&self, addr: u32) -> DeviceResponse<u16> {
        if addr >= self.start && addr <= self.end {
            let data1 = (self.memory[self.relative(addr)] as u16) << 8;
            let data2 = self.memory[self.relative(addr + 1)] as u16;

            return DeviceResponse::Ok(data1 | data2);
        }

        DeviceResponse::NotMyAddress
    }

    fn read_byte(&self, addr: u32) -> DeviceResponse<u8> {
        if addr >= self.start && addr <= self.end {
            return DeviceResponse::Ok(self.memory[self.relative(addr)]);
        }

        DeviceResponse::NotMyAddress
    }

    fn write(&mut self, addr: u32, value: u16) -> DeviceResponse<()> {
        if addr >= self.start && addr <= self.end {
            let val1 = (value >> 8) as u8;
            let val2 = value as u8;
            let addr1 = self.relative(addr);
            let addr2 = self.relative(addr + 1);

            self.memory[addr1] = val1;
            self.memory[addr2] = val2;

            return DeviceResponse::Ok(());
        }

        DeviceResponse::NotMyAddress
    }

    fn write_byte(&mut self, addr: u32, value: u8) -> DeviceResponse<()> {
        if addr >= self.start && addr <= self.end {
            let addr = self.relative(addr);
            self.memory[addr] = value;

            return DeviceResponse::Ok(());
        }

        DeviceResponse::NotMyAddress
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_memory(&self) -> Vec<u8> {
        self.memory.clone()
    }
}
