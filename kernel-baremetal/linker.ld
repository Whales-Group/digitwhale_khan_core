/*
 * Linker script for JevKen kernel (non-UEFI bare-metal entry via kernel_entry).
 * The UEFI path uses the standard aarch64-unknown-uefi PE/COFF layout instead.
 */

SECTIONS
{
    . = 0x40080000;

    .text : {
        *(.text._start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : ALIGN(16) {
        _bss_start = .;
        *(COMMON)
        *(.bss*)
        . = ALIGN(16);
        _bss_end = .;
    }

    . = ALIGN(16);
    _kernel_stack_start = .;
    . += 64K;
    _kernel_stack_end = .;
}
