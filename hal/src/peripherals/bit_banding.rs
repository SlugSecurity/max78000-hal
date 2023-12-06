//! Code to handle bit-banding.
//!
//! Bit-banding is where the SoC maps each 8-bit byte to 8 consecutive 32-bit
//! words. Writing a 1 to that word sets the matching bit. Writing a 0 clears
//! the matching bit. It means you can perform atomic bit set/clear; i.e.
//! without a read-modify-write.

use core::ptr::{read_volatile, write_volatile};

/// Sets/clears a bit at the given address atomically, using the bit-banding
/// feature.
///
/// # Safety
///
/// This function is unsafe as it modifies any arbitrary memory address. Ensure the memory address
/// passed is within the bit-banding range: address must be `>= 0x2000_0000` and `<= 0x2001_FFFF`
/// if writing to SRAM, and `>= 0x4000_0000` and <= `0x400F_FFFF` if writing to peripheral address
/// space. If writing to peripherals, make sure you are writing to a writable bit in a valid
/// register. If writing to SRAM, make sure you are writing to a valid memory address.
pub unsafe fn change_bit<T>(address: *const T, bit: u8, value: bool) {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    // SAFETY: Call to write_volatile is safe assuming caller passes in an initialized address in
    // bit-banding region of memory.
    write_volatile(bit_word, if value { 0x01 } else { 0x00 });
}

/// Continually reads bit at the given address until it is equal to value passed in `state`.
/// Ensure that the bit you're reading *will eventually* become the value you expect,
/// otherwise the program will hang.
///
/// # Safety
///
/// This function is unsafe as it reads any arbitrary memory address. Ensure the memory address
/// passed is within the bit-banding range: address must be `>= 0x2000_0000` and `<= 0x2001_FFFF`
/// if reading SRAM, and `>= 0x4000_0000` and <= `0x400F_FFFF` if reading peripheral address
/// space. If reading from peripherals, make sure you are reading a readable bit in the correct
/// register. If reading from SRAM, make sure you are reading a valid memory address.
pub unsafe fn spin_bit<T>(address: *const T, bit: u8, state: bool) {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    // Call to read_volatile is safe assuming caller passes in an initialized address in
    // bit-banding region of memory; and the said bit in address will be changed to `state`
    // eventually.
    while (read_volatile(bit_word) != 0) != state {}
}

/// Reads a bit at the given address atomically, using the bit-banding feature.
///
/// # Safety
///
/// This function is unsafe as it reads any arbitrary memory address. Ensure the memory address
/// passed is within the bit-banding range: address must be `>= 0x2000_0000` and `<= 0x2001_FFFF`
/// if reading SRAM, and `>= 0x4000_0000` and <= `0x400F_FFFF` if reading peripheral address
/// space. If reading from peripherals, make sure you are reading a readable bit in the correct
/// register. If reading from SRAM, make sure you are reading a valid memory address.
pub unsafe fn read_bit<T>(address: *const T, bit: u8) -> bool {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    // Call to read_volatile is safe assuming caller passes in an initialized address in
    // bit-banding region of memory.
    read_volatile(bit_word) != 0
}

/// Address must be >= 0x2000_0000 and <= 0x2001_FFFF if writing to SRAM.
/// Address must be >= 0x4000_0000 and <= 0x400F_FFFF if writing to peripheral addr space.
/// Bit must be < 32.
fn ref_to_bitband(address: u32, bit: u8) -> *mut u32 {
    debug_assert!(
        (0x2000_0000..=0x2001_FFFF).contains(&address)
            || (0x4000_0000..=0x400F_FFFF).contains(&address)
    );
    debug_assert!(bit < 32);
    let prefix = address & 0xF000_0000;
    let byte_offset = address & 0x0FFF_FFFF;
    let bit_word_offset = (byte_offset * 32) + (u32::from(bit) * 4);
    let bit_word_addr = bit_word_offset + prefix + 0x0200_0000;
    bit_word_addr as *mut u32
}
