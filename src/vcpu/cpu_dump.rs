use super::{bus::DataBus, registers::Registers};

pub struct CPUDump {
    pub registers: Registers,
    pub stack: Vec<u8>,
    pub memory: DataBus,
}

impl CPUDump {
    pub fn new(registers: &Registers, stack: Vec<u8>, memory: &DataBus) -> CPUDump {
        CPUDump {
            registers: registers.clone(),
            stack,
            memory: memory.clone(),
        }
    }
}
