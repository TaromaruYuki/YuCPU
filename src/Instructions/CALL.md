# CALL

This instruction calls a function. This pushes the current program counter register + 4 as a return address to the stack and increments the stack pointer + 2.

| Instruction        | Example    | Function                           | Opcode | Bytecode                  |
| ------------------ | ---------- | ---------------------------------- | ------ | ------------------------- |
| CALL label         | CALL foo   | Calls and pushes the return addr   | 0x50   | 0x50 0x00 ADDR_HI ADDR_LO |
