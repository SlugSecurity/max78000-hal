//! Flash controller peripheral API.

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
    AddressNotAlignedWord,
    PtrBoundsErr,
    FlcClkErr,
    Succ,
}

#[derive(Debug)]
pub enum FlashClkErr {
    SYS_CLK_TOO_LOW,
    Succ,
}

/// Flash Controller peripheral.
pub struct FlashController<'gcr, 'icc> {
    flc: FLC,
    icc: &'icc ICC0,
    gcr: &'gcr GCR,
}

impl<'gcr, 'icc> Drop for FlashController<'gcr, 'icc> {
    fn drop(&mut self) {
        self.enable_icc0();
    }
}

impl<'gcr, 'icc> FlashController<'gcr, 'icc> {
    /// Creates a new flash controller peripheral.
    pub fn new(flc: FLC, icc: &'icc ICC0, gcr: &'gcr GCR) -> Self {
        // TODO: Make this function pub(crate) when the peripheral API is available. Tests needs it public until then.
        let new_flc = Self { flc, icc, gcr };
        new_flc.disable_icc0();

        new_flc
    }

    /// Checks the address to see if it is a valid flash memory address
    fn check_address_bounds(&self, address: u32) -> bool {
        (FLASH_MEM_BASE..(FLASH_MEM_BASE + FLASH_MEM_SIZE)).contains(&address)
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
    fn set_clock_divisor(&self, sys_clk: &SystemClock) -> Result<FlashClkErr, FlashClkErr> {
        let sys_clk_freq = sys_clk.get_freq() / sys_clk.get_div() as u32;
        let flc_clkdiv = 1 / sys_clk_freq;

        if flc_clkdiv <= 0 {
            return Err(FlashClkErr::SYS_CLK_TOO_LOW);
        }

        self.flc
            .clkdiv()
            .modify(|_, w| w.clkdiv().variant(flc_clkdiv as u8));

        Ok(FlashClkErr::Succ)
    }

    /// Flushes the flash line buffer and arm instruction cache.
    ///
    /// This MUST be called after any write/erase flash controller operations.
    fn flush_icc(&self) {
        self.icc.invalidate().modify(|_, w| w.invalid().variant(1));
        while !self.icc.ctrl().read().rdy().bit_is_set() {}

        // Clear the line fill buffer by reading 2 pages from flash
        let ptr = FLASH_MEM_BASE;
        let mut empty_buffer = [];
        if let Err(why) = self.read_bytes(ptr, &mut empty_buffer) {
            match why {
                FlashErr::AddressNotAlignedWord => panic!("Address {} not aligned with word", ptr),
                FlashErr::PtrBoundsErr => panic!("Address {} is not a valid flash address", ptr),
                _ => (),
            }
        }

        if let Err(why) = self.read_bytes(ptr + FLASH_PAGE_SIZE, &mut empty_buffer) {
            match why {
                FlashErr::AddressNotAlignedWord => panic!("Address {} not aligned with word", ptr),
                FlashErr::PtrBoundsErr => panic!("Address {} is not a valid flash address", ptr),
                _ => (),
            }
        }
    }

    /// Disables instruction cache.
    ///
    /// This MUST be called before any non-read flash controller operations.
    fn disable_icc0(&self) {
        self.icc.ctrl().modify(|_, w| w.en().dis());
    }

    /// Disables instruction cache.
    ///
    /// This MUST be called after any non-read flash controller operations.
    fn enable_icc0(&self) {
        // ensure the cache is invalidated when enabled
        self.disable_icc0();

        self.icc.ctrl().modify(|_, w| w.en().en());
        while !self.icc.ctrl().read().rdy().bit_is_set() {}

        // zeroize the icc instance
        self.gcr.memz().modify(|_, w| w.icc0().set_bit());
    }

    /// Reads data from flash.
    pub fn read_bytes(&self, address: u32, data: &mut [u8]) -> Result<FlashErr, FlashErr> {
        if !self.check_address_bounds(address) {
            return Err(FlashErr::PtrBoundsErr);
        }

        unsafe {
            data.copy_from_slice(core::ptr::read_volatile(address as *const &[u8]));
        }

        Ok(FlashErr::Succ)
    }

    /// Write arbitary number of bytes of data to flash.
    pub unsafe fn write(
        &self,
        address: u32,
        data: &[u8],
        sys_clk: &SystemClock,
    ) -> Result<FlashErr, FlashErr> {
        // Check address bounds
        if !self.check_address_bounds(address) {
            return Err(FlashErr::PtrBoundsErr);
        }

        let mut physical_addr = address;
        let bytes_unaligned = if (address & 0xF) > 0 {
            16 - (address & 0xF) as usize
        } else {
            0
        };

        // Write unaligned data
        if bytes_unaligned > 0 {
            if let Err(why) = self.write_lt_128(
                physical_addr,
                &data[0..core::cmp::min(bytes_unaligned, data.len())],
                sys_clk,
            ) {
                return Err(why);
            }

            physical_addr += bytes_unaligned as u32;
        }

        // If data left after writing unaligned part is less than 128 bits (16
        // bytes)
        if data[bytes_unaligned..].len() < 16 {
            if let Err(why) = self.write_lt_128(physical_addr, &data[bytes_unaligned..], sys_clk) {
                return Err(why);
            }
            return Ok(FlashErr::Succ);
        }
        // If all data has already been written
        else if bytes_unaligned == data.len() {
            return Ok(FlashErr::Succ);
        }

        // If data left is more than 128 bits (16 bytes)
        let chunk_8 = data[bytes_unaligned..].chunks_exact(4);
        let chunk_32 = chunk_8
            .clone()
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()));

        let mut buffer_128_bits: [u32; 4] = [0; 4];
        let mut bytes_in_buffer = 0;
        for (idx, word) in chunk_32.into_iter().enumerate() {
            // If buffer is filled with user data
            buffer_128_bits[idx % 4] = word;
            bytes_in_buffer += 4;

            if bytes_in_buffer == 16 {
                if let Err(why) = self.write128(physical_addr, &buffer_128_bits, sys_clk) {
                    return Err(why);
                }
                physical_addr += 16;
                bytes_in_buffer = 0;
            }
        }

        // remainder from chunks
        let data_left_idx = (physical_addr - address) as usize;
        if bytes_in_buffer > 0 {
            if let Err(why) = self.write_lt_128(physical_addr, &data[data_left_idx..], sys_clk) {
                return Err(why);
            }
        } else if !chunk_8.remainder().is_empty() {
            if let Err(why) = self.write_lt_128(physical_addr, chunk_8.remainder(), sys_clk) {
                return Err(why);
            }
        }

        Ok(FlashErr::Succ)
    }

    /// Writes less than 128 bits (16 bytes) of data to flash.
    fn write_lt_128(
        &self,
        address: u32,
        data: &[u8],
        sys_clk: &SystemClock,
    ) -> Result<FlashErr, FlashErr> {
        // Get byte idx within 128-bit word
        let byte_idx = (address & 0xF) as usize;

        // Align address to 128-bit word
        let aligned_addr = address & !0xF;

        let mut current_bytes: [u8; 16] = [0; 16];
        if let Err(why) = self.read_bytes(aligned_addr, &mut current_bytes[..]) {
            return Err(why);
        }

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
    // make sure to disable ICC with ICC_Disable(); before Running this function
    fn write128(
        &self,
        address: u32,
        data: &[u32; 4],
        sys_clk: &SystemClock,
    ) -> Result<FlashErr, FlashErr> {
        // Check if adddress is 128-bit aligned
        if address & 0xF > 0 {
            return Err(FlashErr::PtrBoundsErr);
        }

        if let Err(_) = self.set_clock_divisor(sys_clk) {
            return Err(FlashErr::FlcClkErr);
        };

        self.unlock_write_protection();

        // Clear stale errors
        self.flc.intr().modify(|_, w| w.af().clear_bit());

        while !self.flc.ctrl().read().pend().bit_is_clear() {}

        self.flc.addr().modify(|_, w| w.addr().variant(address));
        self.flc.data(0).modify(|_, w| w.data().variant(data[0]));
        self.flc.data(1).modify(|_, w| w.data().variant(data[1]));
        self.flc.data(2).modify(|_, w| w.data().variant(data[2]));
        self.flc.data(3).modify(|_, w| w.data().variant(data[3]));

        self.flc.ctrl().modify(|_, w| w.wr().set_bit());
        while !self.flc.ctrl().read().wr().is_complete() {}

        self.lock_write_protection();
        self.flush_icc();

        Ok(FlashErr::Succ)
    }

    /// Erases a page of flash. FLC_ADDR\[12:0\] is ignored to ensure the address
    /// is page-aligned.
    pub unsafe fn page_erase(
        &self,
        address: u32,
        sys_clk: &SystemClock,
    ) -> Result<FlashErr, FlashErr> {
        if !self.check_address_bounds(address) {
            return Err(FlashErr::PtrBoundsErr);
        }

        if let Err(_) = self.set_clock_divisor(sys_clk) {
            return Err(FlashErr::FlcClkErr);
        }

        self.unlock_write_protection();

        // Clear stale errors
        self.flc.intr().modify(|_, w| w.af().clear_bit());

        while !self.flc.ctrl().read().pend().bit_is_clear() {}

        self.flc.addr().modify(|_, w| w.addr().variant(address));

        self.flc.ctrl().modify(|_, w| w.erase_code().erase_page());
        self.flc.ctrl().modify(|_, w| w.pge().set_bit());

        while !self.flc.ctrl().read().pend().bit_is_clear() {}

        self.lock_write_protection();
        self.flush_icc();

        Ok(FlashErr::Succ)
    }

    /// Erases the entire flash.
    pub fn mass_erase(&self) -> Result<FlashErr, FlashErr> {
        todo!()
    }
}
