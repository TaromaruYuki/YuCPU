# Flags

| D | L | G | N/A | N/A | N/A | O | Z |
| :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |

- D: Is the data a D-Word? (Deprecated after release 0.2.0)
- L: Less than (CMP)
- G: Greater than (CMP)
- O: Overflow (Arithmetic operations)
- Z: Zero (Eq on CMP)

<!-- These flags get set with the `CMP` instruction, excluding the overflow flag. Overflow gets set when a register overflows due to a overflow using the `ADD` or `SUB` instructions. -->