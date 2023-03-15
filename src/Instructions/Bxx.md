# Bxx (Branch if xx)

This instruction branches if a specific flag is set. 

| Instruction        | Example    | Function                           | Opcode | Bytecode                  |
| ------------------ | ---------- | ---------------------------------- | ------ | ------------------------- |
| BEQ label          | BEQ foo    | Branches if the EQ flag is set     | 0x30   | 0x30 0x00 ADDR_HI ADDR_LO |
| BGT label          | BGT foo    | Branches if the GT flag is set     | 0x31   | 0x31 0x00 ADDR_HI ADDR_LO |
| BLT label          | BLT foo    | Branches if the LT flag is set     | 0x32   | 0x32 0x00 ADDR_HI ADDR_LO |
| BOF label          | BOF foo    | Branches if the OF flag is set     | 0x34   | 0x34 0x00 ADDR_HI ADDR_LO |
| BNE label          | BNE foo    | Branches if the EQ flag is NOT set | 0x35   | 0x35 0x00 ADDR_HI ADDR_LO |
