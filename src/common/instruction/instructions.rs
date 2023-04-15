use super::opcode::AddressingMode;
use super::opcode::Opcode;
use crate::vcpu::cpu::Flags;
use crate::vcpu::cpu::CPU;
use crate::vcpu::device::map::DeviceMapResult;
use crate::vcpu::device::DeviceResponse;

pub type InstructionFunction = fn(&mut CPU);
pub type InstructionInfo = (Opcode, AddressingMode, InstructionFunction, u8);

pub fn mov_immediate(cpu: &mut CPU) {
    let register: u8 = ((0xF0 & cpu.ir) >> 4) as u8;
    *cpu.decode_register(register) = cpu.dr;
    cpu.advance();
}

pub fn mov_register(cpu: &mut CPU) {
    let register: u8 = ((0xF0 & cpu.ir) >> 4) as u8;
    *cpu.decode_register(register) = *cpu.decode_register(cpu.dr as u8);
    cpu.advance();
}

pub fn ld_register(cpu: &mut CPU) {
    let address = *cpu.decode_register(cpu.dr as u8);
    println!("Address: {}", address);

    let res = match cpu.map.read(address as u32) {
        DeviceMapResult::Ok(res) => res,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::WriteOnly {
                panic!("Device write only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = res;
    cpu.advance();
}

pub fn ld_address(cpu: &mut CPU) {
    let address: u32 = if cpu.flags.contains(Flags::D) {
        u32::from_be_bytes([0x00, cpu.ad, ((cpu.dr & 0xFF00) >> 8) as u8, cpu.dr as u8])
    } else {
        cpu.dr as u32
    };

    let res = match cpu.map.read(address) {
        DeviceMapResult::Ok(res) => res,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::WriteOnly {
                panic!("Device write only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = res;
    cpu.advance();
}

pub fn ldb_register(cpu: &mut CPU) {
    let address = *cpu.decode_register(cpu.dr as u8);

    let res = match cpu.map.read_byte(address as u32) {
        DeviceMapResult::Ok(res) => res,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::WriteOnly {
                panic!("Device write only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = res as u16;
    cpu.advance();
}

pub fn ldb_address(cpu: &mut CPU) {
    let address: u32 = if cpu.flags.contains(Flags::D) {
        u32::from_be_bytes([0x00, cpu.ad, ((cpu.dr & 0xFF00) >> 8) as u8, cpu.dr as u8])
    } else {
        cpu.dr as u32
    };

    let res = match cpu.map.read_byte(address) {
        DeviceMapResult::Ok(res) => res,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::WriteOnly {
                panic!("Device write only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = res as u16;
    cpu.advance();
}

pub fn psh_immediate(cpu: &mut CPU) {
    match cpu.map.write(cpu.sp as u32, cpu.dr) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.sp += 2;
    cpu.advance();
}

pub fn psh_register(cpu: &mut CPU) {
    let value = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    match cpu.map.write(cpu.sp as u32, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.sp += 2;
    cpu.advance();
}

pub fn psh_address(cpu: &mut CPU) {
    let address: u32 = if cpu.flags.contains(Flags::D) {
        u32::from_be_bytes([0x00, cpu.ad, ((cpu.dr & 0xFF00) >> 8) as u8, cpu.dr as u8])
    } else {
        cpu.dr as u32
    };

    let value = match cpu.map.read(address) {
        DeviceMapResult::Ok(res) => res,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::WriteOnly {
                panic!("Device write only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    match cpu.map.write(cpu.sp as u32, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.sp += 2;
    cpu.advance();
}

pub fn pop(cpu: &mut CPU) {
    cpu.sp -= 2;
    cpu.advance();
}

pub fn pop_register(cpu: &mut CPU) {
    cpu.sp -= 2;

    let value = match cpu.map.read(cpu.sp as u32) {
        DeviceMapResult::Ok(val) => val,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::WriteOnly {
                panic!("Device write only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = value;
    cpu.advance();
}

pub fn st_register(cpu: &mut CPU) {
    let value = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);

    let address = *cpu.decode_register(cpu.dr as u8) as u32;

    match cpu.map.write(address, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.advance();
}

pub fn st_address(cpu: &mut CPU) {
    let value = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);

    let address: u32 = if cpu.flags.contains(Flags::D) {
        u32::from_be_bytes([0x00, cpu.ad, ((cpu.dr & 0xFF00) >> 8) as u8, cpu.dr as u8])
    } else {
        cpu.dr as u32
    };

    match cpu.map.write(address, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.advance();
}

pub fn stl_register(cpu: &mut CPU) {
    let value = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) as u8;

    let address = *cpu.decode_register(cpu.dr as u8) as u32;

    match cpu.map.write_byte(address, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.advance();
}

pub fn stl_address(cpu: &mut CPU) {
    let value = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) as u8;

    let address: u32 = if cpu.flags.contains(Flags::D) {
        u32::from_be_bytes([0x00, cpu.ad, ((cpu.dr & 0xFF00) >> 8) as u8, cpu.dr as u8])
    } else {
        cpu.dr as u32
    };

    // println!("Storing val {} in address {}", value, address);

    match cpu.map.write_byte(address, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.advance();
}

pub fn sth_register(cpu: &mut CPU) {
    let value = ((*cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8)) >> 8) as u8;

    let address = *cpu.decode_register(cpu.dr as u8) as u32;

    match cpu.map.write_byte(address, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.advance();
}

pub fn sth_address(cpu: &mut CPU) {
    let value = ((*cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8)) >> 8) as u8;

    let address: u32 = if cpu.flags.contains(Flags::D) {
        u32::from_be_bytes([0x00, cpu.ad, ((cpu.dr & 0xFF00) >> 8) as u8, cpu.dr as u8])
    } else {
        cpu.dr as u32
    };

    match cpu.map.write_byte(address, value) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.advance();
}

fn compare(cpu: &mut CPU, val1: u16, val2: u16) {
    // Reset flags
    cpu.flags.set(Flags::Z, false);
    cpu.flags.set(Flags::L, false);
    cpu.flags.set(Flags::G, false);

    if val1 == val2 {
        cpu.flags.set(Flags::Z, true);
    }

    if val1 < val2 {
        cpu.flags.set(Flags::L, true);
    }

    if val1 > val2 {
        cpu.flags.set(Flags::G, true);
    }
}

pub fn cmp_immediate(cpu: &mut CPU) {
    let val1 = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    let val2 = cpu.dr;

    compare(cpu, val1, val2);

    cpu.advance();
}

pub fn cmp_register(cpu: &mut CPU) {
    let val1 = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    let val2 = *cpu.decode_register(cpu.dr as u8);

    compare(cpu, val1, val2);

    cpu.advance();
}

fn branch_flag_set(cpu: &mut CPU, flag: Flags) {
    if cpu.flags.contains(flag) {
        cpu.pc = if cpu.flags.contains(Flags::D) {
            (cpu.dr << 4) | ((cpu.ad as u16) & 0xF)
        } else {
            cpu.dr
        };
        return;
    }

    cpu.advance();
}

fn branch_flag_not_set(cpu: &mut CPU, flag: Flags) {
    if !cpu.flags.contains(flag) {
        cpu.pc = if cpu.flags.contains(Flags::D) {
            (cpu.dr << 4) | ((cpu.ad as u16) & 0xF)
        } else {
            cpu.dr
        };
        return;
    }

    cpu.advance();
}

pub fn beq(cpu: &mut CPU) {
    branch_flag_set(cpu, Flags::Z);
}

pub fn bgt(cpu: &mut CPU) {
    branch_flag_set(cpu, Flags::G);
}

pub fn blt(cpu: &mut CPU) {
    branch_flag_set(cpu, Flags::L);
}

pub fn bof(cpu: &mut CPU) {
    branch_flag_set(cpu, Flags::O);
}

pub fn bne(cpu: &mut CPU) {
    branch_flag_not_set(cpu, Flags::Z);
}

pub fn jmp(cpu: &mut CPU) {
    cpu.pc = if cpu.flags.contains(Flags::D) {
        (cpu.dr << 4) | ((cpu.ad as u16) & 0xF)
    } else {
        cpu.dr
    };
}

pub fn jsr(cpu: &mut CPU) {
    match cpu.map.write(cpu.sp as u32, cpu.pc + cpu.is as u16) {
        DeviceMapResult::Ok(_) => (),
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not write any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not write value.");
            } else {
                panic!("Unknown error. Could not write value.");
            }
        }
    };

    cpu.sp += 2;

    cpu.pc = if cpu.flags.contains(Flags::D) {
        (cpu.dr << 4) | ((cpu.ad as u16) & 0xF)
    } else {
        cpu.dr
    };
}

fn add(cpu: &mut CPU, val1: u16, val2: u16) -> u16 {
    let (result, overflow) = val1.overflowing_add(val2);

    if overflow {
        cpu.flags.set(Flags::O, true);
    } else {
        cpu.flags.set(Flags::O, false);
    }

    result
}

pub fn add_immediate(cpu: &mut CPU) {
    let val1 = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    let val2 = cpu.dr;

    let result = add(cpu, val1, val2);

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = result;
    cpu.advance();
}

pub fn add_register(cpu: &mut CPU) {
    let val1 = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    let val2 = *cpu.decode_register(cpu.dr as u8);

    let result = add(cpu, val1, val2);

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = result;
    cpu.advance();
}

fn sub(cpu: &mut CPU, val1: u16, val2: u16) -> u16 {
    let (result, overflow) = val1.overflowing_sub(val2);

    if overflow {
        cpu.flags.set(Flags::O, true);
    } else {
        cpu.flags.set(Flags::O, false);
    }

    result
}

pub fn sub_immediate(cpu: &mut CPU) {
    let val1 = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    let val2 = cpu.dr;

    let result = sub(cpu, val1, val2);

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = result;
    cpu.advance();
}

pub fn sub_register(cpu: &mut CPU) {
    let val1 = *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8);
    let val2 = *cpu.decode_register(cpu.dr as u8);

    let result = sub(cpu, val1, val2);

    *cpu.decode_register(((0xF0 & cpu.ir) >> 4) as u8) = result;
    cpu.advance();
}

pub fn ret(cpu: &mut CPU) {
    cpu.sp -= 2;
    let addr = match cpu.map.read(cpu.sp as u32) {
        DeviceMapResult::Ok(addr) => addr,
        DeviceMapResult::NoDevices => panic!("No devices attached. Could not read any values."),
        DeviceMapResult::Error(err) => {
            if err == DeviceResponse::ReadOnly {
                panic!("Device read only. Could not read value.");
            } else {
                panic!("Unknown error. Could not read value.");
            }
        }
    };

    cpu.pc = addr;
}

pub fn hlt(cpu: &mut CPU) {
    cpu.running = false;
}

pub fn nop(cpu: &mut CPU) {
    cpu.advance()
}
