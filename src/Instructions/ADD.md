# ADD (Add)

This instruction adds the destination and source numbers together. The result is stored inside the destination. Operands are assumed signed. A overflow flag is set if the result is bigger than 0xFFFF.

| Instruction        | Example    | Function                      | Opcode | Bytecode                |
| ------------------ | ---------- | ----------------------------- | ------ | ----------------------- |
| ADD dest, operand  | ADD R4, 10 | Adds dest & operand's value   | 0x40   | 0x40 DEST VAL_HI VAL_LO |
| ADD dest, register | ADD R1, R3 | Adds dest & register's value. | 0x42   | 0x42 DEST 0x00 REG      |