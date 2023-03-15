# MOV (MoVe)

"Moves" (Copies) a value into a register.

| Instruction     | Example         | Function                     | Opcode | Bytecode                     |
| --------------- | --------------- | ---------------------------- | ------ | ---------------------------- |
| MOV dest, val   | MOV R2, 0x1234  | Moves val into dest.         | 0x00  | 0x00 DEST VAL_HI VAL_LO       |
| MOV dest, reg   | MOV R1, R6      | Moves reg's val into dest.   | 0x09  | 0x0B DEST 0x00 REG            |
| MOV dest, addr  | MOV R2, $0x1234 | Moves addr into dest.        | 0x0A  | 0x08 DEST ADDR_HI ADDR_LO     |
| MOV dest, label | MOV R2, foo     | Moves labels addr into dest. | 0x0A  | 0x08 DEST ADDR_HI ADDR_LO     |
