#![allow(dead_code)]
#![allow(unused_imports)]

mod label;
mod token;

use label::Label;
use label::LabelType;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use token::Token;

type Instruction = (u8, u8, u8, u8);
type InstructionFunction = fn(&AssemblerLabels, &Token) -> Instruction;

pub struct Assembler {
    pub file_in: Box<Path>,
    pub file_out: Box<Path>,

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
        }
    }
}

pub struct AssemblerLabels {
    data_labels: Vec<Label>,
    text_labels: Vec<Label>,
}

impl Assembler {
    fn fill_hashmap(mut self) -> Self {
        // FIXME: There HAS to be a better way to do ALL of this in the `new` function...

        self.inst_map.insert(String::from("LD"), Assembler::inst_ld);

        self
    }

    pub fn assemble(mut self) {
        self = self.fill_hashmap();

        let tokens: (String, Vec<Token>, Vec<Token>) = self.tokenize();

        if !tokens.0.is_empty() {
            self.start_label = tokens.0.clone();
        }

        let labels: AssemblerLabels = self.parse_labels(&tokens.1, &tokens.2);

        println!("Data Labels:\n{:#?}\n\n", labels.data_labels);
        println!("Text Labels:\n{:#?}\n\n", labels.text_labels);

        let bytes = self.labels_to_bytes(labels, (&tokens.1, &tokens.2));

        // println!("{:#?}", bytes);

        match fs::write(&self.file_out, bytes) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to write output file.\n{error}");
                exit(1);
            }
        };
    }

    fn labels_to_bytes(
        &self,
        labels: AssemblerLabels,
        tokens: (&Vec<Token>, &Vec<Token>),
    ) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for label in labels.data_labels.iter() {
            for c in label.value.chars() {
                bytes.push(c as u8);
            }
            bytes.push(b'\0');
        }

        for token in tokens.1 {
            if self.inst_map.contains_key(&token.instruction) {
                let inst_bytes = self.inst_map[&token.instruction](&labels, token);
                bytes.push(inst_bytes.0);
                bytes.push(inst_bytes.1);
                bytes.push(inst_bytes.2);
                bytes.push(inst_bytes.3);
            }
        }

        bytes
    }

    fn tokenize(&self) -> (String, Vec<Token>, Vec<Token>) {
        #[derive(PartialEq)]
        enum TokenPaths {
            None,
            Data,
            Text,
        }

        let lines: Vec<String> = self.read_input_file();

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
        if reg == "PC" {
            return 0x6;
        } else if reg == "SP" {
            return 0x7;
        }

        if &reg[..1] != "R" {
            eprintln!("Invalid register name: \"{}\"", reg);
            exit(1);
        }

        let reg_num: u8 = reg[1..2].parse::<u8>().unwrap();

        if reg_num > 6 {
            eprintln!("Register {} out of range", reg);
            exit(1);
        }

        reg_num
    }

    // FIXME: When a label has more than one line, it makes the addresses too much
    fn parse_labels(&self, data_tokens: &Vec<Token>, text_tokens: &Vec<Token>) -> AssemblerLabels {
        let mut data_labels: Vec<Label> = Vec::new();
        let mut text_labels: Vec<Label> = Vec::new();
        let mut used_data_labels: Vec<String> = Vec::new();

        let mut address_offset = 0;
        let mut text_label = text_tokens[0].label.clone();
        let mut text_label_offset = 0;

        // Data tokens
        for data_t in data_tokens {
            if data_t.instruction != "DAT" {
                eprintln!(
                    "Invalid instruction in data label {}: {}",
                    data_t.label, data_t.instruction
                );
                exit(1);
            }

            if data_t.args.len() != 1 {
                eprintln!(
                    "Invalid number of arguments in data label {}: {}",
                    data_t.label,
                    data_t.args.len()
                );
                exit(1);
            }

            if used_data_labels.contains(&data_t.label) {
                eprintln!(
                    "Label {} already has a DAT. Labels cannot have multiple DATs.",
                    data_t.label
                );
            }

            used_data_labels.push(data_t.label.clone());

            let label =
                Label::create_data(data_t.label.clone(), address_offset, data_t.args[0].clone());

            println!(
                "!!!! Len: {}. Curr off: {address_offset}",
                label.value.len()
            );
            address_offset += label.value.len() as u16;
            address_offset += 1;

            data_labels.push(label);
        }

        println!("!! addr_offset = {address_offset}");

        // Text Tokens
        for (i, text_t) in text_tokens.iter().enumerate() {
            // Couldn't use a "||" to combine both if statements. Not even a else if. Pain.
            // TODO: Maybe find a way to combine into one if statement...
            if text_t.label != text_label {
                let label =
                    Label::create_text(text_label.clone(), address_offset, text_label_offset - 4);

                text_labels.push(label);

                text_label = text_t.label.clone();
            }

            if i == text_tokens.len() - 1 {
                let label =
                    Label::create_text(text_label.clone(), address_offset, text_label_offset);

                text_labels.push(label);

                text_label = text_t.label.clone();
            }

            text_label_offset += 4;
        }

        AssemblerLabels {
            data_labels,
            text_labels,
        }
    }

    fn find_label(name: String, labels: AssemblerLabels) -> Label {
        for label in labels.data_labels {
            if label.name == name {
                return label;
            }
        }

        for label in labels.text_labels {
            if label.name == name {
                return label;
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

    fn generate_data_bytes(labels: AssemblerLabels) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for label in labels.data_labels {
            label.value.as_bytes().iter().for_each(|&b| result.push(b));
        }

        result
    }

    // !!! Instructions

    fn inst_ld(_labels: &AssemblerLabels, token: &Token) -> Instruction {
        let reg = Assembler::is_valid_register(token.args[0].clone());

        if &token.args[1][..1] == "$" {
            let addr = Assembler::get_value_from_str(token.args[1].replace('$', ""));

            (
                0x02,
                reg,
                (addr >> 8_u8).try_into().unwrap(),
                (addr & 0xFF).try_into().unwrap(),
            )
        } else if &token.args[1][..1] == "R" {
            let reg2 = Assembler::is_valid_register(token.args[2].clone());

            (0x01, reg, 0x00, reg2)
        } else {
            let addr = Assembler::get_value_from_str(token.args[1].clone());

            (
                0x02,
                reg,
                (addr >> 8_u8).try_into().unwrap(),
                (addr & 0xFF).try_into().unwrap(),
            )
        }
    }
}
