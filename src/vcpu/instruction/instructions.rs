use super::opcode::AddressingMode;
use super::opcode::Opcode;
use crate::vcpu::cpu::CPU;

pub type InstructionFunction = fn(&mut CPU);
pub type InstructionInfo = (Opcode, AddressingMode, InstructionFunction);

pub fn mov_immediate(_cpu: &mut CPU) {
    todo!();
}
pub fn mov_register(_cpu: &mut CPU) {
    todo!();
}
pub fn ld_register(_cpu: &mut CPU) {
    todo!();
}
pub fn ld_address(_cpu: &mut CPU) {
    todo!();
}
pub fn ldb_register(_cpu: &mut CPU) {
    todo!();
}
pub fn ldb_address(_cpu: &mut CPU) {
    todo!();
}
pub fn psh_immediate(_cpu: &mut CPU) {
    todo!();
}
pub fn psh_register(_cpu: &mut CPU) {
    todo!();
}
pub fn psh_address(_cpu: &mut CPU) {
    todo!();
}
pub fn pop(_cpu: &mut CPU) {
    todo!();
}
pub fn pop_register(_cpu: &mut CPU) {
    todo!();
}

pub fn st_register(_cpu: &mut CPU) {
    todo!();
}
pub fn st_address(_cpu: &mut CPU) {
    todo!();
}
pub fn stl_register(_cpu: &mut CPU) {
    todo!();
}
pub fn stl_address(_cpu: &mut CPU) {
    todo!();
}
pub fn sth_register(_cpu: &mut CPU) {
    todo!();
}
pub fn sth_address(_cpu: &mut CPU) {
    todo!();
}

pub fn cmp_immediate(_cpu: &mut CPU) {
    todo!();
}
pub fn cmp_register(_cpu: &mut CPU) {
    todo!();
}

pub fn beq(_cpu: &mut CPU) {
    todo!();
}
pub fn bgt(_cpu: &mut CPU) {
    todo!();
}
pub fn blt(_cpu: &mut CPU) {
    todo!();
}
pub fn bof(_cpu: &mut CPU) {
    todo!();
}
pub fn bne(_cpu: &mut CPU) {
    todo!();
}

pub fn jmp(_cpu: &mut CPU) {
    todo!();
}
pub fn jsr(_cpu: &mut CPU) {
    todo!();
}

pub fn add_direct(_cpu: &mut CPU) {
    todo!();
}
pub fn add_register(_cpu: &mut CPU) {
    todo!();
}

pub fn sub_direct(_cpu: &mut CPU) {
    todo!();
}
pub fn sub_register(_cpu: &mut CPU) {
    todo!();
}

pub fn ret(_cpu: &mut CPU) {
    todo!();
}

pub fn hlt(_cpu: &mut CPU) {
    todo!();
}
pub fn nop(_cpu: &mut CPU) {
    todo!();
}
