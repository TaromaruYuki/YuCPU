---
layout: default
title: YuCPU docs
---

## Info

Architecture: 16 bit

Register Size: 16 bits (2 bytes)

GP Register Count: 6

Instruction size: 32 bits (4 bytes)

Ram: 64kb

Endianness: Big Endian

## Memory Layout

| Stack | General Purpose Memory | Program Memory |
| :-: | :-: | :-: |
| 8k | 56k | 9.5k |
| 0x00 - 0x1F40 | 0x1F41 - 0xDAC0 | 0xDAC1 - 0xFFDD |

## Flag Registers

| R1 | R2 | R3 | R4 | R5 | R6 | PC | SP |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 |

## Startup and Reset

- Values
    - Register 1-6: 0x0
    - PC: 0xDAC1
    - SP: 0x00
    - Flag Register: 0x00
- Memory clear
- Load program
- Run program

## Opcode Table

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

[^1]: Load Short. Loads the address and address + 1 in big endian order.

## Bytecode Format

| Header | Data | Text |
| --- | --- | --- |
| Address to the start label (Text) | Data section (Strings) | Text section (Code) |

Example program:

```
.main start

.data

@str1:
	DAT "String"

.text

@start:
	LD R1, 0
	JMP count5

@count5:
	PSH R1
	ADD R1, 1

	CMP R1, 5
	BEQ end

	JMP count5

@end:
	HLT
```

Bytecode equivalent:
```
     00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
0000 DA C8 53 74 72 69 6E 67 00 00 01 00 00 33 00 DA
0010 D0 04 01 00 00 40 01 00 01 21 01 00 05 30 00 DA
0020 E4 33 00 DA D0 FE FE FF FF
```

Explanation:

- Address 0x00 - 0x01: start label. Subtract the start of program memory to get the address realitve to the bytecode.
    - Doing this will show that the start function starts at 0x09
- Address 0x02 - 0x08: Our string label. Encoded in ascii and automatically ends in 0x00. Translates to: "String\0"
- Address 0x09 - 0x28: Text section.
    - `start` label starts at 0x09 and ends at 0x10. Only two instructions.
    - `count5` label starts at 0x11 and ends at 0x24. Ony five instructions.
    - `end` label starts at 0x25 and ends at 0x28. Only one instruction.
    
## Missing features

- [ ] Jumping to and returning from subroutines (Saving return address on stack).
- [ ] Bitwise operations
- [ ] Getting address of label as value.
- [ ] Clearing flag(s)
- [ ] BSS section (Declaring variables that have no value)
- [ ] Comments in Assembly
- [ ] Overflows (?)
