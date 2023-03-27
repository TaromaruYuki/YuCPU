#![allow(unused_assignments)]

pub mod cpu;
pub mod device;
pub mod instruction;

pub fn run(program: Vec<u8>) {
    println!("{:?}", program);
    let rom = device::rom::Rom::new(program, 0x0000);
    let ram = device::ram::Ram::new(0x8000, 0xFFFF);
    let mut map = device::map::DeviceMap::new();

    map.add(ram);
    map.add(rom);

    let mut cpu = cpu::CPU::new(map, 0x0000);
    cpu.sp = 0x8000;
    let mut pins = cpu.pins;

    while cpu.running {
        pins = cpu.tick(pins);
    }
}
