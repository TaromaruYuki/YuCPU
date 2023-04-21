mod assembler;
pub mod common;
mod vcpu;

use assembler::parser::{Parser, TokenInfoType};
use assembler::tokenizer::Token;
use assembler::Assembler;
use clap::{Parser as ClapParser, Subcommand};
use itertools::Itertools;
use logos::Logos;
use std::fs::{self, File};
use std::io::{BufRead, Read};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use common::instruction::opcode::{AddressingMode, Instruction, Opcode};

#[derive(clap::Parser, Debug)]
#[command(name = "YuCPU", version)]
#[command(author = "TaromaruYuki")]
#[command(about = "A custom CPU created by TaromaruYuki", long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, about = "Assemble YuCPU Assembly.")]
    Assemble {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },

    #[command(arg_required_else_help = true, about = "Run the YuCPU PC.")]
    Run {
        #[arg(short, long)]
        input: PathBuf,
    },

    #[command(
        arg_required_else_help = false,
        about = "Generate a markdown opcode table."
    )]
    OpcodeTable,

    #[command(
        arg_required_else_help = true,
        about = "Get information about a specific instruction."
    )]
    Instruction { instruction: String },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Assemble { input, output } => {
            if !input.as_path().exists() {
                eprintln!("Input file \"{:?}\" does not exist.", input);
            }

            let mut input_content: String = String::from("");

            let file = match File::open(&input) {
                Err(why) => panic!("Opening file \"{:?}\" failed!\n\n{}", input, why),
                Ok(file) => file,
            };

            let buffer = std::io::BufReader::new(file);

            for line in buffer.lines().flatten() {
                input_content.push_str(&format!("{}\n", line));
            }

            let mut lex = Token::lexer(&input_content);
            let mut tokens: Vec<TokenInfoType> = Vec::new();

            loop {
                let tok_option = lex.next();
                let slice = lex.slice();

                if tok_option.is_none() {
                    break;
                }

                let tok = tok_option.unwrap();

                tokens.push((tok, String::from(slice)));
            }

            let mut parser = Parser::new(tokens);
            let parser_res = parser.parse();

            let assembler = Assembler::new(parser_res);
            let bytecode = assembler.assemble();

            match fs::write(output, bytecode) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Unable to write output file.\n{error}");
                    exit(1);
                }
            };
        }
        Commands::Run { input } => {
            // Check if the input file exists

            if !input.as_path().exists() {
                eprintln!("Input file \"{:?}\" does not exist.", input);
            }

            let mut file = File::open(&input).unwrap();
            let mut buf_file_size = [0_u8; 2];
            file.read_exact(&mut buf_file_size).unwrap();
            let file_size = u16::from_be_bytes(buf_file_size);
            println!("Size: {}", file_size);

            let mut program = vec![0; file_size as usize];
            file.read_exact(&mut program.as_mut_slice()).unwrap();
            println!("{:?}", program);

            let mut ivt_buf = [0; 510];
            file.read_exact(&mut ivt_buf).unwrap();

            vcpu::run(program, ivt_buf);
        }
        Commands::OpcodeTable => {
            let hashmap = common::instruction::opcode::Instruction::hashmap();

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
                    let mut get: Option<common::instruction::instructions::InstructionInfo> = None;

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

                            table += &format!(
                                "| 0x{:02X}<br/>{:?} {} ",
                                (i << 4) | j,
                                value.0,
                                value_type
                            );
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
        Commands::Instruction { instruction } => {
            let opcode = match Opcode::from_str(&instruction.to_lowercase()) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!(
                        "Instruction {} does not exist.\nReceived error {}",
                        instruction, err
                    );
                    exit(1);
                }
            };

            let variants = Instruction::get_variants(opcode);

            println!(
                "=== Opcode {:?} ===\nBinary: {:06b}\nHex: 0x{:01x}\nModes:",
                opcode, opcode as u8, opcode as u8
            );

            for mode in variants {
                let final_opcode = Instruction::create_opcode(opcode, mode);
                println!("    - {:?} ({:02b})", mode, mode as u8);
                println!("        - {:08b}", final_opcode);
                println!("        - 0x{:02x}", final_opcode);
            }
        }
    }
}
