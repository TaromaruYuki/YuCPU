#![allow(dead_code)]
#![allow(unused_imports)]

mod label;
mod token;

use label::Label;
use label::LabelType;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use token::Token;

pub struct Assembler {
    pub file_in: Box<Path>,
    pub file_out: Box<Path>,

    start_label: String,
}

impl Assembler {
    pub fn new(f_in: &String, f_out: &String) -> Self {
        let file_in_path: &Path = Path::new(f_in);
        let file_out_path: &Path = Path::new(f_out);

        Assembler {
            file_in: Into::into(file_in_path),
            file_out: Into::into(file_out_path),
            start_label: String::from(""),
        }
    }
}

pub struct AssemblerLabels {
    data_labels: Vec<Label>,
    text_labels: Vec<Label>,
}

impl Assembler {
    pub fn assemble(mut self) {
        let tokens: (String, Vec<Token>, Vec<Token>) = self.tokenize();

        if !tokens.0.is_empty() {
            self.start_label = tokens.0.clone();
        }

        let _labels: AssemblerLabels = self.parse_labels(tokens.1, tokens.2);
    }

    // fn labels_to_bytes(labels: AssemblerLabels) -> Vec<u8> {
    //     let mut bytes: Vec<u8> = Vec::new();

    //     for label in labels.data_labels {

    //     }
    // }

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

        if &reg[0..0] != "R" {
            println!("Invalid register name: \"{}\"", reg);
        }

        let reg_num: u8 = reg[1..1].parse::<u8>().unwrap();

        if reg_num > 6 {
            println!("Register {} out of range", reg);
        }

        reg_num
    }

    fn parse_labels(&self, data_tokens: Vec<Token>, text_tokens: Vec<Token>) -> AssemblerLabels {
        let mut data_labels: Vec<Label> = Vec::new();
        let mut text_labels: Vec<Label> = Vec::new();
        let mut used_data_labels: Vec<String> = Vec::new();

        let mut address_offset = 0;
        let mut text_label = text_tokens[0].label.clone();
        let mut text_label_offset = 0;

        for data_t in data_tokens {
            // We can just assume that the instruction is "DAT"
            // There is no other instruction that can be used in the data section

            if data_t.instruction != "DAT" {
                println!(
                    "Invalid instruction in data label {}: {}",
                    data_t.label, data_t.instruction
                );
                exit(1);
            }

            if data_t.args.len() != 1 {
                println!(
                    "Invalid number of arguments in data label {}: {}",
                    data_t.label,
                    data_t.args.len()
                );
                exit(1);
            }

            if used_data_labels.contains(&data_t.label) {
                println!(
                    "Label {} already has a DAT. Labels cannot have multiple DATs.",
                    data_t.label
                );
            }

            used_data_labels.push(data_t.label.clone());

            let mut label = Label::new();
            label.name = data_t.label.clone();
            label.l_type = data_t.section;
            label.addr = address_offset;

            address_offset += label.name.len() as u16;
            address_offset += 1;

            data_labels.push(label);
        }

        for text_t in text_tokens {
            if text_t.label != text_label {
                let mut label = Label::new();

                label.name = text_label.clone();
                label.l_type = text_t.section;
                label.addr = address_offset + text_label_offset;

                text_label = text_t.label;

                text_labels.push(label);
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

    fn get_value_from_str(str: String) -> i16 {
        // 0(x or X)(Any amount of characters, 0-9, A-F, and a-f)
        // Examples: 0x14F, 0Xfac
        let reg = Regex::new(r"^0[xX][0-9A-Fa-f]+$").unwrap();

        if reg.is_match(&str) {
            i16::from_str_radix(&str[2..], 16).unwrap()
        } else {
            str.parse::<i16>().unwrap()
        }
    }

    fn generate_data_bytes(labels: AssemblerLabels) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for label in labels.data_labels {
            label.value.as_bytes().iter().for_each(|&b| result.push(b));
        }

        result
    }
}
