pub mod parser;
pub mod tokenizer;

use crate::common::instruction::opcode::Instruction;

use self::parser::{InstructionArg, InstructionType, Label, ParserResult};

pub struct Assembler {
    parser_res: ParserResult,
}

impl Assembler {
    pub fn new(parser_res: ParserResult) -> Assembler {
        // println!("Parser result: {:?}", parser_res);
        Assembler { parser_res }
    }

    fn find_label(name: &String, labels: &Vec<Label>) -> Option<Label> {
        for label in labels {
            if &label.name == name {
                return Some(label.clone());
            }
        }

        None
    }

    pub fn assemble(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        for label in &self.parser_res.labels {
            // println!("Label \"{}\" @ addr $0x{:05x}", label.name, label.addr);
            for instruction in &label.instructions {
                let opcode =
                    Instruction::create_opcode(instruction.opcode, instruction.addressing_mode);
                output.push(opcode);

                let mut meta: u8 = 0x00;

                match instruction.instruction_type {
                    InstructionType::Zero => {
                        meta |= 0b0000_1100;
                        output.push(meta);
                    }
                    InstructionType::One => match &instruction.args[0] {
                        InstructionArg::Register(reg) => {
                            meta |= reg << 4;
                            meta |= 0b0000_1100;
                            output.push(meta);
                        }
                        InstructionArg::Number(num) => {
                            if num > &255 {
                                meta |= 0b0000_0100;
                                output.push(meta);
                                output.push((num >> 8) as u8);
                                output.push(*num as u8);
                            } else {
                                output.push(meta);
                                output.push(*num as u8);
                            }
                        }
                        InstructionArg::Address(addr) => {
                            if addr <= &255 {
                                output.push(meta);
                                output.push(*addr as u8);
                            } else if addr <= &65535 {
                                meta |= 0b0000_0100;
                                output.push(meta);
                                output.push((addr >> 8) as u8);
                                output.push(*addr as u8);
                            } else {
                                meta |= 0b0000_1000;
                                output.push(meta);
                                output.push(((addr & 0xF0000) >> 16) as u8);
                                output.push(((addr & 0x0FF00) >> 8) as u8);
                                output.push(*addr as u8);
                            }
                        }
                        InstructionArg::Identifier(ident) => {
                            let label = match Assembler::find_label(ident, &self.parser_res.labels) {
                                Some(label) => label,
                                None => panic!("Label {} does not exist!", ident),
                            };

                            meta |= 0b0000_0100;
                            output.push(meta);
                            output.push((label.addr >> 8) as u8);
                            output.push(label.addr as u8);
                        }
                    },
                    InstructionType::Two => {
                        match &instruction.args[0] {
                            InstructionArg::Register(reg) => meta |= reg << 4,
                            _ => panic!("Argument 1 must be a register."),
                        }

                        match &instruction.args[1] {
                            InstructionArg::Register(reg) => {
                                output.push(meta);
                                output.push(*reg);
                            }
                            InstructionArg::Number(num) => {
                                if num > &255 {
                                    meta |= 0b0000_0100;
                                    output.push(meta);
                                    output.push((num >> 8) as u8);
                                    output.push(*num as u8);
                                } else {
                                    output.push(meta);
                                    output.push(*num as u8);
                                }
                            }
                            InstructionArg::Address(addr) => {
                                if addr <= &255 {
                                    output.push(meta);
                                    output.push(*addr as u8);
                                } else if addr <= &65535 {
                                    meta |= 0b0000_0100;
                                    output.push(meta);
                                    output.push((addr >> 8) as u8);
                                    output.push(*addr as u8);
                                } else {
                                    meta |= 0b0000_1000;
                                    output.push(meta);
                                    output.push(((addr & 0xF0000) >> 16) as u8);
                                    output.push(((addr & 0x0FF00) >> 8) as u8);
                                    output.push(*addr as u8);
                                }
                            }
                            InstructionArg::Identifier(ident) => {
                                let label =
                                    Assembler::find_label(ident, &self.parser_res.labels).unwrap();

                                meta |= 0b0000_0100;
                                output.push(meta);
                                output.push((label.addr >> 8) as u8);
                                output.push(label.addr as u8);
                            }
                        }
                    }
                }
            }
        }

        let code_len = output.len() as u16;

        output.insert(0, code_len as u8);
        output.insert(0, ((code_len & 0xFF00) >> 8) as u8);

        for interrupt in 0..255 {
            // println!("{}", interrupt);
            if self.parser_res.interrupts.get(&interrupt).is_some() {
                // The interrupt is defined, put the address in the output
                let label = Self::find_label(&self.parser_res.interrupts[&interrupt], &self.parser_res.labels).unwrap();
                output.push((((label.addr as u16) & 0xFF00) >> 8) as u8);
                output.push(label.addr as u8);
            } else {
                // The interrupt is not defined, fill with zero's
                output.push(0);
                output.push(0);
            }
        }

        output
    }
}
