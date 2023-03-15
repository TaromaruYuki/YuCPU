#[derive(Clone)]
pub struct Registers {
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub flags: u8,
    pub pc: u16,
    pub sp: u16,
    pub bp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r1: 0x0000,
            r2: 0x0000,
            r3: 0x0000,
            r4: 0x0000,
            r5: 0x0000,
            r6: 0x0000,
            flags: 0x00,
            pc: 0x0000,
            sp: 0x0000,
            bp: 0x0000,
        }
    }
}
