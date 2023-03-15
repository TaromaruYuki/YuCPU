# CMP (CoMPare)

This instruction compares reg1 and a value / reg2. It sets the GT, LT, or EQ flag if the result is true.

| Instruction        | Example    | Function                           | Opcode | Bytecode                |
| ------------------ | ---------- | ---------------------------------- | ------ | ----------------------- |
| CMP reg1, reg2     | CMP R1, R3 | Compares reg1 & reg2's value.      | 0x20   | 0x20 REG1 0x00 REG2     |
| CMP reg1, operand  | CMP R4, 57 | Compares dest & operand's value.   | 0x21   | 0x20 REG1 VAL_HI VAL_LO |
