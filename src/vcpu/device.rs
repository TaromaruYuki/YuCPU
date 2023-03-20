pub enum DeviceResponse<T> {
    Ok(T),
    NotMyAddress,
}

pub trait Device {
    fn read(&mut self, addr: u32) -> DeviceResponse<u16>;
    fn write(&mut self, addr: u32) -> DeviceResponse<u16>;
}
