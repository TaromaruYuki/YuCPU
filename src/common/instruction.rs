pub struct Instruction {
    pub opcode: u8,
    pub register: u8,
    pub data: u16,
}

impl Instruction {
    pub fn new(opcode: u8, register: u8, data: u16) -> Instruction {
        Instruction {
            opcode,
            register,
            data,
        }
    }

    pub fn new_u8(opcode: u8, register: u8, data_1: u8, data_2: u8) -> Instruction {
        Instruction {
            opcode,
            register,
            data: (((data_1 as u16) << 8) | data_2 as u16),
        }
    }
}
