//! Flash controller peripheral API.

use max78000::{FLC, GCR, ICC0};

pub const FLASH_MEM_BASE: u32 = 0x1000_0000;
pub const FLASH_MEM_SIZE: u32 = 0x0007_FFFF;
pub const FLASH_PAGE_SIZE: u32 = 0x2000;

pub enum FlcReadErr {
    PtrBoundsErr,
    Succ,
}

pub enum FlcWriteErr {
    AddressNotAlignedByte,
    AddressNotAlignedWord,
    PtrBoundsErr,
    Succ,
}

pub enum FlcEraseErr {
    PtrBoundsErr,
    Succ,
}

/// Flash Controller peripheral.
pub struct FlashController<'a> {
    flc: FLC,
    icc: &'a ICC0,
    gcr: &'a GCR,
}

// TODO: Implement with the peripheral API when available.

impl<'a> FlashController<'a> {
    /// Creates a new flash controller peripheral.
    pub fn new(flc: FLC, icc: &'a ICC0, gcr: &'a GCR) -> Self {
        // TODO: Make this function pub(crate) when the peripheral API is available. Tests needs it public until then.
        Self { flc, icc, gcr }
    }

    fn check_address_bounds(&self, address: u32) -> bool {
        if address >= FLASH_MEM_BASE && address < (FLASH_MEM_BASE + FLASH_MEM_SIZE) {
            return true;
        } else {
            return false;
        }
    }

    /// Unlocks memory protection to allow flash operations
    fn unlock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().unlocked());
    }

    fn lock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().locked());
    }

    /// Checks if the flash controller's clock divisor is correct and if not, sets it. Correct
    /// clock frequency is 1 MHz.
    ///
    /// This MUST be called before any non-read flash controller operations.
    fn set_clock_divisor(&self) {
        // TODO: Finish.
    }

    /// Flushes the data and instruction cache.
    ///
    /// This MUST be called after any write/erase flash controller operations.

    // This function should be called after modifying the contents of flash memory.
    // It flushes the instruction caches and line fill buffer.

    // It should be called _afterwards_ because after flash is modified the cache
    // may contain instructions that may no longer be valid.  _Before_ the
    // flash modifications the ICC may contain relevant cached instructions related to
    // the incoming flash instructions (especially relevant in the case of external memory),
    // and these instructions will be valid up until the point that the modifications are made.

    // The line fill buffer is a FLC-related buffer that also may no longer be valid.
    // It's flushed by reading 2 pages of flash.
    // https://github.com/Analog-Devices-MSDK/msdk/blob/main/Libraries/PeriphDrivers/Source/FLC/flc_ai87.c

    fn flush_icc(&self) {
        self.icc.invalidate().modify(|_, w| w.invalid().variant(1));
        while self.icc.ctrl().read().rdy().bit_is_set() == false {}

        // Clear the line fill buffer by reading 2 pages from flash
        unsafe {
            let ptr = FLASH_MEM_BASE as *const u32;
            core::ptr::read_volatile(ptr);
            core::ptr::read_volatile(ptr.add(FLASH_PAGE_SIZE as usize));
        }
    }

    pub fn disable_icc0(&self) {
        self.icc.ctrl().modify(|_, w| w.en().dis());
    }

    pub fn enable_icc0(&self) {
        // ensure the cache is invalidated when enabled
        self.disable_icc0();

        self.icc.ctrl().modify(|_, w| w.en().en());
        while self.icc.ctrl().read().rdy().bit_is_set() == false {}

        // zeroize the icc instance
        self.gcr.memz().modify(|_, w| w.icc0().set_bit());
    }

    /// Reads data from flash.
    pub fn read_bytes(&self, address: u32, data: &mut [u8]) -> FlcReadErr {
        if !self.check_address_bounds(address as u32) {
            return FlcReadErr::PtrBoundsErr;
        }

        unsafe {
            core::ptr::copy_nonoverlapping(address as *const u8, data.as_mut_ptr(), data.len());
        }

        FlcReadErr::Succ
    }

    /// Write arbitary number of bytes of data to flash.
    // make sure to disable ICC with ICC_Disable(); before Running this function

    // Stuff that needs to be taken care of ...
    // unaligned data needs to written to the correct address while maintaining the 128bit write
    // page erases need to be accurate, for example, if a single write spans multiple pages
    pub fn write(&self, address: u32, data: &[u8]) -> FlcWriteErr {
        // Check address bounds
        if !self.check_address_bounds(address) {
            return FlcWriteErr::PtrBoundsErr;
        }

        let mut physical_addr = address;
        let bytes_unaligned = if (address & 0xF) > 0 {
            16 - (address & 0xF) as usize
        } else {
            0
        };

        let bytes_unaligned_idx = if bytes_unaligned > 0 {
            bytes_unaligned - 1
        } else {
            0
        };

        // Write unaligned data
        if bytes_unaligned > 0 {
            self.write_lt_128(
                physical_addr,
                &data[0..core::cmp::min(bytes_unaligned, data.len())],
            );

            physical_addr = physical_addr + bytes_unaligned as u32;
        }

        // If data left is less than 128 bits (16 bytes)
        if bytes_unaligned < data.len() && data[bytes_unaligned_idx..].len() < 16 {
            self.write_lt_128(physical_addr, &data[bytes_unaligned_idx..]);
            return FlcWriteErr::Succ;
        } else if bytes_unaligned >= data.len() {
            return FlcWriteErr::Succ;
        }

        // If data left is more than 128 bits (16 bytes)
        let chunk_8 = data[bytes_unaligned_idx..].chunks_exact(4);
        let chunk_32 = chunk_8
            .clone()
            .into_iter()
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()));

        let mut buffer_128_bits: [u32; 4] = [0; 4];
        let mut bytes_written = 0;
        for (idx, word) in chunk_32.into_iter().enumerate() {
            // If buffer is filled with user data
            if idx != 0 && idx % 4 == 0 {
                self.write128(physical_addr, &buffer_128_bits);
                bytes_written += 16;
                physical_addr += 16;
            }

            buffer_128_bits[idx % 4] = word;
        }

        // remainder from chunks
        if bytes_written < data.len() {
            self.write_lt_128(physical_addr, &data[bytes_written..]);
        } else if chunk_8.remainder().len() > 0 {
            self.write_lt_128(physical_addr, chunk_8.remainder());
        }

        FlcWriteErr::Succ
    }

    /// Writes less than 128 bits (16 bytes) of data to flash. Data should be byte aligned.
    // make sure to disable ICC with ICC_Disable(); before Running this function
    fn write_lt_128(&self, address: u32, data: &[u8]) -> FlcWriteErr {
        // Get byte idx within 128-bit word
        let byte_idx = (address & 0xF) as usize;

        // Align address to 128-bit word
        let aligned_addr = address & !0xF;

        let mut current_bytes: [u8; 16] = [0; 16];
        self.read_bytes(aligned_addr, &mut current_bytes[..]);

        // construct 128 bits of data to write back to flash
        current_bytes[byte_idx..(byte_idx + data.len())].copy_from_slice(data);

        let mut new_data: [u32; 4] = [0; 4];

        current_bytes
            .chunks(4)
            .enumerate()
            .for_each(|(idx, word_chunk)| {
                new_data[idx] = u32::from_le_bytes(word_chunk.try_into().unwrap())
            });

        self.write128(aligned_addr, &new_data)
    }

    /// Writes 128 bits (16 bytes) of data to flash.
    // make sure to disable ICC with ICC_Disable(); before Running this function
    fn write128(&self, address: u32, data: &[u32; 4]) -> FlcWriteErr {
        // Check if adddress is 128-bit aligned
        if address & 0xF > 0 {
            return FlcWriteErr::AddressNotAlignedWord;
        }

        // If desired, enable the flash controller interrupts by setting the
        // FLC_INTR.afie and FLC_INTR.doneie bits.

        while self.flc.ctrl().read().pend().is_busy() == true {}

        self.set_clock_divisor();

        self.flc.addr().modify(|_, w| w.addr().variant(address));
        self.flc.data(0).modify(|_, w| w.data().variant(data[0]));
        self.flc.data(1).modify(|_, w| w.data().variant(data[1]));
        self.flc.data(2).modify(|_, w| w.data().variant(data[2]));
        self.flc.data(3).modify(|_, w| w.data().variant(data[3]));

        self.unlock_write_protection();

        // Turn on write bit
        // The hardware automatically clears this field when the write
        // operation is complete.

        self.flc.ctrl().modify(|_, w| w.wr().set_bit());
        while self.flc.ctrl().read().wr().is_complete() == false {}

        self.lock_write_protection();
        self.flush_icc();
        FlcWriteErr::Succ
    }

    pub fn page_erase(&self, address: u32) -> FlcEraseErr {
        if !self.check_address_bounds(address) {
            return FlcEraseErr::PtrBoundsErr;
        }

        while self.flc.ctrl().read().pend().bit_is_clear() == false {}

        self.set_clock_divisor();

        //  FLC_ADDR[12:0] is ignored by the FLC to ensure the address is
        //  page-aligned.
        self.flc.addr().modify(|_, w| w.addr().variant(address));

        self.unlock_write_protection();
        self.flc.ctrl().modify(|_, w| w.erase_code().erase_page());
        self.flc.ctrl().modify(|_, w| w.pge().set_bit());

        while self.flc.ctrl().read().pend().bit_is_clear() == false {}

        self.lock_write_protection();
        self.flush_icc();
        FlcEraseErr::Succ
    }
}
