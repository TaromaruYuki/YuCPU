use super::Assembler;

// region: instructions

#[test]
fn test_instruction_ld_v_hex() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x00, 0x00, 0xDE, 0xAD]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tLD R0, 0xDEAD"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_ld_r() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x01, 0x00, 0x00, 0x01]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tLD R0, R1"),
    ]));

    let bytes = assembler.assemble();

    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_ld_a_hex() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x02, 0x02, 0x00, 0x50]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tLD R2, $0x50"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_psh_v() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x03, 0x00, 0xDE, 0xAD]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tPSH 0xDEAD"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_psh_r() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x04, 0x06, 0x00, 0x00]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tPSH R6"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_psh_a() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x05, 0x00, 0x01, 0xF4]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tPSH $500"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_pop_r() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x06, 0x02, 0x00, 0x00]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tPOP R2"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_pop() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x07, 0x00, 0x00, 0x00]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tPOP"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_lds_a() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x08, 0x01, 0xBE, 0xEF]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tLDS R1, $0xBEEF"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_st_a() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x10, 0x00, 0xBE, 0xEF]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tST R0, $0xBEEF"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_stl_a() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x11, 0x03, 0x06, 0x17]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tSTL R3, $0x617"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_sth_a() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x12, 0x04, 0x0D, 0x06]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tSTH R4, $0xD06"), // ;)
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_cmp_r() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x20, 0x00, 0x00, 0x01]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tCMP R0, R1"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_cmp_v() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x21, 0x00, 0x0C, 0xA7]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tCMP R0, 0xCA7"), // >:(
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_beq_l() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x30, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tBEQ start"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_bgt_l() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x31, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tBGT start"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_blt_l() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x32, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tBLT start"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_jmp_l() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x33, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tJMP start"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_bof_l() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x34, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tBOF start"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_add_v() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x40, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tADD R0, 2"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_sub_v() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x41, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tSUB R0, 2"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_add_r() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x42, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tADD R0, R2"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_sub_r() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0x43, 0x00, 0x00, 0x02]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tSUB R0, R2"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_hlt() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0xFE, 0xFE, 0xFF, 0xFF]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tHLT"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_instruction_nop() {
    let expected_bytes: Vec<u8> = Vec::from([0x00, 0x02, 0xFF, 0xFF, 0xFF, 0xFF]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tNOP"),
    ]));

    let bytes = assembler.assemble();
    assert_eq!(bytes.0, expected_bytes);
}

// endregion

#[test]
fn test_start_label() {
    let mut assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tLD R0, 16"),
    ]));

    assembler = assembler.assemble().1;

    assert_eq!(assembler.start_label, String::from("start"))
}

#[test]
fn test_data_string() {
    let expected_bytes: Vec<u8> = Vec::from([
        0x00, 0x10, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        0x00, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    let assembler = Assembler::new_lines(Vec::from([
        String::from(".main start"),
        String::from(".data"),
        String::from("@str1:"),
        String::from("\tDAT \"Hello, world!\""),
        String::from(".text"),
        String::from("@start:"),
        String::from("\tHLT"),
    ]));

    let bytes = assembler.assemble();

    assert_eq!(bytes.0, expected_bytes);
}

#[test]
fn test_string_to_value_decimal() {
    // let assembler = Assembler::new_lines(Vec::new());
    let value = Assembler::get_value_from_str(String::from("65535"));

    assert_eq!(value, 65535);
}

#[test]
fn test_string_to_value_hex() {
    // let assembler = Assembler::new_lines(Vec::new());
    let value = Assembler::get_value_from_str(String::from("0xFFFF"));

    assert_eq!(value, 65535);
}
