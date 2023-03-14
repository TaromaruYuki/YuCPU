use std::{collections::HashMap, fs::File, io::Read, path::Path};

use colored::Colorize;

use super::common::instruction::Instruction;

use crate::common::hex::Hex;

#[derive(Clone, PartialEq)]
enum InstructionValues {
    Null,
    Val,
    Reg,
    Addr,
}

#[derive(Clone)]
struct InstructionInfo {
    name: String,
    value: InstructionValues,
    uses_reg1: bool,
}

impl InstructionInfo {
    pub fn new(name: String, value: InstructionValues, uses_reg1: bool) -> InstructionInfo {
        InstructionInfo {
            name,
            value,
            uses_reg1,
        }
    }
}

pub struct Disassembler {
    file_in: Box<Path>,
    instruction_names: HashMap<u8, InstructionInfo>,
    use_addr: bool,
}

impl Disassembler {
    pub fn new(file_in: &String, use_addr: bool) -> Disassembler {
        let file_in_path: &Path = Path::new(file_in);

        let instruction_names: HashMap<u8, InstructionInfo> = HashMap::from([
            (
                0x00,
                InstructionInfo::new(String::from("MOV"), InstructionValues::Val, true),
            ),
            (
                0x01,
                InstructionInfo::new(String::from("LD"), InstructionValues::Reg, true),
            ),
            (
                0x02,
                InstructionInfo::new(String::from("LD"), InstructionValues::Addr, true),
            ),
            (
                0x03,
                InstructionInfo::new(String::from("PSH"), InstructionValues::Val, false),
            ),
            (
                0x04,
                InstructionInfo::new(String::from("PSH"), InstructionValues::Null, true),
            ),
            (
                0x05,
                InstructionInfo::new(String::from("PSH"), InstructionValues::Addr, false),
            ),
            (
                0x06,
                InstructionInfo::new(String::from("POP"), InstructionValues::Null, true),
            ),
            (
                0x07,
                InstructionInfo::new(String::from("POP"), InstructionValues::Null, false),
            ),
            (
                0x08,
                InstructionInfo::new(String::from("LDS"), InstructionValues::Addr, true),
            ),
            (
                0x09,
                InstructionInfo::new(String::from("MOV"), InstructionValues::Reg, true),
            ),
            (
                0x0A,
                InstructionInfo::new(String::from("MOV"), InstructionValues::Addr, true),
            ),
            (
                0x10,
                InstructionInfo::new(String::from("ST"), InstructionValues::Addr, true),
            ),
            (
                0x11,
                InstructionInfo::new(String::from("STL"), InstructionValues::Addr, true),
            ),
            (
                0x12,
                InstructionInfo::new(String::from("STH"), InstructionValues::Addr, true),
            ),
            (
                0x20,
                InstructionInfo::new(String::from("CMP"), InstructionValues::Reg, true),
            ),
            (
                0x21,
                InstructionInfo::new(String::from("CMP"), InstructionValues::Val, true),
            ),
            (
                0x30,
                InstructionInfo::new(String::from("BEQ"), InstructionValues::Addr, false),
            ),
            (
                0x31,
                InstructionInfo::new(String::from("BGT"), InstructionValues::Addr, false),
            ),
            (
                0x32,
                InstructionInfo::new(String::from("BLT"), InstructionValues::Addr, false),
            ),
            (
                0x33,
                InstructionInfo::new(String::from("JMP"), InstructionValues::Addr, false),
            ),
            (
                0x34,
                InstructionInfo::new(String::from("BOF"), InstructionValues::Addr, false),
            ),
            (
                0x35,
                InstructionInfo::new(String::from("BNE"), InstructionValues::Addr, false),
            ),
            (
                0x40,
                InstructionInfo::new(String::from("ADD"), InstructionValues::Val, true),
            ),
            (
                0x41,
                InstructionInfo::new(String::from("SUB"), InstructionValues::Val, true),
            ),
            (
                0x42,
                InstructionInfo::new(String::from("ADD"), InstructionValues::Reg, true),
            ),
            (
                0x43,
                InstructionInfo::new(String::from("SUB"), InstructionValues::Reg, true),
            ),
            (
                0x50,
                InstructionInfo::new(String::from("CALL"), InstructionValues::Addr, false),
            ),
            (
                0x51,
                InstructionInfo::new(String::from("RET"), InstructionValues::Null, false),
            ),
            (
                0xFE,
                InstructionInfo::new(String::from("HLT"), InstructionValues::Null, false),
            ),
            (
                0xFF,
                InstructionInfo::new(String::from("NOP"), InstructionValues::Null, false),
            ),
        ]);

        Disassembler {
            file_in: Into::into(file_in_path),
            instruction_names,
            use_addr,
        }
    }

    fn get_instructions(file_in: &Path) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = Vec::new();

        let mut file = match File::open(file_in) {
            Err(why) => panic!("Opening file \"{}\" failed!\n\n{}", file_in.display(), why),
            Ok(file) => file,
        };

        let mut buffer = [0; 2];

        let n = match file.read(&mut buffer[..]) {
            Err(why) => panic!(
                "Reading program offset for file \"{}\" failed!\n\n{}",
                file_in.display(),
                why
            ),
            Ok(n) => n,
        };

        if n != 2 {
            panic!("Reading program offset resulted in non 2 length read. Make sure file \"{}\" is not empty or came from the YuCPU compiler!", file_in.display());
        }

        loop {
            let mut buffer = [0; 4];

            let n = match file.read(&mut buffer[..]) {
                Err(why) => panic!("Reading file \"{}\" failed!\n\n{}", file_in.display(), why),
                Ok(n) => n,
            };

            if n < 3 {
                break;
            }

            res.push(Instruction::new_source(
                buffer[0],
                buffer[1],
                buffer[2],
                buffer[3],
                buffer.to_vec(),
            ));
        }

        res
    }

    fn source_to_hex_str(source: &[u8]) -> String {
        let mut res = String::from("");

        for byte in source.iter() {
            res += &byte.to_hex_string();
        }

        res
    }

    fn reg_to_string(reg: u8) -> String {
        if reg == 0 || reg <= 5 {
            format!("{}{}", "R".magenta(), reg + 1)
        } else if reg == 6 {
            format!("{}", "RPC".magenta())
        } else if reg == 7 {
            format!("{}", "RSP".magenta())
        } else if reg == 8 {
            format!("{}", "RBP".magenta())
        } else {
            panic!("Invalid register.");
        }
    }

    pub fn disassemble(self) {
        let instructions = Disassembler::get_instructions(&self.file_in);

        let header_padding: &str = if self.use_addr {
            "            "
        } else {
            "         "
        };

        let mut res = format!(
            "Disassembly of file {}:\n\n{}Source        Assembly\n",
            &self.file_in.display(),
            header_padding
        );

        let mut counter: u16 = 0;

        for instruction in instructions {
            let disp_addr: String;
            let counter_padd: u16;

            if self.use_addr {
                disp_addr = (0xDAC1 + counter).to_hex_string();
                counter_padd = 0;
            } else {
                disp_addr = format!("{}", counter);
                counter_padd = counter + 1;
            }

            let instruction_info = self.instruction_names[&instruction.opcode].clone();

            let source_hex = Disassembler::source_to_hex_str(&instruction.source);
            let source_padding =
                " ".repeat((16 - source_hex.len()) - (counter_padd).to_string().len());

            res += &format!(
                "{}{}{}      ",
                disp_addr,
                source_padding,
                source_hex.yellow()
            );

            let inst_padding = " ".repeat(5 - instruction_info.name.len());

            if !instruction_info.uses_reg1 && instruction_info.value == InstructionValues::Null {
                res += &format!("{}", instruction_info.name.cyan());
            } else {
                res += &format!("{}{}", instruction_info.name.cyan(), inst_padding);
            }

            if instruction_info.uses_reg1 {
                res += &format!("{}, ", Disassembler::reg_to_string(instruction.register));
            }

            if instruction_info.value == InstructionValues::Val {
                res += &format!(
                    "{}{}",
                    "0x".yellow(),
                    instruction.data.to_hex_string().yellow()
                );
            } else if instruction_info.value == InstructionValues::Addr {
                res += &format!(
                    "{}{}",
                    "$0x".red(),
                    instruction.data.to_hex_string().red().red()
                );
            } else if instruction_info.value == InstructionValues::Reg {
                res += &Disassembler::reg_to_string(instruction.data as u8);
            }

            res += "\n";
            counter += 4;
        }

        println!("{}", res);
    }
}
