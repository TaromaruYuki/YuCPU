use std::collections::HashMap;

use super::instructions::*;

pub type InstructionMap = HashMap<u8, InstructionInfo>;

#[derive(Clone, Copy)]
pub enum Opcode {
    ADD,
    BEQ,
    BGT,
    BLT,
    BOF,
    BNE,
    CMP,
    HLT,
    JMP,
    JSR,
    LD,
    LDB,
    MOV,
    NOP,
    POP,
    PSH,
    RET,
    ST,
    STL,
    STH,
    SUB,
}

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Immediate = 0b000,
    Register = 0b001,
    Indexed = 0b010,
    RegisterIndexed = 0b011,
    Direct = 0b100,
    Discard = 0b101,
}

pub struct Instruction {
    pub opcode: Opcode,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub exec: InstructionFunction,
}

pub enum InstructionError {
    InvalidOpcode,
}

pub type InstructionResult = Result<Instruction, InstructionError>;

impl Instruction {
    pub fn hashmap() -> InstructionMap {
        let mut map: InstructionMap = HashMap::new();
        map.insert(
            0b000_00000,
            (Opcode::MOV, AddressingMode::Immediate, mov_immediate),
        );
        map.insert(
            0b001_00000,
            (Opcode::MOV, AddressingMode::Register, mov_register),
        );

        map.insert(
            0b001_00001,
            (Opcode::LD, AddressingMode::Register, ld_register),
        );
        map.insert(
            0b100_00001,
            (Opcode::LD, AddressingMode::Direct, ld_address),
        );

        map.insert(
            0b001_00010,
            (Opcode::LDB, AddressingMode::Register, ldb_register),
        );
        map.insert(
            0b100_00010,
            (Opcode::LDB, AddressingMode::Direct, ldb_address),
        );

        map.insert(
            0b000_00011,
            (Opcode::PSH, AddressingMode::Immediate, psh_immediate),
        );
        map.insert(
            0b001_00011,
            (Opcode::PSH, AddressingMode::Register, psh_register),
        );
        map.insert(
            0b100_00011,
            (Opcode::PSH, AddressingMode::Direct, psh_address),
        );

        map.insert(
            0b001_00100,
            (Opcode::POP, AddressingMode::Register, pop_register),
        );
        map.insert(0b101_00100, (Opcode::POP, AddressingMode::Discard, pop));

        map.insert(
            0b001_00101,
            (Opcode::ST, AddressingMode::Register, st_register),
        );
        map.insert(
            0b100_00101,
            (Opcode::ST, AddressingMode::Direct, st_address),
        );

        map.insert(
            0b001_00110,
            (Opcode::STL, AddressingMode::Register, stl_register),
        );
        map.insert(
            0b100_00110,
            (Opcode::STL, AddressingMode::Direct, stl_address),
        );

        map.insert(
            0b001_00111,
            (Opcode::STH, AddressingMode::Register, sth_register),
        );
        map.insert(
            0b100_00111,
            (Opcode::STH, AddressingMode::Direct, sth_address),
        );

        map.insert(
            0b000_01000,
            (Opcode::CMP, AddressingMode::Immediate, cmp_immediate),
        );
        map.insert(
            0b001_01000,
            (Opcode::CMP, AddressingMode::Register, cmp_register),
        );

        map.insert(0b100_01001, (Opcode::BEQ, AddressingMode::Direct, beq));
        map.insert(0b100_01010, (Opcode::BGT, AddressingMode::Direct, bgt));
        map.insert(0b100_01011, (Opcode::BLT, AddressingMode::Direct, blt));
        map.insert(0b100_01100, (Opcode::BOF, AddressingMode::Direct, bof));
        map.insert(0b100_01101, (Opcode::BNE, AddressingMode::Direct, bne));

        map.insert(0b100_01110, (Opcode::JMP, AddressingMode::Direct, jmp));
        map.insert(0b100_01111, (Opcode::JSR, AddressingMode::Direct, jsr));

        map.insert(
            0b000_10000,
            (Opcode::ADD, AddressingMode::Immediate, add_direct),
        );
        map.insert(
            0b001_10000,
            (Opcode::ADD, AddressingMode::Register, add_register),
        );

        map.insert(
            0b000_10001,
            (Opcode::SUB, AddressingMode::Immediate, sub_direct),
        );
        map.insert(
            0b001_10001,
            (Opcode::SUB, AddressingMode::Register, sub_register),
        );

        map.insert(0b101_10010, (Opcode::RET, AddressingMode::Discard, ret));

        map.insert(0b101_11110, (Opcode::HLT, AddressingMode::Discard, hlt));
        map.insert(0b101_11111, (Opcode::NOP, AddressingMode::Discard, nop));

        map
    }

    pub fn new(
        opcode: Opcode,
        mode: AddressingMode,
        cycles: u8,
        exec: InstructionFunction,
    ) -> InstructionResult {
        Ok(Instruction {
            opcode,
            mode,
            cycles,
            exec,
        })
    }

    pub fn from_opcode(opcode: &u8) -> InstructionResult {
        let binding = Instruction::hashmap();
        let result: &InstructionInfo = match binding.get(&opcode) {
            Some(val) => val,
            None => return Err(InstructionError::InvalidOpcode),
        };

        Ok(Instruction {
            opcode: result.0,
            mode: result.1,
            cycles: 0,
            exec: result.2,
        })
    }
}
