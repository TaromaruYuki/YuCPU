# Memory

The CPU uses 64kb of memory. The memory map is in the format shown.

| Interrupt Vector Table | RAM | ROM | Stack | BDA | VGA |
| :-: | :-: | :-: | :-: | :-: | :-: | 
| 1kb | 16kb | 1kb | 1kb | 255 bytes | 4kb |
| 0x0000 - 0x0400 | 0x0401 - 0x4401 | 0x4402 - 0x4802 | 0x4803 - 0x4C03 | 0x4C04 - 4D03 | 0xA000 - 0xAFA0 |

This layout can possibly change in the future due to new features and learning more about design.
