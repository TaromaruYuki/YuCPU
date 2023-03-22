pub mod map;
pub mod ram;
pub mod rom;

pub enum DeviceResponse<T> {
    Ok(T),
    NotMyAddress,
}

pub trait Device {
    fn read(&mut self, addr: u32) -> DeviceResponse<u16>;
    fn read_byte(&mut self, addr: u32) -> DeviceResponse<u8>;
    fn write(&mut self, addr: u32, value: u16) -> DeviceResponse<u16>;
    fn get_name(&self) -> String;
}
