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
| 0x1 | 10<br/>ST A | 11<br/>STL A | 12<br/>STA R |
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
