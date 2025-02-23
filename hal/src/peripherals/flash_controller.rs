//! Flash controller peripheral API.
use core::borrow::BorrowMut;

///    # Examples
///    ```
///    let flash_controller = FlashController::new(flc, icc0, gcr);
///
///    let test_addr: u32 = 0x10070FF0;
///    let test_val: u32 = 0xCAFEBABE;
///    let mut data_read: [u8; 4] = [0; 4];
///
///    // # Safety
///    // Non-read flash operations must not corrupt potentially instructions of the
///    // program.
///    // Callers must ensure that the following condition is met:
///    // * If `address` points to a portion of the program's instructions, `data` must
///    //   contain valid instructions that does not introduce undefined behavior.
///    //
///    // It is very difficult to define what would cause undefined behavior when
///    // modifying program instructions. This would almost certainly result
///    // in unwanted and likely undefined behavior. Do so at your own risk.
///    unsafe {
///        flash_controller.page_erase(test_addr, &sys_clk).unwrap();
///        flash_controller
///            .write(test_addr, &u32::to_le_bytes(test_val), &sys_clk)
///            .unwrap();
///    }
///    flash_controller
///        .read_bytes(test_addr, &mut data_read)
///        .unwrap();
///
///    assert!(u32::from_le_bytes(data_read) == test_val);
///    ```
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

impl<'gcr, 'icc> FlashController<'gcr, 'icc> {
    /// Creates a new flash controller peripheral.
    pub(crate) fn new(flc: FLC, icc: &'icc ICC0, gcr: &'gcr GCR) -> Self {
        Self { flc, icc, gcr }
    }

    /// Checks the address to see if it is a valid flash memory address
    fn check_address_bounds(&self, address_range: core::ops::Range<u32>) -> Result<(), FlashErr> {
        if (FLASH_MEM_BASE..(FLASH_MEM_BASE + FLASH_MEM_SIZE)).contains(&address_range.start)
            && (FLASH_MEM_BASE..(FLASH_MEM_BASE + FLASH_MEM_SIZE)).contains(&address_range.end)
        {
            Ok(())
        } else {
            Err(FlashErr::PtrBoundsErr)
        }
    }

    /// Unlocks memory protection to allow flash operations
    ///
    /// This MUST be called before any non-read flash controller operation.
    fn unlock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().unlocked());
    }

    /// Locks memory protection.
    ///
    /// This MUST be called after any non-read flash controller operation.
    fn lock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().locked());
    }

    /// Checks if the flash controller's clock divisor is correct and if not, sets it. Correct
    /// clock frequency is 1 MHz.
    ///
    /// This MUST be called before any non-read flash controller operations.
    fn set_clock_divisor(&self, sys_clk: &SystemClock) -> Result<(), FlashErr> {
        let sys_clk_freq = sys_clk.get_freq() / sys_clk.get_div() as u32;
        if sys_clk_freq % 1_000_000 != 0 {
            return Err(FlashErr::FlcClkErr);
        }

        let flc_clkdiv = sys_clk_freq / 1_000_000;

        self.flc
            .clkdiv()
            .modify(|_, w| w.clkdiv().variant(flc_clkdiv as u8));

        Ok(())
    }

    /// Busy loop until FLC is ready.
    ///
    /// This MUST be called BEFORE any FLC operation EXCEPT clearing interrupts.
    fn wait_until_ready(&self) {
        while !self.flc.ctrl().read().pend().bit_is_clear() {}
    }

    /// Clear any stale errors in the FLC Interrupt Register
    ///
    /// This can be called without waiting for the FLC to be ready.
    fn clear_interrupts(&self) {
        self.flc.intr().modify(|_, w| w.af().clear_bit());
    }

    /// Prepares the FLC for a write operation.
    ///
    /// Procedure:
    ///  - Wait until FLC ready
    ///  - Disable icc0
    ///  - Clear FLC interrupts
    ///  - Set clock divisor
    ///  - Unlock write protection
    ///  - EXECUTE OPERATION (CLOSURE)
    ///  - Wait until FLC ready
    ///  - Lock write protection
    ///  - Flush ICC
    ///  - Enable icc0
    fn write_guard<F: Fn()>(
        &self,
        sys_clk: &SystemClock,
        operation: F,
    ) -> Result<(), FlashErr> {
        // Pre-write
        self.wait_until_ready();
        self.disable_icc0();
        self.clear_interrupts();
        self.set_clock_divisor(sys_clk)?;
        self.unlock_write_protection();

        operation();

        // Post-write
        self.wait_until_ready();
        self.lock_write_protection();
        self.flush_icc()?;
        self.enable_icc0();

        Ok(())
    }

    /// Flushes the flash line buffer and arm instruction cache.
    ///
    /// This MUST be called after any write/erase flash controller operations.
    fn flush_icc(&self) -> Result<(), FlashErr> {
        self.gcr.sysctrl().modify(|_, w| w.icc0_flush().flush());
        while !self.gcr.sysctrl().read().icc0_flush().bit_is_clear() {}

        // Clear the line fill buffer by reading 2 pages from flash
        let ptr = FLASH_MEM_BASE;
        let mut empty_buffer = [0; 4];
        self.read_bytes(ptr, &mut empty_buffer)?;
        self.read_bytes(ptr + FLASH_PAGE_SIZE, &mut empty_buffer)?;

        Ok(())
    }

    /// Disables instruction cache.
    ///
    /// This MUST be called before any non-read flash controller operations.
    fn disable_icc0(&self) {
        self.icc.ctrl().modify(|_, w| w.en().dis());
    }

    /// Enables instruction cache.
    ///
    /// This MUST be called after any non-read flash controller operations.
    fn enable_icc0(&self) {
        // ensure the cache is invalidated when enabled
        self.disable_icc0();

        self.icc.invalidate().modify(|_, w| w.invalid().variant(1));
        while !self.icc.ctrl().read().rdy().bit_is_set() {}

        self.icc.ctrl().modify(|_, w| w.en().en());
        while !self.icc.ctrl().read().rdy().bit_is_set() {}
    }

    /// Reads data from flash.
    pub fn read_bytes(&self, address: u32, data: &mut [u8]) -> Result<(), FlashErr> {
        // change to range check
        self.check_address_bounds(address..(address + data.len() as u32))?;

        let mut next_read_address = address;

        // read from flash in word chunks
        let mut word_chunk = data.chunks_exact_mut(4);
        for word in word_chunk.borrow_mut() {
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
    pub unsafe fn write(
        &self,
        address: u32,
        data: &[u8],
        sys_clk: &SystemClock,
    ) -> Result<(), FlashErr> {
        self.check_address_bounds(address..(address + data.len() as u32))?;

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

    /// Writes less than 128 bits (16 bytes) of data to flash. Data needs to fit
    /// within one flash word (16 bytes).
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
        self.read_bytes(aligned_addr, &mut current_bytes[..])?;

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

        self.write_guard(sys_clk, || {
            self.flc.addr().modify(|_, w| w.addr().variant(address));
            self.flc.data(0).modify(|_, w| w.data().variant(data[0]));
            self.flc.data(1).modify(|_, w| w.data().variant(data[1]));
            self.flc.data(2).modify(|_, w| w.data().variant(data[2]));
            self.flc.data(3).modify(|_, w| w.data().variant(data[3]));

            self.flc.ctrl().modify(|_, w| w.wr().set_bit());

            // Wait until write completes
            while !self.flc.ctrl().read().wr().is_complete() {}
        })?;

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
        self.check_address_bounds(address..address)?;

        self.write_guard(sys_clk, || {
            self.flc.addr().modify(|_, w| w.addr().variant(address));

            self.flc.ctrl().modify(|_, w| w.erase_code().erase_page());
            self.flc.ctrl().modify(|_, w| w.pge().set_bit());
        })?;

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
