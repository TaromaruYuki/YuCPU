mod assembler;
pub mod common;
mod disassembler;
mod vcpu;

use itertools::Itertools;
use std::fs::{self};
use std::path::Path;
use std::{env, process::exit};
use vcpu::instruction::opcode::AddressingMode;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        eprintln!("No arguments specified.\n\nUse `YuCPU -h` for help.");
        exit(1);
    }

    if args.contains(&String::from("-h")) {
        println!(
            "The YuCPU CLI.\n\nUsage: YuCPU [task] [options]\n\nTasks:\n  assemble\tAssemble a file\n  deassemble\tDessemble a file\n  run\tRun a file\n\nOptions:\n  -i\tInput file\n  -o\tOutput file\n  -h\t Shows this help menu\n\nAny extra options are ignored.\n",
        );

        exit(0);
    }

    if args[0].to_lowercase() == "assemble" {
        // Get input & output file in args

        let input_pos_res = args.iter().position(|r| r == "-i");

        if input_pos_res.is_none() {
            eprintln!("Input is required for task \"assemble\".");
            exit(1);
        }

        let output_pos_res = args.iter().position(|r| r == "-o");

        if output_pos_res.is_none() {
            eprintln!("Output is required for task \"assemble\".");
            exit(1);
        }

        let input_pos = input_pos_res.unwrap();
        let output_pos = output_pos_res.unwrap();

        // Checking if the input or output is the final arg

        if input_pos == args.len() - 1 {
            eprintln!("Input needs a argument.");
            exit(1);
        }

        if output_pos == args.len() - 1 {
            eprintln!("Output needs a argument.");
            exit(1);
        }

        // Check if we have a valid argument (Just see if it's not a flag, then the OS can check)

        if args[input_pos + 1].starts_with('-') {
            eprintln!("Invalid input file.");
            exit(1);
        }

        if args[output_pos + 1].starts_with('-') {
            eprintln!("Invalid output file.");
            exit(1);
        }

        let input_file = args[input_pos + 1].clone();
        let output_file = args[output_pos + 1].clone();

        // Check if the input file exists

        if !Path::new(&args[input_pos + 1]).exists() {
            eprintln!("Input file \"{}\" does not exist.", args[input_pos + 1]);
            exit(1);
        }

        let yu_assembler = assembler::Assembler::new(&input_file, &output_file);

        let bytes = yu_assembler.assemble();

        match fs::write(output_file, bytes.0) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to write output file.\n{error}");
                exit(1);
            }
        };

        exit(0);
    } else if args[0].to_lowercase() == "disassemble" {
        // End of assemble
        // Get input file in args

        let input_pos_res = args.iter().position(|r| r == "-i");

        if input_pos_res.is_none() {
            eprintln!("Input is required for task \"disassemble\".");
            exit(1);
        }

        let input_pos = input_pos_res.unwrap();

        // Checking if the input is the final arg

        if input_pos == args.len() - 1 {
            eprintln!("Input needs a argument.");
            exit(1);
        }

        // Check if we have a valid argument (Just see if it's not a flag, then the OS can check)

        if args[input_pos + 1].starts_with('-') {
            eprintln!("Invalid input file.");
            exit(1);
        }

        let input_file = args[input_pos + 1].clone();

        // Check if the input file exists

        if !Path::new(&args[input_pos + 1]).exists() {
            eprintln!("Input file \"{}\" does not exist.", args[input_pos + 1]);
            exit(1);
        }

        let disassembler =
            disassembler::Disassembler::new(&input_file, args.contains(&String::from("-a")));

        disassembler.disassemble();

        exit(0);
    } else if args[0].to_lowercase() == "run" {
        // End of disassemble
        // Get input file in args

        let input_pos_res = args.iter().position(|r| r == "-i");

        if input_pos_res.is_none() {
            eprintln!("Input is required for task \"run\".");
            exit(1);
        }

        let input_pos = input_pos_res.unwrap();

        // Checking if the input is the final arg

        if input_pos == args.len() - 1 {
            eprintln!("Input needs a argument.");
            exit(1);
        }

        // Check if we have a valid argument (Just see if it's not a flag, then the OS can check)

        if args[input_pos + 1].starts_with('-') {
            eprintln!("Invalid input file.");
            exit(1);
        }

        let input_file = args[input_pos + 1].clone();

        // Check if the input file exists

        if !Path::new(&args[input_pos + 1]).exists() {
            eprintln!("Input file \"{}\" does not exist.", args[input_pos + 1]);
            exit(1);
        }

        let file = match fs::read(&input_file) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to open file \"{input_file}\".\n{error}");
                exit(1);
            }
        };

        vcpu::run(file);
    } else if args[0].to_lowercase() == "opcode_table" {
        let hashmap = vcpu::instruction::opcode::Instruction::hashmap();

        let mut table = String::from("|     ");

        for i in 0x00..0x10 {
            table += &format!("| 0x{:01X} ", i);
        }

        table += "|\n";

        for _ in 0x00..0x11 {
            table += "| --- ";
        }

        table += "|\n";

        for i in 0x00..0x10 {
            table += &format!("| 0x{:01X} ", i);
            for j in 0x00..0x10 {
                let mut get: Option<vcpu::instruction::instructions::InstructionInfo> = None;

                for key in hashmap.keys().sorted() {
                    if key == &(((i << 4) | j) as u8) {
                        get = Some(hashmap[key]);
                        break;
                    }
                }

                match get {
                    Some(value) => {
                        let value_type = match value.1 {
                            AddressingMode::Immediate => "V",
                            AddressingMode::Register => "R",
                            AddressingMode::Direct => "A/L",
                            AddressingMode::Discard => "",
                        };

                        table +=
                            &format!("| 0x{:02X}<br/>{:?} {} ", (i << 4) | j, value.0, value_type);
                    }
                    None => {
                        table += "|   ";
                    }
                }
            }
            table += "|\n"
        }

        match fs::write("opcodes.md", table) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Unable to write output file.\n{error}");
                exit(1);
            }
        };
    }
}
