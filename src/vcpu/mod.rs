#![allow(unused_assignments)]

pub mod cpu;
pub mod device;
pub mod instruction;

pub fn run(program: Vec<u8>) {
    let ram = device::ram::Ram::new(0x0000, 0x7FFF);
    let rom = device::rom::Rom::new(program, 0x8000);
    let mut map = device::map::DeviceMap::new();

    map.add(ram);
    map.add(rom);

    let mut cpu = cpu::CPU::new(map);
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);
}
