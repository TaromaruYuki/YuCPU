# Memory

The CPU uses 64kb of memory. The memory map is in the format shown.

| Interrupt Vector Table | RAM | ROM | Stack |
| :-: | :-: | :-: | :-: |
| 1kb | 16kb | 1kb | 1kb |
| 0x0000 - 0x0400 | 0x0401 - 0x4401 | 0x4402 - 0x4802 | 0x4803 - 0x4C03 |

This layout can possibly change in the future due to new features and learning more about design.
