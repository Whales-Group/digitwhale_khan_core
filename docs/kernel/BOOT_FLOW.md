# JevKen Boot Flow

Target flow:
1. Firmware / bootloader hands control to JevKen entry.
2. `_start` sets stack and CPU baseline.
3. `kernel_init()` runs:
   - memory init
   - IDT init
   - early serial/video init
   - scheduler init
4. Control transitions to kernel main loop.

This repo now tracks this directly in `kernel-baremetal/`.
