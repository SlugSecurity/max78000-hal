#![cfg(feature = "flc-ram")]
//! Flash controller peripheral API.
//!    # Examples
//!    ```
//!    let flash_controller = FlashController::new(flc, icc0, gcr);
//!
//!    let test_addr: u32 = 0x10070FF0;
//!    let test_val: u32 = 0xCAFEBABE;
//!    let mut data_read: [u8; 4] = [0; 4];
//!
//!    // # Safety
//!    // Non-read flash operations must not corrupt potentially instructions of the
//!    // program.
//!    // Callers must ensure that the following condition is met:
//!    // * If `address` points to a portion of the program's instructions, `data` must
//!    //   contain valid instructions that does not introduce undefined behavior.
//!    //
//!    // It is very difficult to define what would cause undefined behavior when
//!    // modifying program instructions. This would almost certainly result
//!    // in unwanted and likely undefined behavior. Do so at your own risk.
//!    unsafe {
//!        flash_controller.page_erase(test_addr, &sys_clk).unwrap();
//!        flash_controller
//!            .write(test_addr, &u32::to_le_bytes(test_val), &sys_clk)
//!            .unwrap();
//!    }
//!    flash_controller
//!        .read_bytes(test_addr, &mut data_read)
//!        .unwrap();
//!
//!    assert!(u32::from_le_bytes(data_read) == test_val);
//!    ```
use crate::peripherals::oscillator::SystemClock;
use max78000::{FLC, GCR, ICC0};

/// Flash memory base address.
pub const FLASH_MEM_BASE: u32 = 0x1000_0000;

/// Flash memory size.
pub const FLASH_MEM_SIZE: u32 = 0x0008_0000;

/// Flash page size.
pub const FLASH_PAGE_SIZE: u32 = 0x2000;

/// Error values a flash write operation throws.
#[derive(Debug)]
pub enum FlashErr {
    /// The flash address is not word aligned
    AddressNotAligned128,
    /// The pointer argument is not a valid flash address.
    PtrBoundsErr,
    /// The flash controller clock could not be set to 1MHz
    FlcClkErr,
}

/// Flash Controller peripheral.
pub struct FlashController<'gcr, 'icc> {
    flc: FLC,
    icc: &'icc ICC0,
    gcr: &'gcr GCR,
}

unsafe extern "C" {
    /// Reads a little-endian `u32` from flash memory.
    ///
    /// Panics if any of the following preconditions are not true:
    /// - `address` must be 32-bit aligned.
    /// - `address` must point to a valid location in flash memory (`0x1000_0000..=0x1007_ffff`).
    pub unsafe fn flc_read32_primitive(address: *const u32) -> u32;

    /// Erases the page at the given address in flash memory.
    ///
    /// Safety:
    /// - The caller must hold a shared reference to the [`FLC`], [`ICC0`], and [`GCR`] registers.
    /// - `address` must point to a valid page within flash memory (`0x1000_0000..=0x1007_ffff`).
    /// - `sys_clk_freq` must be equal to `freq / div` where `freq` is the frequency of
    ///   the current system clock, and `div` is the divider of the system clock.
    /// - `sys_clk_freq` must be divisible by one million (`1_000_000`).
    /// - If `address` erases a page in the currently-running program's instruction space,
    ///   it must be rewritten with [`write128`] before the program reaches those instructions.
    ///
    /// Panics if any of the following preconditions are not true:
    /// - `address` must point to within a valid page in flash space (`0x1000_000..=0x1007_ffff`)
    /// - `sys_clk_freq` must be divisible by one million (`1_000_000`).
    pub unsafe fn flc_page_erase_primitive(address: *mut u8, sys_clk_freq: u32);

    /// Writes a little-endian 128-bit flash word into flash memory.
    ///
    /// Safety:
    /// - The caller must hold a shared reference to the [`FLC`], [`ICC0`], and [`GCR`] registers.
    /// - The flash word at `address` must be in the *erased* state (with [`page_erase`]).
    /// - `data` must point to an array of four `u32`s.
    /// - `sys_clk_freq` must be equal to `freq / div` where `freq` is the frequency of
    ///   the current system clock, and `div` is the divider of the system clock.
    /// - If `address` writes to an address in the currently-running program's instruction space,
    ///   it must be valid instructions.
    ///
    /// Panics if any of the following preconditions are not true:
    /// - `address` must be 128-bit aligned.
    /// - The entire flash word at address (bytes `address..address + 16`) must be within
    ///   the flash memory (`0x1000_0000..=0x1007_ffff`).
    /// - `sys_clk_freq` must be divisible by one million (`1_000_000`).
    pub unsafe fn flc_write128_primitive(
        address: *mut [u32; 4],
        data: *const u32,
        sys_clk_freq: u32,
    );
}

/// Checks whether the given address range (exclusive) is within flash space, returning an `Err` if not.
#[inline(always)]
const fn check_address_bounds(address_range: core::ops::Range<u32>) -> Result<(), FlashErr> {
    if !(FLASH_MEM_BASE <= address_range.start
        && address_range.start < FLASH_MEM_BASE + FLASH_MEM_SIZE
        && FLASH_MEM_BASE < address_range.end
        && address_range.end <= FLASH_MEM_BASE + FLASH_MEM_SIZE)
    {
        Err(FlashErr::PtrBoundsErr)
    } else {
        Ok(())
    }
}

impl<'gcr, 'icc> FlashController<'gcr, 'icc> {
    /// Creates a new flash controller peripheral.
    pub(crate) fn new(flc: FLC, icc: &'icc ICC0, gcr: &'gcr GCR) -> Self {
        Self { flc, icc, gcr }
    }

    /// Calculates the correct `sys_clk_freq` from the passed [`SystemClock`] for FLC primitives.
    /// Returns an `Err` if the calculated frequency is not a multiple of `1_000_000`.
    fn get_clock_divisor(sys_clk: &SystemClock) -> Result<u32, FlashErr> {
        let sys_clk_freq = sys_clk.get_freq() / sys_clk.get_div() as u32;
        if sys_clk_freq % 1_000_000 != 0 {
            return Err(FlashErr::FlcClkErr);
        }

        Ok(sys_clk_freq)
    }

    /// Reads data from flash.
    pub fn read_bytes(address: u32, data: &mut [u8]) -> Result<(), FlashErr> {
        // change to range check
        check_address_bounds(address..(address + data.len() as u32))?;

        let mut next_read_address = address;

        // read from flash in word chunks
        let mut word_chunk = data.chunks_exact_mut(4);
        for word in &mut word_chunk {
            // SAFETY:
            // * src is valid for reads. Because read range is checked at the
            // beginning of function.
            //
            // * src is properly aligned. Because pointer is cast to an array of
            // u8s which is guaranteed to be aligned to 1.
            //
            // * src is pointing to a properly initialized value of type
            // T. Reading initialized mapped peripheral memory which is within
            // bounds of the flash memory space.
            unsafe {
                let buffer = core::ptr::read_volatile(next_read_address as *const [u8; 4]);
                word.copy_from_slice(&buffer);
            }
            next_read_address += 4;
        }

        for byte in word_chunk.into_remainder() {
            // SAFETY:CLOSURE
            // * src is valid for reads. Because read range is checked at the
            // beginning of function.
            //
            // * src is properly aligned. Because pointer is cast to an array of
            // u8s which is guaranteed to be aligned to 1.
            //
            // * src is pointing to a properly initialized value of type
            // T. Reading initialized mapped peripheral memory which is within
            // bounds of the flash memory space.
            unsafe {
                let buffer = core::ptr::read_volatile(next_read_address as *const u8);
                *byte = buffer;
            }
            next_read_address += 1;
        }

        Ok(())
    }

    /// # Safety
    ///
    /// Writes must not corrupt potentially executable instructions of the program.
    /// Callers must ensure that the following condition is met:
    /// * If `address` points to a portion of the program's instructions, `data` must
    ///   contain valid instructions that does not introduce undefined behavior.
    ///
    /// It is very difficult to define what would cause undefined behavior when
    /// modifying program instructions. This would almost certainly result
    /// in unwanted and likely undefined behavior. Do so at your own risk.
    pub unsafe fn write(
        &self,
        address: u32,
        data: &[u8],
        sys_clk: &SystemClock,
    ) -> Result<(), FlashErr> {
        check_address_bounds(address..(address + data.len() as u32))?;

        // Check alignment
        let mut physical_addr = address;
        let bytes_unaligned = if (address & 0xF) > 0 {
            16 - (address & 0xF) as usize
        } else {
            0
        };

        // Write unaligned data
        if bytes_unaligned > 0 {
            let unaligned_data = &data[0..core::cmp::min(bytes_unaligned, data.len())];
            self.write_lt_128_unaligned(physical_addr, unaligned_data, sys_clk)?;
            physical_addr += unaligned_data.len() as u32;
        }

        // Write aligned data in 128 bit chunks
        let mut chunk_8 = data[bytes_unaligned..].chunks_exact(16);
        let flc_words_chunks = chunk_8.by_ref().map(|word| {
            word.chunks_exact(4)
                .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
        });

        for word in flc_words_chunks {
            let mut buffer_128_bits: [u32; 4] = [0; 4];
            word.enumerate()
                .for_each(|(idx, chunk)| buffer_128_bits[idx] = chunk);
            self.write128(physical_addr, &buffer_128_bits, sys_clk)?;
            physical_addr += 16;
        }

        // remainder from chunks
        if !chunk_8.remainder().is_empty() {
            self.write_lt_128_unaligned(physical_addr, chunk_8.remainder(), sys_clk)?;
        }

        Ok(())
    }

    /// Writes less than 128 bits (16 bytes) of data to flash.
    /// Data needs to fit within one flash word (16 bytes).
    ///
    /// # Safety
    ///
    /// Writes must not corrupt potentially executable instructions of the program.
    /// Callers must ensure that the following condition is met:
    /// * If `address` points to a portion of the program's instructions, `data` must
    ///   contain valid instructions that does not introduce undefined behavior.
    ///
    /// It is very difficult to define what would cause undefined behavior when
    /// modifying program instructions. This would almost certainly result
    /// in unwanted and likely undefined behavior. Do so at your own risk.
    unsafe fn write_lt_128_unaligned(
        &self,
        address: u32,
        data: &[u8],
        sys_clk: &SystemClock,
    ) -> Result<(), FlashErr> {
        // Get byte idx within 128-bit word
        let byte_idx = (address & 0xF) as usize;

        // Align address to 128-bit word
        let aligned_addr = address & !0xF;

        let mut current_bytes: [u8; 16] = [0; 16];
        Self::read_bytes(aligned_addr, &mut current_bytes[..])?;

        // construct 128 bits of data to write back to flash
        current_bytes[byte_idx..(byte_idx + data.len())].copy_from_slice(data);

        let mut new_data: [u32; 4] = [0; 4];

        current_bytes
            .chunks(4)
            .enumerate()
            .for_each(|(idx, word_chunk)| {
                new_data[idx] = u32::from_le_bytes(word_chunk.try_into().unwrap())
            });

        self.write128(aligned_addr, &new_data, sys_clk)
    }

    /// Writes 128 bits (16 bytes) of data to flash.
    /// Address must be 128-bit aligned.
    ///
    /// # Safety
    ///
    /// The flash word at `address` must be in the *erased* state.  Otherwise, the write will not
    /// occur.
    ///
    /// Writes must not corrupt potentially executable instructions of the program.
    /// Callers must ensure that the following condition is met:
    /// * If `address` points to a portion of the program's instructions, `data` must
    ///   contain valid instructions that does not introduce undefined behavior.
    ///
    /// It is very difficult to define what would cause undefined behavior when
    /// modifying program instructions. This would almost certainly result
    /// in unwanted and likely undefined behavior. Do so at your own risk.
    unsafe fn write128(
        &self,
        address: u32,
        data: &[u32; 4],
        sys_clk: &SystemClock,
    ) -> Result<(), FlashErr> {
        // Check if adddress is 128-bit aligned
        if address & 0xF > 0 {
            return Err(FlashErr::AddressNotAligned128);
        }
        check_address_bounds(address..address + 16)?;

        let sys_clk_freq = Self::get_clock_divisor(sys_clk)?;

        // SAFETY: per the safety contract of [`flc_write128_primitive`]:
        // - we hold a reference (in `self`) to the FLC, ICC0, and GCR registers
        // - the caller guarantees that the flash word at `address` in the erased state.
        // - `data.as_ptr()` points to an array of 4 u32s.
        // - `sys_clk_freq` is calculated as `freq / div` of the current system clock above
        // - the caller must guarantee that, if the word at `address` is in instruction memory,
        //   it contains safe and valid instructions.
        critical_section::with(|_| unsafe {
            flc_write128_primitive(address as *mut [u32; 4], data.as_ptr(), sys_clk_freq);
        });

        Ok(())
    }

    /// Erases a page of flash. FLC_ADDR\[12:0\] is ignored to ensure the address
    /// is page-aligned.
    ///
    /// # Safety
    ///
    /// Erases must not corrupt potentially executable instructions of the program.
    /// Behavior is undefined if any of the following conditions are violated:
    /// * `address` must be in a valid flash page
    pub unsafe fn page_erase(&self, address: u32, sys_clk: &SystemClock) -> Result<(), FlashErr> {
        check_address_bounds(address..address + 1)?;
        let sys_clk_freq = Self::get_clock_divisor(sys_clk)?;

        // SAFETY: per the safety contract of [`flc_page_erase_primitive`]:
        // - we hold a reference (in `self`) to the FLC, ICC0, and GCR registers.
        // - `sys_clk_freq` is calculated as `freq / div` of the current system clock above.
        // - the caller guarantees safety if the erased page is part of instruction memory.
        critical_section::with(|_| unsafe {
            flc_page_erase_primitive(address as *mut u8, sys_clk_freq);
        });

        Ok(())
    }

    /// Erases the entire flash.
    ///
    /// # Safety
    ///
    /// Mass erase clears the whole flash. Program must be executed from SRAM.
    pub unsafe fn mass_erase(&self) -> Result<(), FlashErr> {
        // Make sure to disable and enable icc0 at the beginning and end of function
        todo!()
    }
}
