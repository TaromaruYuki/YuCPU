use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Flags: u32 {
        const Z = 0b00000001;
        const O = 0b00000010;
        const L = 0b01000000;
        const G = 0b00100000;
        const FLAGS = Self::Z.bits() | Self::O.bits() | Self::L.bits() | Self::G.bits();
    }
}

pub enum ReadWrite {
    Read,
    Write,
}

pub struct Pins {
    pub data: u16,
    pub address: u32, // We will only use 20 bits out of the 32. 0b0000_00000000_00000000
    pub rw: ReadWrite,
}

pub struct CPU {
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub sp: u16,
    pub pc: u16,
    pub bp: u16,
    pub flags: Flags,
    ir: u16,
}

// Public Code
impl CPU {
    #[allow(dead_code)]
    pub fn new() -> CPU {
        CPU {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            sp: 0,
            pc: 0,
            bp: 0,
            flags: Flags::empty(),
            ir: 0,
        }
    }
}

impl CPU {
    fn tick() {
        todo!();
    }
}
