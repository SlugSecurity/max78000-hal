MEMORY
{
    FLASH       (rx)  :  ORIGIN = 0x10000000,                                   LENGTH = 512K
    STACK       (rw)  :  ORIGIN = 0x20000000,                                   LENGTH = 110K
    ANALOGSUCKS (rx)  :  ORIGIN = ORIGIN(STACK) + LENGTH(STACK),                LENGTH = 10K
    RAM         (rw)  :  ORIGIN = ORIGIN(ANALOGSUCKS) + LENGTH(ANALOGSUCKS),    LENGTH = 128K - LENGTH(STACK) - LENGTH(ANALOGSUCKS)
}

_stack_start = ORIGIN(STACK) + LENGTH(STACK);
_stack_end = ORIGIN(STACK);
