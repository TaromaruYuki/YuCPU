# POP

Pops a value from the stack. Optionally returns the value to a register.

| Instruction | Example | Function                      | Opcode | Bytecode            |
| ----------- | ------- | ----------------------------- | ------ | ------------------- |
| POP reg     | ADD R1  | Pops a value and stores it.   | 0x06   | 0x06 REG 0x00 0x00  |
| POP         | POP     | Pops a value and discards it. | 0x07   | 0x40 0x00 0x00 0x00 |
