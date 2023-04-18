use std::{collections::HashMap, str::FromStr};

use super::tokenizer::Token;
use crate::common::instruction::opcode::{AddressingMode, Instruction, Opcode};

pub type TokenInfoType = (Token, String);

use regex::Regex;

#[derive(Debug, Clone)]
pub enum InstructionArg {
    Register(u8),
    Number(u16),
    Address(u32),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Zero,
    One,
    Two,
}

#[derive(Debug)]
pub struct ParserResult {
    pub metadata: HashMap<String, MetadataValue>,
    pub labels: Vec<Label>,
}

impl ParserResult {
    pub fn new(metadata: HashMap<String, MetadataValue>, labels: Vec<Label>) -> ParserResult {
        ParserResult { metadata, labels }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParserInstruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub instruction_type: InstructionType,
    pub args: Vec<InstructionArg>,
}

impl ParserInstruction {
    pub fn get_instruction(opcode: Opcode, args: Vec<InstructionArg>) -> ParserInstruction {
        let instruction_type = match args.len() {
            0 => InstructionType::Zero,
            1 => InstructionType::One,
            2 => InstructionType::Two,
            _ => panic!("Too many arguments for instruction."),
        };

        let map = Instruction::hashmap();

        match args.len() {
            0 => {
                let full_opcode = Instruction::create_opcode(opcode, AddressingMode::Discard);

                if map.contains_key(&full_opcode) {
                    let (_, _, _, arg_count) = map.get(&full_opcode).unwrap();

                    if arg_count < &0 {
                        panic!(
                            "Too many arguments for instruction {:?} with arg {:?}",
                            opcode, args[0]
                        );
                    }

                    return ParserInstruction {
                        opcode,
                        addressing_mode: AddressingMode::Discard,
                        instruction_type,
                        args,
                    };
                }

                panic!("Invalid instruction {:?}", opcode);
            }
            1 => {
                let mode = match args[0] {
                    InstructionArg::Register(_) => AddressingMode::Register,
                    InstructionArg::Number(_) => AddressingMode::Immediate,
                    InstructionArg::Address(_) => AddressingMode::Direct,
                    InstructionArg::Identifier(_) => AddressingMode::Direct,
                };

                let full_opcode = Instruction::create_opcode(opcode, mode);

                if map.contains_key(&full_opcode) {
                    let (_, _, _, arg_count) = map.get(&full_opcode).unwrap();

                    match arg_count.cmp(&1) {
                        std::cmp::Ordering::Less => panic!(
                            "Too many arguments for instruction {:?} with arg {:?}",
                            opcode, args[0]
                        ),
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => panic!(
                            "Too few arguments for instruction {:?} with arg {:?}",
                            opcode, args[0]
                        ),
                    }

                    return ParserInstruction {
                        opcode,
                        addressing_mode: mode,
                        instruction_type,
                        args,
                    };
                }

                panic!("Invalid instruction {:?} with arg {:?} (opcode doesn't accept mode / type {:?})", opcode, args[0], mode);
            }
            2 => {
                match args[0] {
                    InstructionArg::Register(_) => (),
                    _ => panic!(
                        "Invalid instruction {:?} with args {:?} (first arg must be register)",
                        opcode, args
                    ),
                }

                let mode = match args[1] {
                    InstructionArg::Register(_) => AddressingMode::Register,
                    InstructionArg::Number(_) => AddressingMode::Immediate,
                    InstructionArg::Address(_) => AddressingMode::Direct,
                    InstructionArg::Identifier(_) => AddressingMode::Direct,
                };

                let full_opcode = Instruction::create_opcode(opcode, mode);

                if map.contains_key(&full_opcode) {
                    let (_, _, _, arg_count) = map.get(&full_opcode).unwrap();

                    match arg_count.cmp(&2) {
                        std::cmp::Ordering::Less => panic!(
                            "Too many arguments for instruction {:?} with args {:?}",
                            opcode, args
                        ),
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => panic!(
                            "Too few arguments for instruction {:?} with args {:?}",
                            opcode, args
                        ),
                    }

                    return ParserInstruction {
                        opcode,
                        addressing_mode: mode,
                        instruction_type,
                        args,
                    };
                }

                panic!("Invalid instruction {:?} with args {:?} (opcode doesn't accept mode / type {:?})", opcode, args, mode);
            }
            _ => {
                panic!(
                    "Invalid instruction {:?} with args {:?}. Too many args.",
                    opcode, args
                );
            }
        }
    }

    fn len(&self) -> usize {
        let mut init_len: usize = 2;

        match self.instruction_type {
            InstructionType::Zero => (),
            InstructionType::One => match self.args[0] {
                InstructionArg::Register(_) => (),
                InstructionArg::Number(num) => {
                    if num > 255 {
                        init_len += 2;
                    } else {
                        init_len += 1;
                    }
                }
                InstructionArg::Address(addr) => {
                    if addr <= 255 {
                        init_len += 1;
                    } else if addr <= 65535 {
                        init_len += 2;
                    } else {
                        init_len += 3;
                    }
                }
                InstructionArg::Identifier(_) => init_len += 2,
            },
            InstructionType::Two => {
                match self.args[0] {
                    InstructionArg::Register(_) => (),
                    _ => panic!(
                        "Instruction arg 0 is not a register when checking instruction size."
                    ),
                }

                match self.args[1] {
                    InstructionArg::Register(_) => init_len += 1,
                    InstructionArg::Number(num) => {
                        if num > 255 {
                            init_len += 2;
                        } else {
                            init_len += 1;
                        }
                    }
                    InstructionArg::Address(addr) => {
                        if addr <= 255 {
                            init_len += 1;
                        } else if addr <= 65535 {
                            init_len += 2;
                        } else {
                            init_len += 3;
                        }
                    }
                    InstructionArg::Identifier(_) => init_len += 2,
                }
            }
        }

        init_len
    }
}

#[derive(Debug, Clone)]
pub enum MetadataValue {
    String(String),
    Number(u16),
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
    pub instructions: Vec<ParserInstruction>,
    pub addr: usize,
}

impl Label {
    pub fn new(name: String, addr: usize) -> Label {
        Label {
            name,
            instructions: Vec::new(),
            addr,
        }
    }

    fn add(&mut self, instruction: ParserInstruction) {
        self.instructions.push(instruction);
    }

    pub fn len(&self) -> usize {
        let mut size: usize = 0;

        for instruction in &self.instructions {
            size += instruction.len();
        }

        size
    }
}

pub struct Parser {
    tokens: Vec<TokenInfoType>,
    pub metadata: HashMap<String, MetadataValue>,
    pub labels: Vec<Label>,
    current_token_index: u32,
    label_offset: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfoType>) -> Parser {
        Parser {
            tokens,
            metadata: HashMap::new(),
            labels: Vec::new(),
            current_token_index: 0,
            label_offset: 0,
        }
    }

    fn get_token(&self) -> Option<&TokenInfoType> {
        if self.current_token_index == self.tokens.len() as u32 {
            return None;
        }

        Some(&self.tokens[self.current_token_index as usize])
    }

    fn peek_token(&self) -> Option<&TokenInfoType> {
        if self.current_token_index + 1 == self.tokens.len() as u32 {
            return None;
        }

        Some(&self.tokens[(self.current_token_index + 1) as usize])
    }

    fn convert_short_to_base(short: String) -> u16 {
        let reg = Regex::new(r"^0[xX][0-9A-Fa-f]+$").unwrap();

        if reg.is_match(&short) {
            u16::from_str_radix(&short[2..], 16).unwrap()
        } else {
            short.parse::<u16>().unwrap()
        }
    }

    fn convert_int_to_base(short: String) -> u32 {
        let reg = Regex::new(r"^0[xX][0-9A-Fa-f]+$").unwrap();

        if reg.is_match(&short) {
            u32::from_str_radix(&short[2..], 16).unwrap()
        } else {
            short.parse::<u32>().unwrap()
        }
    }

    fn label_exists(&mut self, name: &String) -> bool {
        if self.labels.is_empty() {
            return false;
        }

        for label in &self.labels {
            if &label.name == name {
                return true;
            }
        }

        false
    }

    pub fn parse(&mut self) -> ParserResult {
        while self.current_token_index < self.tokens.len() as u32 {
            // println!("Tokens");
            if self.get_token().is_none() {
                break;
            }

            let token = self.get_token().unwrap();

            match token.0 {
                Token::Metadata => self.parse_metadata(),
                Token::Label => self.parse_label(),
                Token::Error => panic!("Unknown symbol \"{}\"", token.1),
                Token::NewLine => self.current_token_index += 1,
                _ => panic!("Unknown token {:?}", token.0),
            }
        }

        ParserResult::new(self.metadata.clone(), self.labels.clone())
    }

    fn parse_metadata(&mut self) {
        if self.get_token().is_none() {
            panic!("No tokens left, expecting metadata.");
        }

        let token = self.get_token().unwrap();

        // 0 should be the metadata name, 1 should be the value.
        let binding = token.1.replace('.', "");
        let tok_string_split: Vec<&str> = binding.split(' ').collect();

        let type_regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
        let num_regex = Regex::new(r"^\b(0[xX][0-9a-fA-F]+|[0-9]+)\b$").unwrap();

        // Check if the value is a string
        if type_regex.is_match(tok_string_split[1]) {
            // Value is some kind of string
            self.metadata.insert(
                String::from(tok_string_split[0]),
                MetadataValue::String(String::from(tok_string_split[1])),
            );
        } else if num_regex.is_match(tok_string_split[1]) {
            // Value is some kind of number
            self.metadata.insert(
                String::from(tok_string_split[0]),
                MetadataValue::Number(Self::convert_short_to_base(String::from(
                    tok_string_split[1],
                ))),
            );
        } else {
            // Value is something else...
            panic!("Invalid metadata value: {}", tok_string_split[1]);
        }

        self.current_token_index += 1;
    }

    fn parse_label(&mut self) {
        if self.get_token().is_none() {
            panic!("No tokens left, expecting label.");
        }

        let label_token = self.get_token().unwrap();
        let label_name = label_token.1.replace(':', "");

        if self.label_exists(&label_name) {
            panic!("Label {} already defined.", label_name);
        }

        self.current_token_index += 1;

        let mut label = Label::new(label_name, self.label_offset + 0x4402);

        loop {
            if self.get_token().is_none() {
                break;
            }

            let instruction_token = self.get_token().unwrap();

            if instruction_token.0 == Token::NewLine {
                self.current_token_index += 1;
                continue;
            } else if instruction_token.0 != Token::Identifier {
                // panic!("Need instruction, not {:?}", instruction_token.0);
                break;
            }

            let instruction = self.make_instruction();

            label.add(instruction);
        }

        self.label_offset += label.len();

        if label.instructions.is_empty() {
            panic!("Label {} has no body.", label.name);
        }

        self.labels.push(label);
    }

    fn make_instruction(&mut self) -> ParserInstruction {
        if self.get_token().is_none() {
            panic!("No tokens left, expecting instruction.");
        }

        let instruction_token = self.get_token().unwrap();
        let opcode = match Opcode::from_str(&instruction_token.1.to_ascii_lowercase()) {
            Ok(opcode) => opcode,
            Err(_) => panic!("Unknown instruction: {}", instruction_token.1),
        };

        self.current_token_index += 1;

        if self.get_token().is_none() {
            // Depends on what the instruction is, we may not need any args.
            return ParserInstruction::get_instruction(opcode, vec![]);
        }

        let mut args: Vec<InstructionArg> = Vec::new();

        let token = self.get_token().unwrap();

        if token.0 == Token::NewLine {
            // This instruction has no arguments.
            return ParserInstruction::get_instruction(opcode, vec![]);
        }

        if token.0 == Token::Comma {
            panic!("Expected argument, got comma.");
        }

        match token.0 {
            Token::Register => {
                // We need to take into account that instructions that has two arguments always have a register as the first argument.
                // So we need to check if the next token is a comma, if it is, then we know that the next token is the second argument.

                let mut chars = token.1.chars();
                chars.next();
                let cleaned = chars.as_str();

                let reg_num: u8 = match cleaned.to_lowercase().as_str() {
                    "1" => 0,
                    "2" => 1,
                    "3" => 2,
                    "4" => 3,
                    "5" => 4,
                    "6" => 5,
                    "pc" => 6,
                    "sp" => 7,
                    "bp" => 8,
                    _ => panic!("Unknown register \"{}\"", cleaned),
                };

                self.current_token_index += 1;

                args.push(InstructionArg::Register(reg_num));

                if self.peek_token().is_none() {
                    return ParserInstruction::get_instruction(opcode, args);
                }

                let sec_arg_test = self.get_token().unwrap();

                // See if we need to store a second argument.
                // If we don't have a comma, then we don't need to store a second argument.

                if sec_arg_test.0 != Token::Comma {
                    return ParserInstruction::get_instruction(opcode, args);
                }

                self.current_token_index += 1;

                if self.get_token().is_none() {
                    panic!("Expected second argument, got nothing.");
                }

                let sec_arg = self.get_token().unwrap();

                match sec_arg.0 {
                    Token::Register => {
                        let mut chars = sec_arg.1.chars();
                        chars.next();
                        let cleaned = chars.as_str();

                        let reg_num: u8 = match cleaned.to_lowercase().as_str() {
                            "1" => 0,
                            "2" => 1,
                            "3" => 2,
                            "4" => 3,
                            "5" => 4,
                            "6" => 5,
                            "pc" => 6,
                            "sp" => 7,
                            "bp" => 8,
                            _ => panic!("Unknown register \"{}\"", cleaned),
                        };

                        args.push(InstructionArg::Register(reg_num));
                        self.current_token_index += 1;

                        ParserInstruction::get_instruction(opcode, args)
                    }
                    Token::Number => {
                        let value = Self::convert_short_to_base(sec_arg.1.clone());

                        args.push(InstructionArg::Number(value));
                        self.current_token_index += 1;

                        ParserInstruction::get_instruction(opcode, args)
                    }
                    Token::Address => {
                        let cleaned = sec_arg.1.replace('$', "");
                        let value = Self::convert_int_to_base(cleaned);

                        args.push(InstructionArg::Address(value));
                        self.current_token_index += 1;

                        ParserInstruction::get_instruction(opcode, args)
                    }
                    Token::Identifier => {
                        args.push(InstructionArg::Identifier(sec_arg.1.clone()));
                        self.current_token_index += 1;

                        ParserInstruction::get_instruction(opcode, args)
                    }
                    _ => todo!(),
                }
            }
            Token::Address => {
                let cleaned = token.1.replace('$', "");
                let value = Self::convert_int_to_base(cleaned);

                args.push(InstructionArg::Address(value));

                self.current_token_index += 1;

                ParserInstruction::get_instruction(opcode, args)
            }
            Token::Number => {
                let value = Self::convert_short_to_base(token.1.clone());

                args.push(InstructionArg::Number(value));

                self.current_token_index += 1;

                ParserInstruction::get_instruction(opcode, args)
            }
            Token::Identifier => {
                args.push(InstructionArg::Identifier(token.1.clone()));
                self.current_token_index += 1;

                ParserInstruction::get_instruction(opcode, args)
            }
            Token::Error => panic!("Error token found: {:?}", token.0),
            _ => panic!("Expected argument, got {:?}", token.0),
        }
    }
}
