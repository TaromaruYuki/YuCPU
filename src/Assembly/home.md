# Assembly Language

The Assembly Language made for YuCPU is meant to be easier to use and understand compared to others.

Registers are easy to identify, starting with the letter `R` and the register number. Accessing the `SP` and `PC` registers still use the `R` syntax. So `SP` would be `RSP` and `PC` would be `RPC`.

Labels start with the `@` symbol, the name, and a `:` to end it off.

Indention isn't necessary, but if you want to use it spaces cannot be used.

Sections and metadata is defined with a `.` symbol. Two sections exist, `.data` and `.text`. `.data` for data already defined and `.text` for code. There's only one metadata and thats defining a start label, `.main`.

Instructions are used in the way they are defined in [Opcodes](../The_CPU/opcodes.md), for example `LD`. Instructions are case sensitive. Most instructions have a register as a parameter, which is usually the value's output. Special cases can occur, like the `POP` instruction. Then it takes in a data type, the type also defined in [Opcodes](../The_CPU/opcodes.md).

Instruction examples:

`LD R1, 5`

`ADD R3, R2`

Special case instruction examples:

`POP`

`POP R1`

`HLT`

A example program can be found in [Bytecode](../The_CPU/bytecode.md).