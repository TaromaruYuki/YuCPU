# PSH (PuSH)

Pushes a value onto the stack and increments the stack pointer + 2.

| Instruction | Example     | Function                     | Opcode | Bytecode                     |
| ----------- | ----------- | ---------------------------- | ------ | ---------------------------- |
| PSH val     | PSH 0x1234  | Pushes val onto stack.       | 0x03  | 0x00 0x00 VAL_HI VAL_LO       |
| PSH reg     | PSH R6      | Pushes reg's val onto stack. | 0x04  | 0x0B REG 0x00 0x00            |
| PSH addr    | PSH $0x1234 | Reads addr and pushes value. | 0x05  | 0x08 0x00 ADDR_HI ADDR_LO     |
