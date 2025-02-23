MEMORY
{
    FLASH      (rx)  :  ORIGIN = 0x1000E000,                             LENGTH = 222K 
    SECFLASH   (rx)  :  ORIGIN = ORIGIN(FLASH) + LENGTH(FLASH),          LENGTH = 2K 
    STACK      (rw)  :  ORIGIN = 0x20000000,                             LENGTH = 110K
    RAM        (rw)  :  ORIGIN = ORIGIN(STACK) + LENGTH(STACK),          LENGTH = 128K - LENGTH(STACK)
}

/*
Add a block of memory for the stack before the RAM block, so that a stack overflow leaks into
reserved space and flash memory, instead of .data and .bss.
*/
ASSERT((LENGTH(SECFLASH) == 2K), "Error: SECFLASH is not 2K. To change the size, update this assert, the size in the MEMORY section, and the assert in the flash layout crate.") 

_stack_start = ORIGIN(STACK) + LENGTH(STACK);
_stack_end = ORIGIN(STACK);

/* Bootloader hard jumps to 0x1000e200 */
_stext = ORIGIN(FLASH) + 0x200;

SECTIONS {
    /* Add a section for the secure flash space. */
    .secflash ORIGIN(SECFLASH) :
    {
        KEEP(*(.secflash .secflash.*));
        . = ALIGN(4);
    } > SECFLASH
}
