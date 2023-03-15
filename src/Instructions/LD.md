# LD (LoaD)

Loads a byte into a register from memory.

| Instruction   | Example        | Function                            | Opcode | Bytecode                      |
| ------------- | -------------- | ----------------------------------- | ------ | ----------------------------- |
| LD dest, reg  | LD R1, R6      | Loads reg's byte in mem into dest.  | 0x01   | 0x01 DEST 0x00 REG            |
| LD dest, addr | LD R2, $0x1234 | Loads addr's byte in mem into dest. | 0x02   | 0x02 DEST ADDR_HI ADDR_LO     |
