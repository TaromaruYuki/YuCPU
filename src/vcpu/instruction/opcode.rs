#![allow(clippy::unusual_byte_groupings)]

use std::collections::HashMap;

use super::instructions::*;

pub type InstructionMap = HashMap<u8, InstructionInfo>;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate = 0b00,
    Register = 0b01,
    Direct = 0b010,
    Discard = 0b11,
}

pub struct Instruction {
    pub opcode: Opcode,
    pub mode: AddressingMode,
    pub exec: InstructionFunction,
}

#[derive(Debug)]
pub enum InstructionError {
    InvalidOpcode,
}

pub type InstructionResult = Result<Instruction, InstructionError>;

impl Instruction {
    pub fn hashmap() -> InstructionMap {
        let mut map: InstructionMap = HashMap::new();
        map.insert(
            0b00_000000,
            (Opcode::MOV, AddressingMode::Immediate, mov_immediate),
        );
        map.insert(
            0b01_000000,
            (Opcode::MOV, AddressingMode::Register, mov_register),
        );

        map.insert(
            0b01_000001,
            (Opcode::LD, AddressingMode::Register, ld_register),
        );
        map.insert(
            0b10_000001,
            (Opcode::LD, AddressingMode::Direct, ld_address),
        );

        map.insert(
            0b01_000010,
            (Opcode::LDB, AddressingMode::Register, ldb_register),
        );
        map.insert(
            0b10_000010,
            (Opcode::LDB, AddressingMode::Direct, ldb_address),
        );

        map.insert(
            0b00_000011,
            (Opcode::PSH, AddressingMode::Immediate, psh_immediate),
        );
        map.insert(
            0b01_000011,
            (Opcode::PSH, AddressingMode::Register, psh_register),
        );
        map.insert(
            0b10_000011,
            (Opcode::PSH, AddressingMode::Direct, psh_address),
        );

        map.insert(
            0b01_000100,
            (Opcode::POP, AddressingMode::Register, pop_register),
        );
        map.insert(0b11_000100, (Opcode::POP, AddressingMode::Discard, pop));

        map.insert(
            0b01_000101,
            (Opcode::ST, AddressingMode::Register, st_register),
        );
        map.insert(
            0b10_000101,
            (Opcode::ST, AddressingMode::Direct, st_address),
        );

        map.insert(
            0b01_000110,
            (Opcode::STL, AddressingMode::Register, stl_register),
        );
        map.insert(
            0b10_000110,
            (Opcode::STL, AddressingMode::Direct, stl_address),
        );

        map.insert(
            0b01_000111,
            (Opcode::STH, AddressingMode::Register, sth_register),
        );
        map.insert(
            0b10_000111,
            (Opcode::STH, AddressingMode::Direct, sth_address),
        );

        map.insert(
            0b00_001000,
            (Opcode::CMP, AddressingMode::Immediate, cmp_immediate),
        );
        map.insert(
            0b01_001000,
            (Opcode::CMP, AddressingMode::Register, cmp_register),
        );

        map.insert(0b10_001001, (Opcode::BEQ, AddressingMode::Direct, beq));
        map.insert(0b10_001010, (Opcode::BGT, AddressingMode::Direct, bgt));
        map.insert(0b10_001011, (Opcode::BLT, AddressingMode::Direct, blt));
        map.insert(0b10_001100, (Opcode::BOF, AddressingMode::Direct, bof));
        map.insert(0b10_001101, (Opcode::BNE, AddressingMode::Direct, bne));

        map.insert(0b10_001110, (Opcode::JMP, AddressingMode::Direct, jmp));
        map.insert(0b10_001111, (Opcode::JSR, AddressingMode::Direct, jsr));

        map.insert(
            0b00_010000,
            (Opcode::ADD, AddressingMode::Immediate, add_direct),
        );
        map.insert(
            0b01_010000,
            (Opcode::ADD, AddressingMode::Register, add_register),
        );

        map.insert(
            0b00_010001,
            (Opcode::SUB, AddressingMode::Immediate, sub_direct),
        );
        map.insert(
            0b01_010010,
            (Opcode::SUB, AddressingMode::Register, sub_register),
        );

        map.insert(0b11_010011, (Opcode::RET, AddressingMode::Discard, ret));

        map.insert(0b11_011110, (Opcode::HLT, AddressingMode::Discard, hlt));
        map.insert(0b11_011111, (Opcode::NOP, AddressingMode::Discard, nop));

        map
    }

    // pub fn new(
    //     opcode: Opcode,
    //     mode: AddressingMode,
    //     exec: InstructionFunction,
    // ) -> InstructionResult {
    //     Ok(Instruction { opcode, mode, exec })
    // }

    pub fn from_opcode(opcode: &u8) -> InstructionResult {
        let binding = Instruction::hashmap();
        let result: &InstructionInfo = match binding.get(opcode) {
            Some(val) => val,
            None => return Err(InstructionError::InvalidOpcode),
        };

        Ok(Instruction {
            opcode: result.0,
            mode: result.1,
            exec: result.2,
        })
    }
}
