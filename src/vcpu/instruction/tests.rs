use crate::vcpu::instruction::opcode::{AddressingMode, Opcode};

use super::opcode::Instruction;

#[test]
fn test_from_opcode_instruction() {
    let result = match Instruction::from_opcode(&(0b000_00000 as u8)) {
        Ok(res) => res,
        Err(_) => panic!("Opcode does not exist."),
    };

    let result2 = match Instruction::from_opcode(&(0b100_01111 as u8)) {
        Ok(res) => res,
        Err(_) => panic!("Opcode does not exist."),
    };

    assert_eq!(result.opcode, Opcode::MOV);
    assert_eq!(result.mode, AddressingMode::Immediate);

    assert_eq!(result2.opcode, Opcode::JSR);
    assert_eq!(result2.mode, AddressingMode::Direct);
}

#[test]
#[should_panic]
fn test_from_opcode_instruction_fail() {
    match Instruction::from_opcode(&(0b101_00000 as u8)) {
        Ok(res) => res,
        Err(_) => panic!("Opcode does not exist."),
    };
}
