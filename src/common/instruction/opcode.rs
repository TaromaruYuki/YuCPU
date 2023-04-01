#![allow(clippy::unusual_byte_groupings)]

use std::collections::HashMap;

use super::instructions::*;

use core::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

pub type InstructionMap = HashMap<u8, InstructionInfo>;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Opcode {
    MOV = 0b000000,
    LD = 0b000001,
    LDB = 0b000010,
    PSH = 0b000011,
    POP = 0b000100,
    ST = 0b000101,
    STL = 0b000110,
    STH = 0b000111,
    CMP = 0b001000,
    BEQ = 0b001001,
    BGT = 0b001010,
    BLT = 0b001011,
    BOF = 0b001100,
    BNE = 0b001101,
    JMP = 0b001110,
    JSR = 0b001111,
    ADD = 0b010000,
    SUB = 0b010001,
    RET = 0b010010,
    HLT = 0b111110,
    NOP = 0b111111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
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

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("opcode", &self.opcode)
            .field("mode", &self.mode)
            .finish()
    }
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
            Self::create_opcode(Opcode::MOV, AddressingMode::Immediate),
            (Opcode::MOV, AddressingMode::Immediate, mov_immediate, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::MOV, AddressingMode::Register),
            (Opcode::MOV, AddressingMode::Register, mov_register, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::LD, AddressingMode::Register),
            (Opcode::LD, AddressingMode::Register, ld_register, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::LD, AddressingMode::Direct),
            (Opcode::LD, AddressingMode::Direct, ld_address, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::LDB, AddressingMode::Register),
            (Opcode::LDB, AddressingMode::Register, ldb_register, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::LDB, AddressingMode::Direct),
            (Opcode::LDB, AddressingMode::Direct, ldb_address, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::PSH, AddressingMode::Immediate),
            (Opcode::PSH, AddressingMode::Immediate, psh_immediate, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::PSH, AddressingMode::Register),
            (Opcode::PSH, AddressingMode::Register, psh_register, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::PSH, AddressingMode::Direct),
            (Opcode::PSH, AddressingMode::Direct, psh_address, 1),
        );

        map.insert(
            Self::create_opcode(Opcode::POP, AddressingMode::Register),
            (Opcode::POP, AddressingMode::Register, pop_register, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::POP, AddressingMode::Discard),
            (Opcode::POP, AddressingMode::Discard, pop, 0),
        );

        map.insert(
            Self::create_opcode(Opcode::ST, AddressingMode::Register),
            (Opcode::ST, AddressingMode::Register, st_register, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::ST, AddressingMode::Direct),
            (Opcode::ST, AddressingMode::Direct, st_address, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::STL, AddressingMode::Register),
            (Opcode::STL, AddressingMode::Register, stl_register, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::STL, AddressingMode::Direct),
            (Opcode::STL, AddressingMode::Direct, stl_address, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::STH, AddressingMode::Register),
            (Opcode::STH, AddressingMode::Register, sth_register, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::STH, AddressingMode::Direct),
            (Opcode::STH, AddressingMode::Direct, sth_address, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::CMP, AddressingMode::Immediate),
            (Opcode::CMP, AddressingMode::Immediate, cmp_immediate, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::CMP, AddressingMode::Register),
            (Opcode::CMP, AddressingMode::Register, cmp_register, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::BEQ, AddressingMode::Direct),
            (Opcode::BEQ, AddressingMode::Direct, beq, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::BGT, AddressingMode::Direct),
            (Opcode::BGT, AddressingMode::Direct, bgt, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::BLT, AddressingMode::Direct),
            (Opcode::BLT, AddressingMode::Direct, blt, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::BOF, AddressingMode::Direct),
            (Opcode::BOF, AddressingMode::Direct, bof, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::BNE, AddressingMode::Direct),
            (Opcode::BNE, AddressingMode::Direct, bne, 1),
        );

        map.insert(
            Self::create_opcode(Opcode::JMP, AddressingMode::Direct),
            (Opcode::JMP, AddressingMode::Direct, jmp, 1),
        );
        map.insert(
            Self::create_opcode(Opcode::JSR, AddressingMode::Direct),
            (Opcode::JSR, AddressingMode::Direct, jsr, 1),
        );

        map.insert(
            Self::create_opcode(Opcode::ADD, AddressingMode::Immediate),
            (Opcode::ADD, AddressingMode::Immediate, add_immediate, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::ADD, AddressingMode::Register),
            (Opcode::ADD, AddressingMode::Register, add_register, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::SUB, AddressingMode::Immediate),
            (Opcode::SUB, AddressingMode::Immediate, sub_immediate, 2),
        );
        map.insert(
            Self::create_opcode(Opcode::SUB, AddressingMode::Register),
            (Opcode::SUB, AddressingMode::Register, sub_register, 2),
        );

        map.insert(
            Self::create_opcode(Opcode::RET, AddressingMode::Discard),
            (Opcode::RET, AddressingMode::Discard, ret, 0),
        );

        map.insert(
            Self::create_opcode(Opcode::HLT, AddressingMode::Discard),
            (Opcode::HLT, AddressingMode::Discard, hlt, 0),
        );
        map.insert(
            Self::create_opcode(Opcode::NOP, AddressingMode::Discard),
            (Opcode::NOP, AddressingMode::Discard, nop, 0),
        );

        map
    }

    pub fn get_variants(opcode: Opcode) -> Vec<AddressingMode> {
        let map = Instruction::hashmap();
        let mut res: Vec<AddressingMode> = Vec::new();

        for mode in AddressingMode::iter() {
            let code = map.get(&Instruction::create_opcode(opcode, mode));

            if code.is_some() {
                res.push(mode);
            }
        }

        res
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

    pub fn create_opcode(opcode: Opcode, mode: AddressingMode) -> u8 {
        let num_code = opcode as u8;
        let mode_code = mode as u8;

        (mode_code << 6) | num_code
    }
}
