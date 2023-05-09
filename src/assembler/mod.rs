pub mod parser;
pub mod tokenizer;

use std::collections::HashMap;

use crate::common::instruction::opcode::Instruction;

use self::parser::{DefineByteData, InstructionArg, InstructionType, Label, ParserResult};

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

    fn find_data_label(
        name: &String,
        labels: &HashMap<String, Vec<DefineByteData>>,
    ) -> Option<(String, Vec<DefineByteData>)> {
        for (label, values) in labels {
            if label == name {
                return Some((label.clone(), values.clone()));
            }
        }

        None
    }

    // fn data_label_len(byte_data: Vec<DefineByteData>) -> usize {
    //     let mut ret = 0;

    //     for data in byte_data {
    //         ret += data.len();
    //     }

    //     ret
    // }

    pub fn assemble(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        let mut data_section_len = 0;

        for (data_label_name, data_label_values) in &self.parser_res.data_labels {
            // The parser already checks for existing data labels and text labels.

            // But just in case...
            if Assembler::find_label(data_label_name, &self.parser_res.text_labels).is_some() {
                panic!(
                    "Label {} already defined for a text label.",
                    data_label_name
                );
            }

            for value in data_label_values {
                match value.clone() {
                    parser::DefineByteData::String(string, _) => {
                        data_section_len += string.len();

                        for char in string.chars() {
                            output.push(char.try_into().unwrap())
                        }
                    }
                    parser::DefineByteData::Byte(byte, _) => {
                        data_section_len += 1;

                        output.push(byte);
                    }

                    parser::DefineByteData::Short(short, _) => {
                        data_section_len += 2;

                        output.push(((short & 0xFF00) >> 8) as u8);
                        output.push(short as u8);
                    }
                }
            }
        }

        let mut code_len = 0;

        for label in &self.parser_res.text_labels {
            code_len += label.len();
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
                            let new_addr = addr + data_section_len as u32;

                            if new_addr <= 255 {
                                output.push(meta);
                                output.push(new_addr as u8);
                            } else if new_addr <= 65535 {
                                meta |= 0b0000_0100;
                                output.push(meta);
                                output.push((new_addr >> 8) as u8);
                                output.push(new_addr as u8);
                            } else {
                                meta |= 0b0000_1000;
                                output.push(meta);
                                output.push(((new_addr & 0xF0000) >> 16) as u8);
                                output.push(((new_addr & 0x0FF00) >> 8) as u8);
                                output.push(new_addr as u8);
                            }
                        }
                        InstructionArg::Identifier(ident) => {
                            meta |= 0b0000_0100;

                            match Assembler::find_label(ident, &self.parser_res.text_labels) {
                                Some(label) => {
                                    let addr = label.addr + data_section_len;
                                    output.push(meta);
                                    output.push((addr >> 8) as u8);
                                    output.push(addr as u8);
                                }
                                None => {
                                    // Label may be a data label
                                    // Let's check

                                    if Assembler::find_data_label(
                                        ident,
                                        &self.parser_res.data_labels,
                                    )
                                    .is_none()
                                    {
                                        panic!("No label named {}", ident);
                                    }

                                    let data_label = Assembler::find_data_label(
                                        ident,
                                        &self.parser_res.data_labels,
                                    )
                                    .unwrap();
                                    let data_label_start = match data_label.1[0] {
                                        DefineByteData::String(_, offset) => offset + 0x4402,
                                        DefineByteData::Byte(_, offset) => offset + 0x4402,
                                        DefineByteData::Short(_, offset) => offset + 0x4402,
                                    };

                                    output.push(meta);

                                    output.push((data_label_start >> 8) as u8);
                                    output.push(data_label_start as u8);
                                }
                            };

                            // let label = match Assembler::find_label(ident, &self.parser_res.text_labels) {
                            //     Some(label) => label,
                            //     None => panic!("Label {} does not exist!", ident),
                            // };

                            // meta |= 0b0000_0100;
                            // output.push(meta);
                            // output.push((label.addr >> 8) as u8);
                            // output.push(label.addr as u8);
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
                                let new_addr = addr + data_section_len as u32;

                                if new_addr <= 255 {
                                    output.push(meta);
                                    output.push(new_addr as u8);
                                } else if new_addr <= 65535 {
                                    meta |= 0b0000_0100;
                                    output.push(meta);
                                    output.push((new_addr >> 8) as u8);
                                    output.push(new_addr as u8);
                                } else {
                                    meta |= 0b0000_1000;
                                    output.push(meta);
                                    output.push(((new_addr & 0xF0000) >> 16) as u8);
                                    output.push(((new_addr & 0x0FF00) >> 8) as u8);
                                    output.push(new_addr as u8);
                                }
                            }
                            InstructionArg::Identifier(ident) => {
                                meta |= 0b0000_0100;

                                match Assembler::find_label(ident, &self.parser_res.text_labels) {
                                    Some(label) => {
                                        let addr = label.addr + data_section_len;
                                        output.push(meta);
                                        output.push((addr >> 8) as u8);
                                        output.push(addr as u8);
                                    }
                                    None => {
                                        // Label may be a data label
                                        // Let's check

                                        if Assembler::find_data_label(
                                            ident,
                                            &self.parser_res.data_labels,
                                        )
                                        .is_none()
                                        {
                                            panic!("No label named {}", ident);
                                        }

                                        let data_label = Assembler::find_data_label(
                                            ident,
                                            &self.parser_res.data_labels,
                                        )
                                        .unwrap();
                                        let data_label_start = match data_label.1[0] {
                                            DefineByteData::String(_, offset) => offset + 0x4402,
                                            DefineByteData::Byte(_, offset) => offset + 0x4402,
                                            DefineByteData::Short(_, offset) => offset + 0x4402,
                                        };

                                        output.push(meta);

                                        output.push((data_label_start >> 8) as u8);
                                        output.push(data_label_start as u8);
                                    }
                                };
                            }
                        }
                    }
                }
            }
        }

        output.insert(0, data_section_len as u8);
        output.insert(0, ((data_section_len & 0xFF00) >> 8) as u8);

        output.insert(0, code_len as u8);
        output.insert(0, ((code_len & 0xFF00) >> 8) as u8);

        let start_index = match self.parser_res.metadata.get("main") {
            None => panic!("No main label defined."),
            Some(label_value) => match label_value.to_owned() {
                parser::MetadataValue::String(label_name) => {
                    match Assembler::find_label(&label_name, &self.parser_res.text_labels) {
                        None => panic!("Label {} does not exist.", label_name),
                        Some(label) => label.addr + data_section_len,
                    }
                }
                parser::MetadataValue::Number(_) => {
                    panic!("Defining main label cannot have a number as a value.")
                }
            },
        };

        output.insert(0, start_index as u8);
        output.insert(0, ((start_index & 0xFF00) >> 8) as u8);

        for interrupt in 0..255 {
            // println!("{}", interrupt);
            if self.parser_res.interrupts.get(&interrupt).is_some() {
                // The interrupt is defined, put the address in the output
                let label = Self::find_label(
                    &self.parser_res.interrupts[&interrupt],
                    &self.parser_res.text_labels,
                )
                .unwrap();
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
