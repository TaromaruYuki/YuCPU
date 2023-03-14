mod label;
mod token;

#[cfg(test)]
mod tests;

use super::common::instruction::Instruction;
use label::Label;
use label::LabelType;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use token::Token;

// type Instruction = (u8, u8, u8, u8);
type InstructionFunction = fn(&AssemblerLabels, &Token) -> Instruction;

#[derive(PartialEq)]
enum AssemblerUse {
    File,
    Lines,
}

pub struct Assembler {
    pub file_in: Box<Path>,
    pub file_out: Box<Path>,
    pub lines: Vec<String>,
    assembler_use: AssemblerUse,

    start_label: String,
    inst_map: HashMap<String, InstructionFunction>,
}

impl Assembler {
    pub fn new(f_in: &String, f_out: &String) -> Self {
        let file_in_path: &Path = Path::new(f_in);
        let file_out_path: &Path = Path::new(f_out);

        Assembler {
            file_in: Into::into(file_in_path),
            file_out: Into::into(file_out_path),
            start_label: String::from(""),
            inst_map: HashMap::new(),
            assembler_use: AssemblerUse::File,
            lines: Vec::new(),
        }
    }

    #[allow(dead_code)] // Used in tests, is used.
    pub fn new_lines(lines: Vec<String>) -> Self {
        Assembler {
            file_in: Into::into(Path::new("")),
            file_out: Into::into(Path::new("")),
            start_label: String::from(""),
            inst_map: HashMap::new(),
            assembler_use: AssemblerUse::Lines,
            lines,
        }
    }
}

pub struct AssemblerLabels {
    data_labels: Vec<Label>,
    text_labels: Vec<Label>,
}

impl Assembler {
    fn fill_hashmap(&mut self) {
        // FIXME: There HAS to be a better way to do ALL of this in the `new` function...

        self.inst_map.insert(String::from("LD"), Assembler::inst_ld);
        self.inst_map
            .insert(String::from("MOV"), Assembler::inst_mov);
        self.inst_map
            .insert(String::from("PSH"), Assembler::inst_psh);
        self.inst_map
            .insert(String::from("POP"), Assembler::inst_pop);
        self.inst_map
            .insert(String::from("LDS"), Assembler::inst_lds);
        self.inst_map.insert(String::from("ST"), Assembler::inst_st);
        self.inst_map
            .insert(String::from("STL"), Assembler::inst_stl);
        self.inst_map
            .insert(String::from("STH"), Assembler::inst_sth);
        self.inst_map
            .insert(String::from("CMP"), Assembler::inst_cmp);
        self.inst_map
            .insert(String::from("BEQ"), Assembler::inst_beq);
        self.inst_map
            .insert(String::from("BGT"), Assembler::inst_bgt);
        self.inst_map
            .insert(String::from("BLT"), Assembler::inst_blt);
        self.inst_map
            .insert(String::from("JMP"), Assembler::inst_jmp);
        self.inst_map
            .insert(String::from("BOF"), Assembler::inst_bof);
        self.inst_map
            .insert(String::from("BNE"), Assembler::inst_bne);
        self.inst_map
            .insert(String::from("ADD"), Assembler::inst_add);
        self.inst_map
            .insert(String::from("SUB"), Assembler::inst_sub);
        self.inst_map
            .insert(String::from("CALL"), Assembler::inst_call);
        self.inst_map
            .insert(String::from("RET"), Assembler::inst_ret);
        self.inst_map
            .insert(String::from("HLT"), Assembler::inst_hlt);
        self.inst_map
            .insert(String::from("NOP"), Assembler::inst_nop);
    }

    pub fn assemble(mut self) -> (Vec<u8>, Self) {
        self.fill_hashmap();
        let tokens: (String, Vec<Token>, Vec<Token>) = if self.assembler_use == AssemblerUse::File {
            self.tokenize(self.read_input_file())
        } else {
            self.tokenize(self.lines.clone())
        };

        if !tokens.0.is_empty() {
            self.start_label = tokens.0.clone();
        }

        let labels: AssemblerLabels = self.parse_labels(&tokens.1, &tokens.2);

        (self.labels_to_bytes(labels, (&tokens.1, &tokens.2)), self)
    }

    fn labels_to_bytes(
        &self,
        labels: AssemblerLabels,
        tokens: (&Vec<Token>, &Vec<Token>),
    ) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let start_label = Assembler::find_label(self.start_label.clone(), &labels);

        if start_label.l_type == LabelType::None {
            panic!("No start label set.");
        }

        bytes.push(((start_label.addr >> 8) & 0xFF).try_into().unwrap());
        bytes.push((start_label.addr & 0xFF).try_into().unwrap());

        for label in labels.data_labels.iter() {
            for c in label.value.chars() {
                bytes.push(c as u8);
            }
            bytes.push(b'\0');
        }

        for token in tokens.1 {
            if self.inst_map.contains_key(&token.instruction) {
                let inst_bytes = self.inst_map[&token.instruction](&labels, token);
                bytes.push(inst_bytes.opcode);
                bytes.push(inst_bytes.register);
                bytes.push(((inst_bytes.data >> 8) & 0xFF).try_into().unwrap());
                bytes.push((inst_bytes.data & 0xFF).try_into().unwrap());
            }
        }

        bytes
    }

    fn tokenize(&self, lines: Vec<String>) -> (String, Vec<Token>, Vec<Token>) {
        #[derive(PartialEq)]
        enum TokenPaths {
            None,
            Data,
            Text,
        }

        let mut data_tokens: Vec<Token> = Vec::new();
        let mut text_tokens: Vec<Token> = Vec::new();
        let mut path: TokenPaths = TokenPaths::None;

        let mut curr_label: String = String::from("");
        let mut start_label: String = String::from("");

        for line in lines {
            if (line.len() as u32) == 0 {
                continue;
            }

            if path == TokenPaths::None {
                if &line[..1] == "." {
                    let cloned_line = line.clone();
                    let split = cloned_line.split(' ').collect::<Vec<&str>>();

                    if split[0] == ".main" {
                        start_label = split[1].to_string();
                    } else if split[0] == ".data" {
                        path = TokenPaths::Data;
                    } else if split[0] == ".text" {
                        path = TokenPaths::Text;
                    }
                }
            } else if path == TokenPaths::Data {
                let f_line_s = line.replace('\t', "");
                let f_line: Vec<&str> = f_line_s.split(' ').collect();

                if &f_line[0][..1] == "@" {
                    curr_label = f_line[0].replace(['@', ':'], "").to_string();
                    continue;
                } else if &f_line[0][..1] == "." {
                    path = TokenPaths::Text;
                } else {
                    let mut args: Vec<String> = Vec::new();

                    if f_line[0] == "DAT" {
                        if &f_line[1][..1] == "\"" {
                            let mut arg_str = String::from("");

                            if &f_line[1][f_line[1].len() - 1..f_line[1].len()] == "\"" {
                                arg_str += f_line[1].replace('\"', "").as_str();
                            } else {
                                arg_str += f_line[1].replace('\"', "").as_str();
                                arg_str += " ";

                                for str_segment in f_line[2..f_line.len()].iter() {
                                    arg_str += str_segment.replace('\"', "").as_str();

                                    if &str_segment[str_segment.len() - 1..str_segment.len()]
                                        != "\""
                                    {
                                        arg_str += " ";
                                    }
                                }
                            }

                            args.push(arg_str);
                        }
                    } else {
                        for arg in f_line[1..f_line.len()].iter() {
                            args.push(arg.to_string());
                        }
                    }

                    let token = Token::create_token(
                        curr_label.clone(),
                        f_line[0].to_string(),
                        args,
                        LabelType::Data,
                    );

                    data_tokens.push(token);
                }
            } else if path == TokenPaths::Text {
                let f_line_s = line.replace('\t', "");
                let f_line: Vec<&str> = f_line_s.split(' ').collect();

                if &f_line[0][..1] == "@" {
                    curr_label = f_line[0].replace(['@', ':'], "").to_string();
                    continue;
                } else {
                    let token = Token {
                        label: curr_label.clone(),
                        instruction: f_line[0].to_string(),
                        args: f_line[1..f_line.len()]
                            .iter()
                            .map(|x| x.replace(',', ""))
                            .collect(),
                        section: LabelType::Text,
                    };

                    text_tokens.push(token);
                }
            }
        }

        (start_label, data_tokens, text_tokens)
    }

    fn is_valid_register(reg: String) -> u8 {
        if reg == "RPC" {
            return 0x6;
        } else if reg == "RSP" {
            return 0x7;
        } else if reg == "RBP" {
            return 0x8;
        }

        if &reg[..1] != "R" {
            panic!("Invalid register name: \"{}\"", reg);
        }

        let reg_num: u8 = reg[1..2].parse::<u8>().unwrap();

        if reg_num == 0 || reg_num > 6 {
            panic!("Register {} out of range", reg);
        }

        reg_num - 1
    }

    fn parse_labels(&self, data_tokens: &Vec<Token>, text_tokens: &Vec<Token>) -> AssemblerLabels {
        let mut data_labels: Vec<Label> = Vec::new();
        let mut text_labels: Vec<Label> = Vec::new();
        let mut used_data_labels: Vec<String> = Vec::new();

        let mut address_offset = 0;
        let mut text_label = text_tokens[0].label.clone();
        let mut text_label_offset = 0;
        let mut instruction_count = 0;

        // Data tokens
        for data_t in data_tokens {
            if data_t.instruction != "DAT" {
                panic!(
                    "Invalid instruction in data label {}: {}",
                    data_t.label, data_t.instruction
                );
            }

            if data_t.args.len() != 1 {
                panic!(
                    "Invalid number of arguments in data label {}: {}",
                    data_t.label,
                    data_t.args.len()
                );
            }

            if used_data_labels.contains(&data_t.label) {
                panic!(
                    "Label {} already has a DAT. Labels cannot have multiple DATs.",
                    data_t.label
                );
            }

            used_data_labels.push(data_t.label.clone());

            let label =
                Label::create_data(data_t.label.clone(), address_offset, data_t.args[0].clone());

            address_offset += label.value.len() as u16;
            address_offset += 1;

            data_labels.push(label);
        }

        // Text Tokens
        for (i, text_t) in text_tokens.iter().enumerate() {
            // Couldn't use a "||" to combine both if statements. Not even a else if. Pain.
            // TODO: Maybe find a way to combine into one if statement...
            if text_t.label != text_label {
                let label = Label::create_text(
                    text_label.clone(),
                    address_offset,
                    text_label_offset - (instruction_count * 4),
                );

                text_labels.push(label);

                text_label = text_t.label.clone();
                instruction_count = 0;
            }

            if i == text_tokens.len() - 1 {
                let label = Label::create_text(
                    text_label.clone(),
                    address_offset,
                    text_label_offset - (instruction_count * 4),
                );

                text_labels.push(label);

                text_label = text_t.label.clone();
            }

            instruction_count += 1;
            text_label_offset += 4;
        }

        AssemblerLabels {
            data_labels,
            text_labels,
        }
    }

    fn find_label(name: String, labels: &AssemblerLabels) -> Label {
        for label in &labels.data_labels {
            if label.name == name {
                return label.to_owned();
            }
        }

        for label in &labels.text_labels {
            if label.name == name {
                return label.to_owned();
            }
        }

        Label::new()
    }

    fn read_input_file(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let display = self.file_in.display();

        let file = match File::open(&self.file_in) {
            Err(why) => panic!("Opening file \"{}\" failed!\n\n{}", display, why),
            Ok(file) => file,
        };

        let buffer = std::io::BufReader::new(file);

        for line in buffer.lines().flatten() {
            result.push(line);
        }

        result
    }

    fn get_value_from_str(str: String) -> u16 {
        // 0(x or X)(Any amount of characters, 0-9, A-F, and a-f)
        // Examples: 0x14F, 0Xfac
        let reg = Regex::new(r"^0[xX][0-9A-Fa-f]+$").unwrap();

        if reg.is_match(&str) {
            u16::from_str_radix(&str[2..], 16).unwrap()
        } else {
            str.parse::<u16>().unwrap()
        }
    }

    // !!! Instructions

    fn inst_ld(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());
        let test_regex = Regex::new(r"^\d+$").unwrap();

        if &token.args[1][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[1].replace('$', ""));

            Instruction::new(0x02, reg, addr)
        } else if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x01, reg, 0x00, reg2)
        // } else if !&token.args[1].starts_with("0x") && test_regex.is_match(&token.args[1]) {
        } else if !&token.args[1].starts_with("0x")
            && test_regex.replace(&token.args[1].clone(), "") != ""
        {
            // Most likely a label pointing to the text section or the data section.

            let found_label = Assembler::find_label(token.args[1].clone(), labels);

            if found_label.l_type == LabelType::None {
                panic!("Label \"{}\" does not exist!", token.args[1]);
            }

            Instruction::new(0x02, reg, found_label.addr)
        } else {
            panic!("LD takes in a address, label, or register.");
        }
    }

    fn inst_mov(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());
        if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x09, reg, 0x00, reg2)
        } else {
            let val = Assembler::get_value_from_str(token.args[1].clone());

            Instruction::new(0x00, reg, val)
        }
    }

    fn inst_psh(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        if &token.args[0][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[0].replace('$', ""));

            Instruction::new(0x05, 0x00, addr)
        } else if &token.args[0][..1] == "R" {
            let reg = Assembler::is_valid_register(token.args[0].clone());

            Instruction::new_u8(0x04, reg, 0x00, 0x00)
        } else {
            let addr = Assembler::get_value_from_str(token.args[0].clone());

            Instruction::new(0x03, 0x00, addr)
        }
    }

    fn inst_pop(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        if token.args.len() == 1 {
            let reg = Assembler::is_valid_register(token.args[0].clone());

            Instruction::new(0x06, reg, 0x00)
        } else if token.args.is_empty() {
            Instruction::new(0x07, 0x00, 0x00)
        } else {
            panic!("POP instruction has too many args!");
        }
    }

    fn inst_lds(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[1].replace('$', ""));

            Instruction::new(0x08, reg, addr)
        } else {
            panic!("LDS can only have an address as a value")
        }
    }

    fn inst_st(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[1].replace('$', ""));

            Instruction::new(0x10, reg, addr)
        } else if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x13, reg, 0x00, reg2)
        } else {
            panic!("ST instruction cannot take in any other data type except ADDR and REG.");
        }
    }

    fn inst_stl(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[1].replace('$', ""));

            Instruction::new(0x11, reg, addr)
        } else if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x14, reg, 0x00, reg2)
        } else {
            panic!("STL instruction cannot take in any other data type except ADDR and REG.");
        }
    }

    fn inst_sth(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[1].replace('$', ""));

            Instruction::new(0x12, reg, addr)
        } else if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x15, reg, 0x00, reg2)
        } else {
            panic!("STH instruction cannot take in any other data type except ADDR and REG.");
        }
    }

    fn inst_cmp(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x20, reg, 0x00, reg2)
        } else {
            let addr = Assembler::get_value_from_str(token.args[1].clone());

            Instruction::new(0x21, reg, addr)
        }
    }

    fn inst_beq(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x30, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_bgt(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x31, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_blt(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x32, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_jmp(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x33, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_bof(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x34, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_bne(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x35, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_add(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x42, reg, 0x00, reg2)
        } else {
            let addr = Assembler::get_value_from_str(token.args[1].clone());

            Instruction::new(0x40, reg, addr)
        }
    }

    fn inst_sub(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[1].clone());

            Instruction::new_u8(0x43, reg, 0x0, reg2)
        } else {
            let addr = Assembler::get_value_from_str(token.args[1].clone());

            Instruction::new(0x41, reg, addr)
        }
    }

    fn inst_call(labels: &AssemblerLabels, token: &Token) -> Instruction {
        let found_label = Assembler::find_label(token.args[0].clone(), labels);

        if found_label.l_type != LabelType::None {
            Instruction::new(0x50, 0x00, found_label.addr)
        } else {
            panic!("Label {} is undefined", token.args[0]);
        }
    }

    fn inst_ret(_labels: &AssemblerLabels, _token: &Token) -> Instruction {
        Instruction::new(0x51, 0x00, 0x0000)
    }

    fn inst_hlt(_labels: &AssemblerLabels, _token: &Token) -> Instruction {
        Instruction::new_u8(0xFE, 0xFE, 0xFF, 0xFF)
    }

    fn inst_nop(_labels: &AssemblerLabels, _token: &Token) -> Instruction {
        Instruction::new_u8(0xFF, 0xFF, 0xFF, 0xFF)
    }
}
