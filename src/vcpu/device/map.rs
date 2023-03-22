use super::{Device, DeviceResponse};

pub enum DeviceMapResult<T> {
    Ok(T),
    NoDevices,
}

pub struct DeviceMap {
    devices: Vec<Box<dyn Device>>,
}

impl DeviceMap {
    pub fn new() -> DeviceMap {
        DeviceMap {
            devices: Vec::new(),
        }
    }

    pub fn add<T: 'static + Device>(&mut self, device: T) {
        self.devices.push(Box::new(device));
    }

    pub fn read(&mut self, addr: u32) -> DeviceMapResult<u16> {
        for device in &mut self.devices {
            println!("!! Checking device {}", device.get_name());
            let res = match device.read(addr) {
                DeviceResponse::Ok(val) => {
                    println!("Got a value");
                    val
                }
                DeviceResponse::NotMyAddress => {
                    println!("Not my address");
                    continue;
                }
            };

            return DeviceMapResult::Ok(res);
        }

        DeviceMapResult::NoDevices
    }

    pub fn read_byte(&mut self, addr: u32) -> DeviceMapResult<u8> {
        for device in &mut self.devices {
            println!("!! Checking device {}", device.get_name());
            let res = match device.read_byte(addr) {
                DeviceResponse::Ok(val) => {
                    println!("Got a value");
                    val
                }
                DeviceResponse::NotMyAddress => {
                    println!("Not my address");
                    continue;
                }
            };

            return DeviceMapResult::Ok(res);
        }

        DeviceMapResult::NoDevices
    }
}
