MEMORY
{
    FLASH      (rx)  :  ORIGIN = 0x10000000,                             LENGTH = 512K
    STACK      (rw)  :  ORIGIN = 0x20000000,                             LENGTH = 110K
    RAM        (rw)  :  ORIGIN = ORIGIN(STACK) + LENGTH(STACK),          LENGTH = 128K - LENGTH(STACK)
}

_stack_start = ORIGIN(STACK) + LENGTH(STACK);
_stack_end = ORIGIN(STACK);
