# SUB (Subtract)

This instruction subtracts the destination and source numbers together. The result is stored inside the destination. Operands are assumed unsigned. No flags are set yet for going below 0x00, just a panic.

| Instruction        | Example    | Function                      | Opcode | Bytecode                |
| ------------------ | ---------- | ----------------------------- | ------ | ----------------------- |
| SUB dest, operand  | SUB R4, 10 | Subs dest & operand's value   | 0x41   | 0x41 DEST VAL_HI VAL_LO |
| SUB dest, register | SUB R1, R3 | Subs dest & register's value. | 0x43   | 0x43 DEST 0x00 REG      |