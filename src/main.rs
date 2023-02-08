mod assembler;
pub mod common;

use std::path::Path;
use std::{env, process::exit};

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

        let assembler = assembler::Assembler::new(&input_file, &output_file);

        assembler.assemble();

        exit(0);
    } // End of assemble
}
