# LDS (LoaD Short)

Loads a short into a register from memory.

| Instruction    | Example         | Function                             | Opcode | Bytecode                      |
| -------------- | --------------- | ------------------------------------ | ------ | ----------------------------- |
| LDS dest, reg  | LDS R1, R6      | Loads reg's short in mem into dest.  | 0x0B   | 0x0B DEST 0x00 REG            |
| LDS dest, addr | LDS R2, $0x1234 | Loads addr's short in mem into dest. | 0x08   | 0x08 DEST ADDR_HI ADDR_LO     |
