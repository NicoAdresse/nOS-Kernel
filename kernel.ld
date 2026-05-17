/* Linker Script */

ENTRY(_start)

SECTIONS {
    . = 0x10000;

    .text : {
        /* Force the entry point function to be at the very beginning */
        *(.text._start)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}