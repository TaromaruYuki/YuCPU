# Memory

The CPU uses 64kb of memory. The memory is layed out in the format shown.

| Stack | General Purpose Memory | Program Memory |
| :-: | :-: | :-: |
| 8k | 56k | 9.5k |
| 0x00 - 0x1F40 | 0x1F41 - 0xDAC0 | 0xDAC1 - 0xFFDD |

This layout can possibly change in the future due to new features and learning more about design.
