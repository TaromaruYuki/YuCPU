use super::bus::DataBus;
use super::VCPU;
// use super::bus::DataBusError;

#[test]
fn test_bus_read() {
    let mut bus = DataBus::new();

    bus.memory[0x1234] = 0xAB;

    assert_eq!(bus.read_panic(0x1234), 0xAB);
}

#[test]
fn test_bus_write() {
    let mut bus = DataBus::new();

    bus.write_panic(0xABCD, &0xD0);

    assert_eq!(bus.memory[0xABCD], 0xD0);
}

#[test]
fn test_bus_reset() {
    let mut bus = DataBus::new();

    bus.memory[0xABCD] = 100;
    bus.memory[0x00] = 0xAA;

    bus.reset();

    assert_eq!(bus.memory, vec![0; 0xFFDD]);
}

#[test]
fn test_bus_mem_copy() {
    let mut bus = DataBus::new();
    let copy = vec![0x0A_u8, 0x0B_u8, 0x0C_u8, 0x0D_u8];

    bus.mem_copy(0xABCD, &copy);

    assert_eq!(bus.memory[0xABCD..0xABCD + 4], copy);
}

#[test]
fn cpu_mov_value() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x00, 0x00, 0x00, 0x05]);
    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x05);
}

#[test]
fn cpu_mov_register() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x09, 0x00, 0x00, 0x01]);

    cpu.registers.r2 = 0x10;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x10);
}

#[test]
fn cpu_ld_register() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x01, 0x00, 0x00, 0x01]);

    cpu.data_bus.memory[0x10] = 0x00;
    cpu.data_bus.memory[0x11] = 0x50;
    cpu.registers.r2 = 0x10;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x50);
}

#[test]
fn cpu_ld_address() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x02, 0x00, 0x0D, 0x07]);

    cpu.data_bus.memory[0x0D07] = 0xBA;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0xBA);
}

#[test]
fn cpu_psh_value() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x03, 0x00, 0xAB, 0xCD]);
    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.stack[0], 0xAB);
    assert_eq!(dump.stack[1], 0xCD);
}

#[test]
fn cpu_psh_register() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x04, 0x00, 0x00, 0x00]);

    cpu.registers.r1 = 0x0D07;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.stack[0], 0x0D);
    assert_eq!(dump.stack[1], 0x07);
}

#[test]
fn cpu_psh_address() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x05, 0x00, 0xAB, 0xCD]);

    cpu.data_bus.memory[0xABCD] = 0x0D;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.stack[0], 0x00);
    assert_eq!(dump.stack[1], 0x0D);
}

#[test]
fn cpu_pop_register() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x06, 0x00, 0x00, 0x00]);

    cpu.data_bus.memory[0x0000] = 0x0D;
    cpu.data_bus.memory[0x0001] = 0x07;
    cpu.registers.sp = 0x02;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x0D07);
}

#[test]
fn cpu_pop() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x07, 0x00, 0x00, 0x00]);

    cpu.data_bus.memory[0x0000] = 0x0D;
    cpu.data_bus.memory[0x0001] = 0x07;
    cpu.registers.sp = 0x02;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x00);
    assert_eq!(dump.stack.len(), 0x00);
    assert_eq!(dump.registers.sp, 0x00);
    assert_eq!(dump.memory.memory[0x0000], 0x00);
    assert_eq!(dump.memory.memory[0x0001], 0x00);
}

#[test]
fn cpu_lds() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x08, 0x01, 0xBE, 0xEF]);

    cpu.data_bus.memory[0xBEEF] = 0x0D;
    cpu.data_bus.memory[0xBEF0] = 0x07;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r2, 0x0D07);
}

#[test]
fn cpu_st_address() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x10, 0x00, 0xAB, 0xCD]);

    cpu.registers.r1 = 0x0D07;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.memory.memory[0xABCD], 0x0D);
    assert_eq!(dump.memory.memory[0xABCE], 0x07);
}

#[test]
fn cpu_stl_address() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x11, 0x00, 0xAB, 0xCD]);

    cpu.registers.r1 = 0x0D07;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.memory.memory[0xABCD], 0x07);
    assert_eq!(dump.memory.memory[0xABCE], 0x00);
}

#[test]
fn cpu_sth_address() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x12, 0x00, 0xAB, 0xCD]);

    cpu.registers.r1 = 0x0D07;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.memory.memory[0xABCD], 0x0D);
    assert_eq!(dump.memory.memory[0xABCE], 0x00);
}

#[test]
fn cpu_cmp_register_is_eq() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x20, 0x00, 0x00, 0x01]);

    cpu.registers.r1 = 0x05;
    cpu.registers.r2 = 0x05;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert!(dump.registers.flags & (1 << 0) > 0); // EQ flag
    assert_eq!(dump.registers.flags & (1 << 5), 0); // GT flag
    assert_eq!(dump.registers.flags & (1 << 6), 0); // LT flag
}

#[test]
fn cpu_cmp_register_is_gt() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x20, 0x00, 0x00, 0x01]);

    cpu.registers.r1 = 0x0A;
    cpu.registers.r2 = 0x05;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.flags & (1 << 0), 0); // EQ flag
    assert!(dump.registers.flags & (1 << 5) > 0); // GT flag
    assert_eq!(dump.registers.flags & (1 << 6), 0); // LT flag
}

#[test]
fn cpu_cmp_register_is_lt() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x20, 0x00, 0x00, 0x01]);

    cpu.registers.r1 = 0x05;
    cpu.registers.r2 = 0x0A;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.flags & (1 << 0), 0); // EQ flag
    assert_eq!(dump.registers.flags & (1 << 5), 0); // GT flag
    assert!(dump.registers.flags & (1 << 6) > 0); // LT flag
}

#[test]
fn cpu_cmp_value_is_eq() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x21, 0x00, 0x00, 0x05]);

    cpu.registers.r1 = 0x05;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert!(dump.registers.flags & (1 << 0) > 0); // EQ flag
    assert_eq!(dump.registers.flags & (1 << 5), 0); // GT flag
    assert_eq!(dump.registers.flags & (1 << 6), 0); // LT flag
}

#[test]
fn cpu_cmp_value_is_gt() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x21, 0x00, 0x00, 0x05]);

    cpu.registers.r1 = 0x0A;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.flags & (1 << 0), 0); // EQ flag
    assert!(dump.registers.flags & (1 << 5) > 0); // GT flag
    assert_eq!(dump.registers.flags & (1 << 6), 0); // LT flag
}

#[test]
fn cpu_cmp_value_is_lt() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x21, 0x00, 0x00, 0x0A]);

    cpu.registers.r1 = 0x05;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.flags & (1 << 0), 0); // EQ flag
    assert_eq!(dump.registers.flags & (1 << 5), 0); // GT flag
    assert!(dump.registers.flags & (1 << 6) > 0); // LT flag
}

#[test]
fn cpu_beq_branches() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x30, 0x00, 0xDA, 0xC5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0000_0001;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACE);
}

#[test]
fn cpu_beq_no_branch() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x30, 0x00, 0xDA, 0xC5, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0010_0000;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACA);
}

#[test]
fn cpu_bgt_branches() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x31, 0x00, 0xDA, 0xC5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0010_0000;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACE);
}

#[test]
fn cpu_bgt_no_branch() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x31, 0x00, 0xDA, 0xC5, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b1000_0000;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACA);
}

#[test]
fn cpu_blt_branches() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x31, 0x00, 0xDA, 0xC5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0100_0000;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACE);
}

#[test]
fn cpu_blt_no_branch() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x31, 0x00, 0xDA, 0xC5, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0000_1000;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACA);
}

#[test]
fn cpu_bne_branches() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x35, 0x00, 0xDA, 0xC5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0000_0000;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACE);
}

#[test]
fn cpu_bne_no_branch() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x35, 0x00, 0xDA, 0xC9, 0xFE, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);

    cpu.registers.flags = 0b0000_0001;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACE);
}

#[test]
fn cpu_jmp() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![
        0xDA, 0xC1, 0x31, 0x00, 0xDA, 0xC5, 0xFE, 0xFE, 0xFF, 0xFF,
    ]);

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.pc, 0xDACA);
}

#[test]
fn cpu_add_value() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x40, 0x00, 0x00, 0x05]);

    cpu.registers.r1 = 0x06;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x0B);
}

#[test]
fn cpu_sub_value() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x41, 0x00, 0x00, 0x05]);

    cpu.registers.r1 = 0x06;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x01);
}

#[test]
fn cpu_add_register() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x42, 0x00, 0x00, 0x01]);

    cpu.registers.r1 = 0x06;
    cpu.registers.r2 = 0x05;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x0B);
}

#[test]
fn cpu_sub_register() {
    let mut cpu = VCPU::new();

    cpu.load_program(vec![0xDA, 0xC1, 0x43, 0x00, 0x00, 0x01]);

    cpu.registers.r1 = 0x06;
    cpu.registers.r2 = 0x05;

    cpu.run();

    let dump = cpu.dump_cpu(super::DumpAction::Struct).unwrap();

    assert_eq!(dump.registers.r1, 0x01);
}
