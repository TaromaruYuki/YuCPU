#![allow(dead_code)]

use std::{collections::HashMap, fs, process::exit};

use crate::common::{hex::Hex, instruction::Instruction};

use self::{bus::DataBus, cpu_dump::CPUDump, registers::Registers};

mod bus;
mod cpu_dump;
mod registers;

type InstructionExecuteFunction = fn(&mut VCPU, &Instruction);

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
pub enum DumpAction {
    File,
    Struct,
}

#[allow(clippy::upper_case_acronyms)]
pub struct VCPU {
    pub data_bus: DataBus,
    pub registers: Registers,
    inst_map: HashMap<u8, InstructionExecuteFunction>,
    start_addr: u16,
    end_of_program: u16,
}

impl VCPU {
    pub fn new() -> VCPU {
        VCPU {
            data_bus: DataBus::new(),
            registers: Registers::new(),
            inst_map: HashMap::new(),
            start_addr: 0xDAC1,
            end_of_program: 0x0000,
        }
    }
}

impl VCPU {
    pub fn reset(&mut self) {
        self.data_bus.reset();

        self.registers.r1 = 0x00;
        self.registers.r2 = 0x00;
        self.registers.r3 = 0x00;
        self.registers.r4 = 0x00;
        self.registers.r5 = 0x00;
        self.registers.r6 = 0x00;
        self.registers.pc = 0xDAC1;
        self.registers.sp = 0x00;
        self.registers.bp = 0x00;
        self.registers.flags = 0x00;
    }

    pub fn load_program(&mut self, mut program: Vec<u8>) {
        if program.is_empty() {
            panic!("Empty program is not valid!");
        }

        let start_addr = ((program[0] as u16) << 8) | program[1] as u16;
        program.drain(..2);

        self.start_addr = start_addr;

        self.data_bus.mem_copy(0xDAC1, &program);
        self.end_of_program = (program.len() + 0xDAC1) as u16;
    }

    pub fn run(&mut self) {
        self.fill_hashmap();

        self.registers.pc = self.start_addr;

        while self.registers.pc < self.end_of_program {
            let opcode = self.data_bus.read_panic(self.registers.pc);
            let register = self.data_bus.read_panic(self.registers.pc + 1);
            let data_1 = self.data_bus.read_panic(self.registers.pc + 2);
            let data_2 = self.data_bus.read_panic(self.registers.pc + 3);

            let instruction = Instruction::new_u8(opcode, register, data_1, data_2);
            // println!("Instruction: {:#?}", instruction);
            self.decode(instruction);
        }
    }

    fn advance(&mut self) {
        self.registers.pc += 4;
    }

    fn decode(&mut self, instruction: Instruction) {
        if self.inst_map.contains_key(&instruction.opcode) {
            self.inst_map[&instruction.opcode](self, &instruction);
        } else {
            panic!("Unknown opcode: {}\n", instruction.opcode);
        }
    }

    #[allow(unused_variables, clippy::needless_return)]
    fn decode_register(&mut self, reg: u8) -> &mut u16 {
        return match reg {
            0 => &mut self.registers.r1,
            1 => &mut self.registers.r2,
            2 => &mut self.registers.r3,
            3 => &mut self.registers.r4,
            4 => &mut self.registers.r5,
            5 => &mut self.registers.r6,
            6 => &mut self.registers.pc,
            7 => &mut self.registers.sp,
            8 => &mut self.registers.bp,
            _ => panic!("Invalid register"),
        };
    }

    fn compare(&mut self, a: u16, b: u16) {
        self.registers.flags = 0x0;

        match a.cmp(&b) {
            std::cmp::Ordering::Equal => self.registers.flags |= 0b00000001,
            std::cmp::Ordering::Greater => self.registers.flags |= 0b00100000,
            std::cmp::Ordering::Less => self.registers.flags |= 0b01000000,
        }
    }

    pub fn dump_cpu(&mut self, dump_action: DumpAction) -> Option<CPUDump> {
        if dump_action == DumpAction::File {
            let mut string = format!("Program Counter: 0x{}\n", self.registers.pc.to_hex_string());

            string += &format!("Stack Pointer: 0x{}\n", self.registers.sp.to_hex_string());
            string += &format!("Base Pointer: 0x{}\n\n", self.registers.bp.to_hex_string());
            string += "Register:\n";

            for i in 0..6 {
                let reg = *self.decode_register(i as u8);
                string += &format!("    R{}: 0x{}\n", i + 1, reg.to_hex_string());
            }

            string += "\nFlags:\n";
            string += &format!("    Zero: {}\n", self.registers.flags & (1 << 0));
            string += &format!("    GT  : {}\n", self.registers.flags & (1 << 5));
            string += &format!("    LT  : {}\n", self.registers.flags & (1 << 6));
            string += &format!("    OvrF: {}\n", self.registers.flags & (1 << 1));

            string += "\nStack: [";

            for i in 0..self.registers.sp {
                let val = self.data_bus.read_panic(i);

                string += &format!("0x{}", val.to_hex_string());

                if i != self.registers.sp - 1 {
                    string += ", ";
                }
            }

            string += "]\n";

            self.data_bus.dump();

            string += "\nMemory dump: 'memory.bin'";

            match fs::write("cpu_dump.txt", string) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Unable to dump CPU.\n{error}");
                    exit(1);
                }
            };

            None
        } else {
            let mut stack: Vec<u8> = Vec::new();

            for i in 0..self.registers.sp {
                let val = self.data_bus.read_panic(i);

                stack.push(val);
            }

            Some(CPUDump::new(&self.registers, stack, &self.data_bus))
        }
    }

    fn push(&mut self, value: &u16) {
        self.data_bus
            .write_panic(self.registers.sp, &((value >> 8) as u8));
        self.data_bus
            .write_panic(self.registers.sp + 1, &((value & 0xFF) as u8));
        self.registers.sp += 2;
    }

    fn pop(&mut self) -> u16 {
        self.registers.sp -= 2;

        let d1 = self.data_bus.read_panic(self.registers.sp);
        let d2 = self.data_bus.read_panic(self.registers.sp + 1);

        self.data_bus.write_panic(self.registers.sp, &0x00);
        self.data_bus.write_panic(self.registers.sp + 1, &0x00);

        ((d1 as u16) << 8) | d2 as u16
    }

    fn fill_hashmap(&mut self) {
        self.inst_map.insert(0x00, VCPU::inst_mov_value);
        self.inst_map.insert(0x01, VCPU::inst_ld_register);
        self.inst_map.insert(0x02, VCPU::inst_ld_address);
        self.inst_map.insert(0x03, VCPU::inst_psh_value);
        self.inst_map.insert(0x04, VCPU::inst_psh_register);
        self.inst_map.insert(0x05, VCPU::inst_psh_address);
        self.inst_map.insert(0x06, VCPU::inst_pop_register);
        self.inst_map.insert(0x07, VCPU::inst_ins_pop);
        self.inst_map.insert(0x08, VCPU::inst_ld_short);
        self.inst_map.insert(0x09, VCPU::inst_mov_register);
        self.inst_map.insert(0x0A, VCPU::inst_mov_value); // mov_addr but we don't really do anything with the bus

        self.inst_map.insert(0x10, VCPU::inst_st_address);
        self.inst_map.insert(0x11, VCPU::inst_stl_address);
        self.inst_map.insert(0x12, VCPU::inst_sth_address);

        self.inst_map.insert(0x13, VCPU::inst_st_register);
        self.inst_map.insert(0x14, VCPU::inst_stl_register);
        self.inst_map.insert(0x15, VCPU::inst_sth_register);

        self.inst_map.insert(0x20, VCPU::inst_cmp_register);
        self.inst_map.insert(0x21, VCPU::inst_cmp_value);

        self.inst_map.insert(0x30, VCPU::inst_beq);
        self.inst_map.insert(0x31, VCPU::inst_bgt);
        self.inst_map.insert(0x32, VCPU::inst_blt);
        self.inst_map.insert(0x33, VCPU::inst_jmp);
        self.inst_map.insert(0x34, VCPU::inst_bof);
        self.inst_map.insert(0x35, VCPU::inst_bne);

        self.inst_map.insert(0x40, VCPU::inst_add_value);
        self.inst_map.insert(0x41, VCPU::inst_sub_value);
        self.inst_map.insert(0x42, VCPU::inst_add_register);
        self.inst_map.insert(0x43, VCPU::inst_sub_register);

        self.inst_map.insert(0x50, VCPU::inst_call);
        self.inst_map.insert(0x51, VCPU::inst_ret);

        self.inst_map.insert(0xFE, VCPU::inst_hlt);
        self.inst_map.insert(0xFF, VCPU::inst_nop);
    }

    // !!! Instructions

    fn inst_mov_value(&mut self, instruction: &Instruction) {
        *self.decode_register(instruction.register) = instruction.data;
        self.advance();
    }

    fn inst_ld_register(&mut self, instruction: &Instruction) {
        let addr = *self.decode_register(instruction.data as u8);

        *self.decode_register(instruction.register) = self.data_bus.read_short_panic(addr);
        self.advance();
    }

    fn inst_ld_address(&mut self, instruction: &Instruction) {
        *self.decode_register(instruction.register) =
            self.data_bus.read_panic(instruction.data) as u16;
        self.advance();
    }

    fn inst_psh_value(&mut self, instruction: &Instruction) {
        self.push(&instruction.data);
        self.advance();
    }

    fn inst_psh_register(&mut self, instruction: &Instruction) {
        let val = *self.decode_register(instruction.register);
        self.push(&val);
        self.advance();
    }

    fn inst_psh_address(&mut self, instruction: &Instruction) {
        self.push(&(self.data_bus.read_panic(instruction.data) as u16));
        self.advance();
    }

    fn inst_pop_register(&mut self, instruction: &Instruction) {
        *self.decode_register(instruction.register) = self.pop();
        self.advance();
    }

    fn inst_ins_pop(&mut self, _instruction: &Instruction) {
        self.pop();
        self.advance();
    }

    fn inst_ld_short(&mut self, instruction: &Instruction) {
        *self.decode_register(instruction.register) =
            ((self.data_bus.read_panic(instruction.data) as u16) << 8)
                | self.data_bus.read_panic(instruction.data + 1) as u16;
        self.advance();
    }

    fn inst_mov_register(&mut self, instruction: &Instruction) {
        *self.decode_register(instruction.register) = *self.decode_register(instruction.data as u8);
        self.advance();
    }

    fn inst_st_address(&mut self, instruction: &Instruction) {
        let r = self.decode_register(instruction.register);
        let v1 = (*r >> 8_u16) as u8;
        let v2 = (*r & 0xFF_u16) as u8;

        // CPU is big endian, so store the high byte first
        self.data_bus.write_panic(instruction.data, &v1);
        self.data_bus.write_panic(instruction.data + 1, &v2);
        self.advance();
    }

    fn inst_stl_address(&mut self, instruction: &Instruction) {
        let value = *self.decode_register(instruction.register) as u8;
        self.data_bus.write_panic(instruction.data, &value);
        self.advance();
    }

    fn inst_sth_address(&mut self, instruction: &Instruction) {
        let value = (*self.decode_register(instruction.register) >> 8) as u8;
        self.data_bus.write_panic(instruction.data, &value);
        self.advance();
    }

    fn inst_st_register(&mut self, instruction: &Instruction) {
        let r = self.decode_register(instruction.register);
        let v1 = (*r >> 8_u16) as u8;
        let v2 = (*r & 0xFF_u16) as u8;

        let r2 = *self.decode_register(instruction.data as u8);

        // CPU is big endian, so store the high byte first
        self.data_bus.write_panic(r2, &v1);
        self.data_bus.write_panic(r2 + 1, &v2);
        self.advance();
    }

    fn inst_stl_register(&mut self, instruction: &Instruction) {
        let value = *self.decode_register(instruction.register) as u8;
        let reg = *self.decode_register(instruction.data as u8);

        self.data_bus.write_panic(reg, &value);
        self.advance();
    }

    fn inst_sth_register(&mut self, instruction: &Instruction) {
        let value = (*self.decode_register(instruction.register) >> 8) as u8;
        let reg = *self.decode_register(instruction.data as u8);

        self.data_bus.write_panic(reg, &value);
        self.advance();
    }

    fn inst_cmp_register(&mut self, instruction: &Instruction) {
        let a = *self.decode_register(instruction.register);
        let b = *self.decode_register(instruction.data as u8);
        self.compare(a, b);
        self.advance();
    }

    fn inst_cmp_value(&mut self, instruction: &Instruction) {
        let a = *self.decode_register(instruction.register);
        self.compare(a, instruction.data);

        self.advance();
    }

    fn inst_beq(&mut self, instruction: &Instruction) {
        // value & (1 << pos)
        if self.registers.flags & (1 << 0) > 0 {
            self.registers.pc = instruction.data;
        } else {
            self.advance();
        }
    }

    fn inst_bgt(&mut self, instruction: &Instruction) {
        if self.registers.flags & (1 << 5) > 0 {
            self.registers.pc = instruction.data;
        } else {
            self.advance();
        }
    }

    fn inst_blt(&mut self, instruction: &Instruction) {
        if self.registers.flags & (1 << 6) > 0 {
            self.registers.pc = instruction.data;
        } else {
            self.advance();
        }
    }

    fn inst_jmp(&mut self, instruction: &Instruction) {
        self.registers.pc = instruction.data;
    }

    fn inst_bof(&mut self, instruction: &Instruction) {
        if self.registers.flags & (1 << 1) > 0 {
            self.registers.pc = instruction.data;
        } else {
            self.advance();
        }
    }

    fn inst_bne(&mut self, instruction: &Instruction) {
        if self.registers.flags & (1 << 0) == 0 {
            self.registers.pc = instruction.data;
        } else {
            self.advance();
        }
    }

    fn inst_add_value(&mut self, instruction: &Instruction) {
        let val = (*self.decode_register(instruction.register)) + instruction.data;
        *self.decode_register(instruction.register) = val;
        self.advance();
    }

    fn inst_sub_value(&mut self, instruction: &Instruction) {
        let val = *self.decode_register(instruction.register) - instruction.data;
        *self.decode_register(instruction.register) = val;
        self.advance();
    }

    fn inst_add_register(&mut self, instruction: &Instruction) {
        let val = *self.decode_register(instruction.register)
            + *self.decode_register(instruction.data as u8);
        *self.decode_register(instruction.register) = val;
        self.advance();
    }

    fn inst_sub_register(&mut self, instruction: &Instruction) {
        let val = *self.decode_register(instruction.register)
            - *self.decode_register(instruction.data as u8);
        *self.decode_register(instruction.register) = val;
        self.advance();
    }

    fn inst_call(&mut self, instruction: &Instruction) {
        self.push(&(self.registers.pc + 4));
        self.inst_jmp(instruction);
    }

    fn inst_ret(&mut self, _instruction: &Instruction) {
        let addr = self.pop();
        self.registers.pc = addr;
    }

    fn inst_hlt(&mut self, _instruction: &Instruction) {
        self.registers.pc = self.end_of_program + 1;
    }

    fn inst_nop(&mut self, _instruction: &Instruction) {
        self.advance();
    }
}
