#![allow(unused_assignments)]

use std::sync::{Arc, Mutex};

use crate::vcpu::{
    cpu::{Flags, CPU},
    device::{
        map::{DeviceMap, DeviceMapResult},
        ram::Ram,
        rom::Rom,
    },
};

use crate::common::instruction::opcode::{AddressingMode, Opcode};

use super::opcode::Instruction;

#[test]
fn test_from_opcode_instruction() {
    let result = match Instruction::from_opcode(&(0b00_000000 as u8)) {
        Ok(res) => res,
        Err(_) => panic!("Opcode does not exist."),
    };

    let result2 = match Instruction::from_opcode(&(0b10_001111 as u8)) {
        Ok(res) => res,
        Err(_) => panic!("Opcode does not exist."),
    };

    assert_eq!(result.opcode, Opcode::MOV);
    assert_eq!(result.mode, AddressingMode::Immediate);

    assert_eq!(result2.opcode, Opcode::JSR);
    assert_eq!(result2.mode, AddressingMode::Direct);
}

#[test]
#[should_panic]
fn test_from_opcode_instruction_fail() {
    match Instruction::from_opcode(&(0b101_00000 as u8)) {
        Ok(res) => res,
        Err(_) => panic!("Opcode does not exist."),
    };
}

#[test]
fn test_mov_immediate_byte() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x00, 0x00, 0xAB], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xAB);
}

#[test]
fn test_mov_immediate_word() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x00, 0x04, 0xAB, 0xCD],
        0x0000,
        4,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xABCD);
}

#[test]
fn test_mov_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x40, 0x00, 0x1], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r2 = 0xABCD;
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xABCD);
    assert_eq!(cpu.r2, 0xABCD);
}

#[test]
fn test_ld_register() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x41, 0x00, 0x1, 0xAB, 0xCD],
        0x0000,
        5,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r2 = 0x0003;
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xABCD);
}

#[test]
fn test_ld_address() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x81, 0x00, 0x03, 0xAB, 0xCD],
        0x0000,
        5,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xABCD);
}

#[test]
fn test_ldb_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x42, 0x00, 0x1, 0xCD], 0x0000, 4)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r2 = 0x0003;
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xCD);
}

#[test]
fn test_ldb_address() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x82, 0x00, 0x03, 0xCD],
        0x0000,
        4,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xCD);
}

#[test]
fn test_cmp_immediate_eq() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x08, 0x20, 0x5], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r3 = 0x5;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert!(cpu.flags.contains(Flags::Z));
    assert!(!cpu.flags.contains(Flags::G));
    assert!(!cpu.flags.contains(Flags::L));
}

#[test]
fn test_cmp_immediate_lt() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x08, 0x20, 0x5], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r3 = 0x4;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert!(cpu.flags.contains(Flags::L));
    assert!(!cpu.flags.contains(Flags::Z));
    assert!(!cpu.flags.contains(Flags::G));
}

#[test]
fn test_cmp_immediate_gt() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x08, 0x20, 0x5], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r3 = 0x6;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert!(cpu.flags.contains(Flags::G));
    assert!(!cpu.flags.contains(Flags::L));
    assert!(!cpu.flags.contains(Flags::Z));
}

#[test]
fn test_cmp_register_eq() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x48, 0x20, 0x0], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r3 = 0x5;
    cpu.r1 = 0x5;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert!(cpu.flags.contains(Flags::Z));
    assert!(!cpu.flags.contains(Flags::G));
    assert!(!cpu.flags.contains(Flags::L));
}

#[test]
fn test_cmp_register_lt() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x48, 0x20, 0x0], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r3 = 0x4;
    cpu.r1 = 0x5;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert!(cpu.flags.contains(Flags::L));
    assert!(!cpu.flags.contains(Flags::Z));
    assert!(!cpu.flags.contains(Flags::G));
}

#[test]
fn test_cmp_register_gt() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x48, 0x20, 0x0], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r3 = 0x6;
    cpu.r1 = 0x5;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert!(cpu.flags.contains(Flags::G));
    assert!(!cpu.flags.contains(Flags::L));
    assert!(!cpu.flags.contains(Flags::Z));
}

fn test_branch_flag_is_set(opcode: u8, flag: Flags) {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![opcode, 0x00, 0x05, 0xFF, 0x0C, 0xFF, 0x0C],
        0x0000,
        7,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.flags.set(flag, true);

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x05);
}

fn test_no_branch_flag_is_not_set(opcode: u8, flag: Flags) {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![opcode, 0x00, 0x05, 0xFF, 0x0C, 0xFF, 0x0C],
        0x0000,
        7,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.flags.set(flag, false);

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x03);
}

fn test_branch_flag_if_not_set(opcode: u8, flag: Flags) {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![opcode, 0x00, 0x05, 0xFF, 0x0C, 0xFF, 0x0C],
        0x0000,
        7,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.flags.set(flag, false);

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x05);
}

fn test_no_branch_flag_is_set(opcode: u8, flag: Flags) {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![opcode, 0x00, 0x05, 0xFF, 0x0C, 0xFF, 0x0C],
        0x0000,
        7,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.flags.set(flag, true);

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x03);
}

#[test]
fn test_beq_is_set() {
    test_branch_flag_is_set(0x89, Flags::Z);
}

#[test]
fn test_beq_is_not_set() {
    test_no_branch_flag_is_not_set(0x89, Flags::Z);
}

#[test]
fn test_bgt_is_set() {
    test_branch_flag_is_set(0x8A, Flags::G);
}

#[test]
fn test_bgt_is_not_set() {
    test_no_branch_flag_is_not_set(0x8A, Flags::G);
}

#[test]
fn test_blt_is_set() {
    test_branch_flag_is_set(0x8B, Flags::L);
}

#[test]
fn test_blt_is_not_set() {
    test_no_branch_flag_is_not_set(0x8B, Flags::L);
}

#[test]
fn test_bof_is_set() {
    test_branch_flag_is_set(0x8C, Flags::O);
}

#[test]
fn test_bof_is_not_set() {
    test_no_branch_flag_is_not_set(0x8C, Flags::O);
}

#[test]
fn test_bne_is_not_set() {
    test_branch_flag_if_not_set(0x8D, Flags::Z);
}

#[test]
fn test_bne_is_set() {
    test_no_branch_flag_is_set(0x8D, Flags::Z);
}

#[test]
fn test_jmp() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x8E, 0x00, 0x05, 0xFF, 0x0C, 0xFF, 0x0C],
        0x0000,
        7,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x05);
}

#[test]
fn test_hlt() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0xFE, 0x0C, 0xFF, 0x0C],
        0x0000,
        4,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x0000);
    assert_eq!(cpu.running, false);
}

#[test]
fn test_nop() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0xFF, 0x0C, 0xFE, 0x0C],
        0x0000,
        4,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x0002);
}

#[test]
fn test_add_immediate_normal() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x10, 0x00, 0x5], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x2;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0x7);
    assert!(!cpu.flags.contains(Flags::O));
}

#[test]
fn test_add_immediate_overflow() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x10, 0x04, 0xFF, 0xFE],
        0x0000,
        4,
    )));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x2;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0x00); // Wraps the value when a overflow occurs.
    assert!(cpu.flags.contains(Flags::O));
}

#[test]
fn test_add_register_normal() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x50, 0x00, 0x1], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x2;
    cpu.r2 = 0x5;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0x7);
    assert!(!cpu.flags.contains(Flags::O));
}

#[test]
fn test_add_register_overflow() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x50, 0x00, 0x01], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x2;
    cpu.r2 = 0xFFFE;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0x00); // Wraps the value when a overflow occurs.
    assert!(cpu.flags.contains(Flags::O));
}

#[test]
fn test_sub_immediate_normal() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x11, 0x00, 0x2], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x5;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0x3);
    assert!(!cpu.flags.contains(Flags::O));
}

#[test]
fn test_sub_immediate_overflow() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x11, 0x00, 0x01], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x0;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xFFFF); // Wraps the value when a overflow occurs.
    assert!(cpu.flags.contains(Flags::O));
}

#[test]
fn test_sub_register_normal() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x51, 0x00, 0x1], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x5;
    cpu.r2 = 0x2;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0x3);
    assert!(!cpu.flags.contains(Flags::O));
}

#[test]
fn test_sub_register_overflow() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x51, 0x00, 0x01], 0x0000, 3)));

    let mut map = DeviceMap::new();
    map.add(rom);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;

    cpu.r1 = 0x00;
    cpu.r2 = 0x01;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xFFFF); // Wraps the value when a overflow occurs.
    assert!(cpu.flags.contains(Flags::O));
}

#[test]
fn test_psh_immediate() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x03, 0x00, 0x05], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0x07, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.sp, 0x09);
    assert_eq!(
        match cpu.map.read((cpu.sp - 2) as u32) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x05
    );
}

#[test]
fn test_psh_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x43, 0x0C], 0x0000, 2)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0x07, false);
    cpu.map = map;
    cpu.r1 = 0x05;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.sp, 0x09);
    assert_eq!(
        match cpu.map.read((cpu.sp - 2) as u32) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x05
    );
}

#[test]
fn test_psh_address() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x83, 0x00, 0x1E], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));
    {
        let mut locked_ram = ram.lock().unwrap();
        locked_ram.memory[0x17] = 0xAB;
        locked_ram.memory[0x18] = 0xCD;
    }

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0x07, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.sp, 0x09);
    assert_eq!(
        match cpu.map.read((cpu.sp - 2) as u32) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0xABCD
    );
}

#[test]
fn test_pop_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x44, 0x0C], 0x0000, 2)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));
    {
        let mut locked_ram = ram.lock().unwrap();
        locked_ram.memory[0x00] = 0xAB;
        locked_ram.memory[0x01] = 0xCD;
    }

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0x09, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xABCD);
    assert_eq!(cpu.sp, 0x07);
}

#[test]
fn test_pop() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0xC4, 0x0C], 0x0000, 2)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));
    {
        let mut locked_ram = ram.lock().unwrap();
        locked_ram.memory[0x00] = 0xAB;
        locked_ram.memory[0x01] = 0xCD;
    }

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0x09, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.sp, 0x07);
}

#[test]
fn test_st_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x45, 0x00, 0x01], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r1 = 0xD07;
    cpu.r2 = 0x07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xD07);
    assert_eq!(
        match cpu.map.read(0x07) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0xD07
    );
}

#[test]
fn test_st_address() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x85, 0x00, 0x07], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r1 = 0xD07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xD07);
    assert_eq!(
        match cpu.map.read(0x07) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0xD07
    );
}

#[test]
fn test_stl_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x46, 0x00, 0x01], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r1 = 0xD07;
    cpu.r2 = 0x07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xD07);
    assert_eq!(
        match cpu.map.read_byte(0x07) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x07
    );
}

#[test]
fn test_stl_address() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x86, 0x00, 0x07], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r1 = 0xD07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xD07);
    assert_eq!(
        match cpu.map.read_byte(0x07) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x07
    );
}

#[test]
fn test_sth_register() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x47, 0x00, 0x01], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r1 = 0xD07;
    cpu.r2 = 0x07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xD07);
    assert_eq!(
        match cpu.map.read_byte(0x07) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x0D
    );
}

#[test]
fn test_sth_address() {
    let rom = Arc::new(Mutex::new(Rom::new(vec![0x87, 0x00, 0x07], 0x0000, 3)));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.r1 = 0xD07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.r1, 0xD07);
    assert_eq!(
        match cpu.map.read_byte(0x07) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x0D
    );
}

#[test]
fn test_jsr() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0x8F, 0x00, 0x05, 0xFF, 0x0C, 0xFF, 0x0C],
        0x0000,
        7,
    )));
    let ram = Arc::new(Mutex::new(Ram::new(0x07, 0x20)));

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0xFFFF, false);
    cpu.map = map;
    cpu.sp = 0x07;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x05);
    assert_eq!(
        match cpu.map.read((cpu.sp - 2) as u32) {
            DeviceMapResult::Ok(val) => val,
            DeviceMapResult::NoDevices => panic!("Received no devices in map for jsr test"),
            DeviceMapResult::Error(err) => panic!("Received result {:?} in map for jsr test", err),
        },
        0x03
    );
}

#[test]
fn test_ret() {
    let rom = Arc::new(Mutex::new(Rom::new(
        vec![0xD2, 0x00, 0xFF, 0x0C, 0xFE, 0x0C],
        0x0000,
        6,
    )));
    let ram = Arc::new(Mutex::new(Ram::new(0x06, 0x20)));
    {
        let mut locked_ram = ram.lock().unwrap();
        locked_ram.memory[0x01] = 0x04;
    }

    let mut map = DeviceMap::new();
    map.add(rom);
    map.add(ram);

    let mut cpu = CPU::new(0x0000, 0x08, false);
    cpu.map = map;

    let mut pins = cpu.pins;

    pins = cpu.tick(pins);

    assert_eq!(cpu.pc, 0x04);
}
