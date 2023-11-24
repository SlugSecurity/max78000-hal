MEMORY
{
    FLASH      (rx)  :  ORIGIN = 0x10000000,                             LENGTH = 512K
    STACK      (rw)  :  ORIGIN = 0x20000000,                             LENGTH = 110K
    RAM        (rw)  :  ORIGIN = ORIGIN(STACK) + LENGTH(STACK),          LENGTH = 128K - LENGTH(STACK)
}

/*
Add a block of memory for the stack before the RAM block, so that a stack overflow leaks into
reserved space and flash memory, instead of .data and .bss.
*/

_stack_start = ORIGIN(STACK) + LENGTH(STACK);
