|     | 0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 | 0xA | 0xB | 0xC | 0xD | 0xE | 0xF |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x0 | 0x00<br/>MOV V |   |   | 0x03<br/>PSH V |   |   |   |   | 0x08<br/>CMP V |   |   |   |   |   |   |   |
| 0x1 | 0x10<br/>ADD V | 0x11<br/>SUB V |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0x2 |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0x3 |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0x4 | 0x40<br/>MOV R | 0x41<br/>LD R | 0x42<br/>LDB R | 0x43<br/>PSH R | 0x44<br/>POP R | 0x45<br/>ST R | 0x46<br/>STL R | 0x47<br/>STH R | 0x48<br/>CMP R |   |   |   |   |   |   |   |
| 0x5 | 0x50<br/>ADD R | 0x51<br/>SUB R |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0x6 |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0x7 |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0x8 |   | 0x81<br/>LD A/L | 0x82<br/>LDB A/L | 0x83<br/>PSH A/L |   | 0x85<br/>ST A/L | 0x86<br/>STL A/L | 0x87<br/>STH A/L |   | 0x89<br/>BEQ A/L | 0x8A<br/>BGT A/L | 0x8B<br/>BLT A/L | 0x8C<br/>BOF A/L | 0x8D<br/>BNE A/L | 0x8E<br/>JMP A/L | 0x8F<br/>JSR A/L |
| 0x9 |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0xA |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0xB |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0xC |   |   |   |   | 0xC4<br/>POP  |   |   |   |   |   |   |   |   |   |   |   |
| 0xD |   |   |   | 0xD3<br/>RET  |   |   |   |   |   |   |   |   |   |   |   |   |
| 0xE |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
| 0xF |   |   |   |   |   |   |   |   |   |   |   |   |   |   | 0xFE<br/>HLT  | 0xFF<br/>NOP  |
