/*
 * This file is required by the rp-pico crate, instructions here: https://crates.io/crates/rp-pico
 * Description: "This file tells the linker the flash and RAM layout, so it won't clobber the bootloader or write to an out of bounds memory address."
 */

MEMORY{
    BOOT2 : ORIGIN = 0x10000000,
    LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100,
    LENGTH = 2048K - 0x100
             RAM : ORIGIN = 0x20000000,
    LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS{
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) :
        {
            KEEP(*(.boot2));
}
> BOOT2
}
INSERT BEFORE.text;