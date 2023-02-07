mod assembler;
pub mod common;

fn main() {
    let assembler = assembler::Assembler::new(&String::from("test.txt"), &String::from("test.bin"));

    assembler.assemble();
}
