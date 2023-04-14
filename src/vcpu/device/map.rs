use super::{Device, DeviceResponse};

pub enum DeviceMapResult<T> {
    Ok(T),
    NoDevices,
    Error(DeviceResponse<()>),
}

pub struct DeviceMap {
    pub devices: Vec<Box<dyn Device>>,
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
            let res = match device.read(addr) {
                DeviceResponse::Ok(val) => val,
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    panic!("Something went wrong with device {}.\nRead only received on a read action.", device.get_name());
                }
                DeviceResponse::WriteOnly => {
                    println!("Device is write only.");
                    return DeviceMapResult::Error(DeviceResponse::WriteOnly);
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        device.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(res);
        }

        DeviceMapResult::NoDevices
    }

    pub fn read_byte(&mut self, addr: u32) -> DeviceMapResult<u8> {
        for device in &mut self.devices {
            let res = match device.read_byte(addr) {
                DeviceResponse::Ok(val) => val,
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    panic!("Something went wrong with device {}.\nRead only received on a read action.", device.get_name());
                }
                DeviceResponse::WriteOnly => {
                    println!("Device is write only.");
                    return DeviceMapResult::Error(DeviceResponse::WriteOnly);
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        device.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(res);
        }

        DeviceMapResult::NoDevices
    }

    pub fn write(&mut self, addr: u32, value: u16) -> DeviceMapResult<()> {
        println!("Writing to address {}", addr);
        for device in &mut self.devices {
            match device.write(addr, value) {
                DeviceResponse::Ok(_) => (),
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    return DeviceMapResult::Error(DeviceResponse::ReadOnly)
                }
                DeviceResponse::WriteOnly => {
                    panic!("Something went wrong with device {}.\nWrite only received on a write action.", device.get_name());
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        device.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(());
        }

        DeviceMapResult::NoDevices
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) -> DeviceMapResult<()> {
        for device in &mut self.devices {
            match device.write_byte(addr, value) {
                DeviceResponse::Ok(_) => (),
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    return DeviceMapResult::Error(DeviceResponse::ReadOnly);
                }
                DeviceResponse::WriteOnly => {
                    panic!("Something went wrong with device {}.\nWrite only received on a write action.", device.get_name());
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        device.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(());
        }

        DeviceMapResult::NoDevices
    }
}

impl Default for DeviceMap {
    fn default() -> Self {
        Self::new()
    }
}
