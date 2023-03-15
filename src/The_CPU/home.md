# The CPU

YuCPU is a **16 bit** [Big Endian](https://en.wikipedia.org/wiki/Endianness) CPU. YuCPU includes 6 general purpose registers,.

## Instructions

Instructions are 32 bits, which is split up into 4 parts.

| Opcode (8 bits) | Register (8 bits) | Data 1 (8 bits) | Data 2 (8 bits) |
| --- | --- | --- | --- |
| 0x00 - 0xFF | 0x00 - 0x0A | 0x00 - 0xFF | 0x00 - 0xFF |

Example:

| 0x40 | 0x01 | 0xBE | 0xEF |
| ---| --- | --- | --- |

YuCPU uses 64kb of memory. [Find out more about the memory layout.](./memory.md)

## CPU Startup

Register 1 through 6 is initialized at 0x00. Program counter starts at memory address 0xA000. Stack pointer starts at address 0xE000. Flag register is initialized at 0x00, clearing all flags.

Memory gets cleared, then the program loads. The CPU then finally starts running the program.

## Missing features

Currently, the CPU is pretty limited. Features such as, but not limiting to:

- [x] Jumping to and returning from subroutines (Saving return address on stack).
- [ ] Bitwise operations
- [ ] Getting address of label as value.
- [ ] Clearing flag(s)
- [ ] BSS section (Declaring variables that have no value)
- [ ] Comments in Assembly
- [ ] Overflows (?)