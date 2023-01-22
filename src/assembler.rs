#![allow(dead_code)]
#![allow(unused_imports)]

mod label;

use label::Label;
use label::LabelType;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub struct Assembler {
    pub file_in: Box<Path>,
    pub file_out: Box<Path>,

    start_label: String,
}

impl Assembler {
    pub fn new(f_in: &String, f_out: &String) -> Self {
        let file_in_path = Path::new(f_in);
        let file_out_path = Path::new(f_out);

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
    pub fn assemble(self: Self) {
        let lines = self.read_input_file();

        let mut i: u16 = 1;

        for line in lines {
            println!("Line {}: {}", i, line);

            i += 1;
        }
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

    fn parse_labels(mut self: Self) { // -> AssemblerLabels
        let mut data_labels: Vec<Label> = Vec::new();
        let mut text_labels: Vec<Label> = Vec::new();
        let lines = self.read_input_file();

        // Go through the file looking for the start label

        for line in lines {
            if(&line[..0] == ".") {
                let mut cloned_line = line.clone();
                let split = cloned_line.split(" ").collect::<Vec<&str>>();
                
                if(split[0] == ".main") {
                    self.start_label = split[1].to_string();
                }
            }
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

    fn read_input_file(self: Self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let display = self.file_in.display();

        let file = match File::open(&self.file_in) {
            Err(why) => panic!("Opening file \"{}\" failed!\n\n{}", display, why),
            Ok(file) => file,
        };

        let buffer = std::io::BufReader::new(file);

        for line in buffer.lines() {
            if let Ok(line_val) = line {
                result.push(line_val);
            }
        }

        result
    }

    fn get_value_from_str(str: String) -> i16 {
        let reg = Regex::new(r"^0[xX][0-9A-Fa-f]+$").unwrap();

        if reg.is_match(&str) {
            return i16::from_str_radix(&str[2..], 16).unwrap();
        } else {
            return str.parse::<i16>().unwrap();
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
