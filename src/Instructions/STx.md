# STx (STore xx)

Stores a short into memory. Also can store the hi byte and lo byte of a value. Register is the value.

| Instruction   | Example         | Function                            | Opcode | Bytecode                     |
| ------------- | --------------- | ----------------------------------- | ------ | ---------------------------- |
| ST val, addr  | ST R2, $0x1234  | Stores val into addr.               | 0x10   | 0x10 VAL ADDR_HI ADDR_LO     |
| ST val, reg   | ST R1, R6       | Stores val into reg's addr.         | 0x13   | 0x13 VAL 0x00 REG            |
| STL val, addr | STL R2, $0x1234 | Stores val lo byte into addr.       | 0x11   | 0x11 VAL ADDR_HI ADDR_LO     |
| STL val, reg  | STL R1, R6      | Stores val lo byte into reg's addr. | 0x14   | 0x14 VAL 0x00 REG            |
| STH val, addr | STH R2, $0x1234 | Stores val hi byte into addr.       | 0x12   | 0x12 VAL ADDR_HI ADDR_LO     |
| STH val, reg  | STH R1, R6      | Stores val hi byte into reg's addr. | 0x15   | 0x15 VAL 0x00 REG            |
