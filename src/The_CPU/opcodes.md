# Opcodes

The opcode table for the current design.

V = Value | R = Register | A = Address | L = Label (Address)

|     | 0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 | 0xA | 0xB | 0xC | 0xD | 0xE | 0xF |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x0 | 00<br/>LD V | 01<br/>LD R | 02<br/>LD A | 03<br/>PSH V | 04<br/>PSH R | 05<br/>PSH A | 06<br/>POP R | 07<br/>POP | 08[^1]<br/>LDS A |
| 0x1 | 10<br/>ST A | 11<br/>STL A | 12<br/>STH A |
| 0x2 | 20<br/>CMP R | 21<br/>CMP V |
| 0x3 | 30<br/>BEQ L | 31<br/>BGT L | 32<br/>BLT L | 33<br/>JMP L | 34<br/>BOF R |
| 0x4 | 40<br/>ADD V | 41<br/>SUB V | 42<br/>ADD R | 43<br/>SUB R |
| 0x5 |
| 0x6 |
| 0x7 |
| 0x8 |
| 0x9 |
| 0xA |
| 0xB |
| 0xC |
| 0xD |
| 0xE |
| 0xF | | | | | | | | | | | | | | |  FE<br/>HLT | FF<br/>NOP |