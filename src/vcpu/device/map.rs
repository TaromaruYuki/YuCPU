use std::sync::{Arc, Mutex};

use super::{Device, DeviceResponse};

pub enum DeviceMapResult<T> {
    Ok(T),
    NoDevices,
    Error(DeviceResponse<()>),
}

pub struct DeviceMap {
    pub devices: Vec<Arc<Mutex<dyn Device>>>,
}

impl DeviceMap {
    pub fn new() -> DeviceMap {
        DeviceMap {
            devices: Vec::new(),
        }
    }

    pub fn add<T: 'static + Device>(&mut self, device: Arc<Mutex<T>>) {
        self.devices.push(device);
    }

    pub fn read(&mut self, addr: u32) -> DeviceMapResult<u16> {
        for device in &mut self.devices {
            let dev = device.lock().unwrap();
            let res = match dev.read(addr) {
                DeviceResponse::Ok(val) => val,
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    panic!("Something went wrong with device {}.\nRead only received on a read action.", dev.get_name());
                }
                DeviceResponse::WriteOnly => {
                    println!("Device is write only.");
                    return DeviceMapResult::Error(DeviceResponse::WriteOnly);
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        dev.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(res);
        }

        DeviceMapResult::NoDevices
    }

    pub fn read_byte(&mut self, addr: u32) -> DeviceMapResult<u8> {
        for device in &mut self.devices {
            let dev = device.lock().unwrap();
            let res = match dev.read_byte(addr) {
                DeviceResponse::Ok(val) => val,
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    panic!("Something went wrong with device {}.\nRead only received on a read action.", dev.get_name());
                }
                DeviceResponse::WriteOnly => {
                    println!("Device is write only.");
                    return DeviceMapResult::Error(DeviceResponse::WriteOnly);
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        dev.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(res);
        }

        DeviceMapResult::NoDevices
    }

    pub fn write(&mut self, addr: u32, value: u16) -> DeviceMapResult<()> {
        for device in &mut self.devices {
            let mut dev = device.lock().unwrap();
            match dev.write(addr, value) {
                DeviceResponse::Ok(_) => (),
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    return DeviceMapResult::Error(DeviceResponse::ReadOnly)
                }
                DeviceResponse::WriteOnly => {
                    panic!("Something went wrong with device {}.\nWrite only received on a write action.", dev.get_name());
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        dev.get_name()
                    );
                }
            };

            return DeviceMapResult::Ok(());
        }

        DeviceMapResult::NoDevices
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) -> DeviceMapResult<()> {
        for device in &mut self.devices {
            let mut dev = device.lock().unwrap();
            match dev.write_byte(addr, value) {
                DeviceResponse::Ok(_) => (),
                DeviceResponse::NotMyAddress => continue,
                DeviceResponse::ReadOnly => {
                    return DeviceMapResult::Error(DeviceResponse::ReadOnly);
                }
                DeviceResponse::WriteOnly => {
                    panic!("Something went wrong with device {}.\nWrite only received on a write action.", dev.get_name());
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!(
                        "Something wrong wrong with device {}.\nA unknown error received.",
                        dev.get_name()
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
