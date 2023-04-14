pub mod map;
pub mod ram;
pub mod rom;
pub mod vga;

#[derive(PartialEq, Eq, Debug)]
pub enum DeviceResponse<T> {
    Ok(T),
    NotMyAddress,
    ReadOnly,
    WriteOnly,
    InvalidAddress,
}

pub trait Device {
    fn read(&self, addr: u32) -> DeviceResponse<u16>;
    fn read_byte(&self, addr: u32) -> DeviceResponse<u8>;
    fn write(&mut self, addr: u32, value: u16) -> DeviceResponse<()>;
    fn write_byte(&mut self, addr: u32, value: u8) -> DeviceResponse<()>;
    fn get_name(&self) -> String;
    fn get_memory(&self) -> &Vec<u8>;
}
