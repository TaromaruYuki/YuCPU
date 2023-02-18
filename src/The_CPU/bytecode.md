# Bytecode

The format for the bytecode needed to run YuCPU programs is the following.

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