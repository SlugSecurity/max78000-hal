//! Flash controller peripheral API.

use max78000::{FLC, ICC0};

const FLASH_MEM_BASE: u32 = 0x1000_0000;
const FLASH_MEM_SIZE: u32 = 0x0007_FFFF;

enum FlcReadErr {
    PtrBoundsErr,
    Succ,
}

enum FlcWriteErr {
    AddressNotAlignedByte,
    AddressNotAlignedWord,
    PtrBoundsErr,
    Succ,
}

/// Flash Controller peripheral.
pub struct FlashController<'a> {
    flc: FLC,
    icc: &'a ICC0,
}

// TODO: Implement with the peripheral API when available.

impl<'a> FlashController<'a> {
    /// Creates a new flash controller peripheral.
    // TODO: Make this function pub(crate) when the peripheral API is available. Tests needs it public until then.
    pub fn new(flc: FLC, icc: &'a ICC0) -> Self {
        Self { flc, icc }
    }

    fn check_address_bounds(&self, address: u32) -> bool {
        if address >= FLASH_MEM_BASE && address < (FLASH_MEM_BASE + FLASH_MEM_SIZE) {
            return true;
        } else {
            return false;
        }
    }

    fn get_physical_address(&self, address: u32) -> u32 {
        address - self::FLASH_MEM_BASE
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
        // TODO: Finish.
        // Page 57 of user manual under Unified Internal Cache Controllers
    }

    /// Reads data from flash.
    pub fn read_bytes(&self, address: u32, data: &mut [u8]) -> FlcReadErr {
        if !self.check_address_bounds(address as u32) {
            return FlcReadErr::PtrBoundsErr;
        }

        // Safety
        // Behavior is undefined if any of the following conditions are violated:
        //     src must be valid for reads of count * size_of::<T>() bytes.
        //     dst must be valid for writes of count * size_of::<T>() bytes.
        //     Both src and dst must be properly aligned.
        //     The region of memory beginning at src with a size of count *
        //     size_of::<T>() bytes must not overlap with the region of
        //     memory beginning at dst with the same size.
        // Like read, copy_nonoverlapping creates a bitwise copy of T,
        // regardless of whether T is Copy. If T is not Copy, using both the
        // values in the region beginning at *src and the region beginning
        // at *dst can violate memory safety.

        unsafe {
            core::ptr::copy_nonoverlapping(address as *const u8, data.as_mut_ptr(), data.len());
        }

        FlcReadErr::Succ
    }

    /// Write arbitary number of bytes of data to flash.
    // make sure to disable ICC with ICC_Disable(); before Running this function
    pub fn write(&self, address: u32, data: &[u8]) -> FlcWriteErr {
        // Check address bounds
        if !self.check_address_bounds(address) {
            return FlcWriteErr::PtrBoundsErr;
        }

        let mut physical_addr = self.get_physical_address(address);
        let bytes_unaligned = (address & 0xF) as usize;

        // Write unaligned data
        if bytes_unaligned > 0 {
            self.write_lt_128(physical_addr, &data[0..bytes_unaligned]);
            physical_addr = physical_addr + bytes_unaligned as u32;
        }

        // If data left is less than 128 bits (16 bytes)
        if data[bytes_unaligned..].len() < 16 {
            self.write_lt_128(physical_addr, &data[bytes_unaligned..]);
            return FlcWriteErr::Succ;
        }

        let word_chunks = data[bytes_unaligned..].chunks_exact(4);
        let slice_32 = word_chunks
            .into_iter()
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()));

        let mut slice128: [u32; 4];
        for (idx, word) in slice_32.into_iter().enumerate() {
            if idx != 0 && idx % 4 == 0 {
                self.write128(physical_addr, &slice128);
                physical_addr += 16;
            }

            slice128[0 % 4] = word;
        }

        if word_chunks.remainder().len() > 0 {
            self.write_lt_128(physical_addr, word_chunks.remainder());
        }

        FlcWriteErr::Succ
    }

    /// Writes less than 128 bits (16 bytes) of data to flash. Data should be byte aligned.
    // make sure to disable ICC with ICC_Disable(); before Running this function
    fn write_lt_128(&self, address: u32, data: &[u8]) -> FlcWriteErr {
        // Check if adddress is byte addressable
        if address & 0x3 > 0 {
            return FlcWriteErr::AddressNotAlignedByte;
        }

        // Get byte idx within 128-bit word
        let byte_idx = address & 0xF;

        // Align address to 128-bit word
        let aligned_addr = address & !0xF;

        let mut current_bytes: [u8; 16];
        self.read_bytes(aligned_addr, &mut current_bytes[..]);

        // construct 128 bits of data to write back to flash
        current_bytes[byte_idx as usize..].copy_from_slice(data);
        let mut new_data: [u32; 4];
        for (idx, word_chunk) in current_bytes.chunks(4).into_iter().enumerate() {
            new_data[idx] = u32::from_le_bytes(word_chunk.try_into().unwrap());
        }

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

        self.flc.ctrl().modify(|r, w| {
            while r.pend().bit_is_clear() == false {}
            w
        });

        self.set_clock_divisor();
        unsafe {
            self.flc.addr().modify(|_, w| w.bits(address));
            self.flc.data(0).modify(|r, w| w.bits(data[0]));
            self.flc.data(1).modify(|r, w| w.bits(data[1]));
            self.flc.data(2).modify(|r, w| w.bits(data[2]));
            self.flc.data(3).modify(|r, w| w.bits(data[3]));
        }

        self.unlock_write_protection();
        // Turn on write bit
        // The hardware automatically clears this field when the write
        // operation is complete.
        self.flc.ctrl().modify(|r, w| w.wr().set_bit());
        self.flc.intr().modify(|r, w| {
            while r.done().bit_is_set() == false {}
            w
        });

        // If an error occurred, the FLC_INTR.af field is set to 1 by
        // hardware. An interrupt is generated if the FLC_INTR.afie field is
        // set to 1.

        self.lock_write_protection();
        self.flush_icc();
        FlcWriteErr::Succ
    }

    // TODO: Finish adding functions for the flash controller. No need to
    // implement async/interrupts.
}
