//! Flash controller peripheral API, but in asm
//!
//! This generates the `flc_asm.s` assembly file in the flash controller,
//! to ensure that all function calls in the HAL primitives are located in RAM.
//!
//! Compile with `cargo rustc --release -- --emit=asm`, then find the assembly file
//! `target/thumbv7em-none-eabihf/release/deps/flc-asm-<hash>.s` file.
//!
//! You need to remove the `CORE_PERIPHERALS` and `DEVICE_PERIPHERALS` symbols to
//! avoid linker conflicts.
#![no_std]
#![deny(
    clippy::missing_safety_doc,
    unsafe_op_in_unsafe_fn,
    clippy::undocumented_unsafe_blocks
)]
#![allow(
    clippy::inline_always,
    reason = "we need the functions to be inlined in this specific case"
)]

use core::{arch::asm, panic::PanicInfo, ptr::read_volatile};

use max78000::{FLC, GCR, ICC0};

/// A macro for ensuring that code never exits, even in cases of fault-injection attacks.
macro_rules! never_exit {
    () => {
        // SAFETY: All branches are to a local label.
        unsafe {
            asm!(
                "2:",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                "b 2b",
                options(noreturn),
            )
        }
    };
}

#[panic_handler]
#[link_section = ".analogsucks"]
fn panic_handler(_: &PanicInfo) -> ! {
    never_exit!()
}

/// A "panic" function that is guaranteed to be in RAM
#[link_section = ".analogsucks"]
fn panic() -> ! {
    never_exit!()
}

/// Flash memory base address.
const FLASH_MEM_BASE: u32 = 0x1000_0000;

/// Flash memory size.
const FLASH_MEM_SIZE: u32 = 0x0008_0000;

/// Flash page size.
const FLASH_PAGE_SIZE: u32 = 0x2000;

struct FlashController<'gcr, 'icc> {
    flc: FLC,
    gcr: &'gcr GCR,
    icc: &'icc ICC0,
}

/// Checks whether the given address range (exclusive) is within flash space, returning `false` if there
/// is an error.
#[must_use]
#[link_section = ".analogsucks"]
const fn check_address_bounds(address_range: core::ops::Range<u32>) -> bool {
    FLASH_MEM_BASE <= address_range.start
        && address_range.start < FLASH_MEM_BASE + FLASH_MEM_SIZE
        && FLASH_MEM_BASE < address_range.end
        && address_range.end <= FLASH_MEM_BASE + FLASH_MEM_SIZE
}

impl FlashController<'_, '_> {
    /// Unlocks memory protection to allow flash operations.
    ///
    /// This MUST be called before any non-read flash controller operation.
    ///
    /// # Safety
    /// - The FLC must be in its ready state after [`Self::wait_until_ready`]
    #[link_section = ".analogsucks"]
    unsafe fn unlock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().unlocked());
    }

    /// Locks memory protection.
    ///
    /// This MUST be called after any non-read flash controller operation.
    ///
    /// # Safety
    /// - The FLC must be in its ready state after [`Self::wait_until_ready`]
    #[link_section = ".analogsucks"]
    unsafe fn lock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().locked());
    }

    /// Sets the `FLC_CLKDIV` register to the system clock's frequency `sys_clk_freq`,
    /// calculated with `freq / div` of the current system clock. It must be a multiple
    /// of 1 MHz.
    ///
    /// This MUST be called before any non-read flash controller operations after
    /// the FLC is ready.
    ///
    /// # Panics
    /// - If the clock is not a multiple of 1 MHz, this function panics.
    ///
    /// # Safety
    /// - The passed argument `sys_clk_freq` must be the current system clock's
    ///   frequency divided by its divider.
    /// - The FLC must be in its ready state after [`Self::wait_until_ready`]
    #[link_section = ".analogsucks"]
    unsafe fn set_clock_divisor(&self, sys_clk_freq: u32) {
        if sys_clk_freq % 1_000_000 != 0 {
            panic()
        }

        let flc_clkdiv = sys_clk_freq / 1_000_000;

        self.flc
            .clkdiv()
            .modify(|_, w| w.clkdiv().variant(flc_clkdiv as u8));
    }

    /// Wait, by busy-looping, until the FLC is ready.
    ///
    /// This MUST be called BEFORE any FLC operation EXCEPT clearing interrupts.
    #[link_section = ".analogsucks"]
    fn wait_until_ready(&self) {
        while !self.flc.ctrl().read().pend().bit_is_clear() {}
    }

    /// Clear any stale errors in the FLC interrupt register.
    ///
    /// This can be called without waiting for the FLC to be ready.
    #[link_section = ".analogsucks"]
    fn clear_interrupts(&self) {
        self.flc.intr().modify(|_, w| w.af().clear_bit());
    }

    /// Prepares the FLC for a write operation, performs the operation, and
    /// cleans up after the operation.
    ///
    /// # Safety
    /// - The argument `sys_clk_freq` must be equal to the current system clock's
    ///   frequency divided by its divider.
    ///
    /// # Panics
    /// - If `sys_clk_freq` is not a multiple of 1 MHz, this function panics.
    #[link_section = ".analogsucks"]
    unsafe fn write_guard<F: Fn()>(&self, sys_clk_freq: u32, operation: F) {
        // Pre-write
        self.wait_until_ready();
        self.disable_icc0();
        self.clear_interrupts();

        // SAFETY: we wait until the FLC is ready above, and the caller must
        // guarantee that `sys_clk_freq` is valid per `[Self::set_clock_divisor]`.
        unsafe {
            self.set_clock_divisor(sys_clk_freq);
        }

        // SAFETY: we wait until the FLC is ready above
        unsafe {
            self.unlock_write_protection();
        }

        operation();

        // Post-write
        self.wait_until_ready();

        // SAFETY: we wait until the FLC is ready above
        unsafe {
            self.lock_write_protection();
        }
        self.flush_icc();
        self.enable_icc0();
    }

    /// Flushes the flash line buffer and arm instruction cache.
    ///
    /// This MUST be called after any write/erase flash controller operations.
    #[link_section = ".analogsucks"]
    fn flush_icc(&self) {
        const PAGE1: u32 = FLASH_MEM_BASE;
        const PAGE2: u32 = FLASH_MEM_BASE + FLASH_PAGE_SIZE;

        self.gcr.sysctrl().modify(|_, w| w.icc0_flush().flush());
        while !self.gcr.sysctrl().read().icc0_flush().bit_is_clear() {}

        // Clear the line fill buffer by reading 2 pages from flash

        // SAFETY: `FLASH_MEM_BASE` points to a valid, aligned word within flash space.
        const {
            assert!(check_address_bounds(PAGE1..PAGE1 + 4));
            assert!(PAGE1 % 4 == 0);
            assert!(check_address_bounds(PAGE2..PAGE2 + 4));
            assert!(PAGE2 % 4 == 0);
        }

        // SAFETY: `FLASH_MEM_BASE` points to a valid, aligned word within flash space as asserted above.
        unsafe { core::hint::black_box(read32(PAGE1 as *const u32)) };
        // SAFETY: `FLASH_MEM_BASE + FLASH_PAGE_SIZE` points to a valid, aligned word within flash space.
        unsafe { core::hint::black_box(read32(PAGE2 as *const u32)) };
    }

    /// Disables instruction cache.
    ///
    /// This MUST be called before any non-read flash controller operations.
    #[link_section = ".analogsucks"]
    fn disable_icc0(&self) {
        self.icc.ctrl().modify(|_, w| w.en().dis());
    }

    /// Enables instruction cache.
    ///
    /// This MUST be called after any non-read flash controller operations.
    #[link_section = ".analogsucks"]
    fn enable_icc0(&self) {
        // ensure the cache is invalidated when enabled
        self.disable_icc0();

        self.icc.invalidate().modify(|_, w| w.invalid().variant(1));
        while !self.icc.ctrl().read().rdy().bit_is_set() {}

        self.icc.ctrl().modify(|_, w| w.en().en());
        while !self.icc.ctrl().read().rdy().bit_is_set() {}
    }

    /// Writes 128 bits (16 bytes) of data to flash.
    /// Address must be 128-bit aligned.
    ///
    /// # Safety
    ///
    /// - The argument `sys_clk_freq` must be equal to the current system clock's
    ///   frequency divided by its divider.
    /// - Writes must not corrupt potentially executable instructions of the program.
    /// - Callers must ensure that the following condition is met:
    ///     * If `address` points to a portion of the program's instructions, `data` must
    ///       contain valid instructions that does not introduce undefined behavior.
    ///
    /// It is very difficult to define what would cause undefined behavior when
    /// modifying program instructions. This would almost certainly result
    /// in unwanted and likely undefined behavior. Do so at your own risk.
    ///
    ///
    /// # Panics
    ///
    /// If any of the following conditions are not met, this function panics:
    ///
    /// - `sys_clk_freq` must be a multiple of 1 MHz
    /// - `address` must point to a word contained in flash space
    /// - `address` must be aligned to 128 bits
    #[link_section = ".analogsucks"]
    unsafe fn write128(&self, address: u32, data: &[u32; 4], sys_clk_freq: u32) {
        if !check_address_bounds(address..address + 16) {
            panic();
        }
        #[allow(
            clippy::cast_possible_truncation,
            reason = "the target pointer width is 32, so this will not truncate"
        )]
        if address % size_of::<[u32; 4]>() as u32 != 0 {
            panic();
        }

        // SAFETY: the caller must guarantee that `sys_clk_freq` is valid per this function's
        // safety comment.
        unsafe {
            self.write_guard(sys_clk_freq, || {
                self.flc.addr().modify(|_, w| w.addr().variant(address));
                self.flc.data(0).modify(|_, w| w.data().variant(data[0]));
                self.flc.data(1).modify(|_, w| w.data().variant(data[1]));
                self.flc.data(2).modify(|_, w| w.data().variant(data[2]));
                self.flc.data(3).modify(|_, w| w.data().variant(data[3]));

                self.flc.ctrl().modify(|_, w| w.wr().set_bit());

                // Wait until write completes
                while !self.flc.ctrl().read().wr().is_complete() {}
            });
        }
    }

    /// Erases a page of flash. `address[12:0]` is ignored to ensure the address
    /// is page-aligned.
    ///
    /// # Safety
    ///
    /// - The argument `sys_clk_freq` must be equal to the current system clock's
    ///   frequency divided by its divider.
    /// - Erases must not corrupt potentially executable instructions of the program.
    /// - `address` must be in a valid flash page
    ///
    /// # Panics
    /// - If `sys_clk_freq` is not a multiple of 1 MHz, this function panics.
    /// - This function also panics when the `address` does not point inside of a page
    ///   contained in flash space.
    #[link_section = ".analogsucks"]
    unsafe fn page_erase(&self, address: u32, sys_clk_freq: u32) {
        #[allow(
            clippy::range_plus_one,
            reason = "the caller takes a Range struct, not an `impl RangeBounds`"
        )]
        if !check_address_bounds(address..address + 1) {
            panic()
        }
        // SAFETY: the caller must guarantee that `sys_clk_freq` is valid per this function's
        // safety comment.
        unsafe {
            self.write_guard(sys_clk_freq, || {
                self.flc.addr().modify(|_, w| w.addr().variant(address));

                self.flc.ctrl().modify(|_, w| w.erase_code().erase_page());
                self.flc.ctrl().modify(|_, w| w.pge().set_bit());
            });
        }
    }
}

/// Reads a little-endian `u32` from flash memory.
///
/// Panics if any of the following preconditions are not true:
/// - `address` must be 32-bit aligned.
/// - `address` must point to a valid location in flash memory (`0x1000_0000..=0x1007_ffff`).
///
/// # Safety
///
/// This is a pointer read to flash space, it is the caller's responsibility to ensure
/// that the pointer points to the correct value.
#[export_name = "flc_read32_primitive"]
#[link_section = ".analogsucks"]
pub unsafe extern "C" fn read32(address: *const u32) -> u32 {
    if !address.is_aligned() {
        panic();
    }
    if !check_address_bounds(address as u32..(address as u32 + 4)) {
        panic();
    }
    // SAFETY: the caller must guarantee that `address` is aligned and is within
    // flash memory.
    unsafe { read_volatile(address) }
}

/// Writes a little-endian 128-bit flash word into flash memory.
///
/// # Safety
///
/// - The caller must hold a shared reference to the [`FLC`], [`ICC0`], and [`GCR`] registers.
/// - The flash word at `address` must be in the *erased* state (with [`page_erase`]).
/// - `data` must point to an array of four `u32`s.
/// - `sys_clk_freq` must be equal to `freq / div` where `freq` is the frequency of
///   the current system clock, and `div` is the divider of the system clock.
/// - If `address` writes to an address in the currently-running program's instruction space,
///   it must be valid instructions.
///
/// # Panics
///
/// Panics if any of the following preconditions are not true:
/// - `address` must be 128-bit aligned.
/// - The entire flash word at address (bytes `address..address + 16`) must be within
///   the flash memory (`0x1000_0000..=0x1007_ffff`).
/// - `sys_clk_freq` must be divisible by one million (`1_000_000`).
#[export_name = "flc_write128_primitive"]
#[link_section = ".analogsucks"]
pub unsafe extern "C" fn write128(address: *mut [u32; 4], data: *const u32, sys_clk_freq: u32) {
    // SAFETY: the caller must hold a valid reference to these registers during this call.
    let flc = unsafe {
        FlashController {
            flc: FLC::steal(),
            icc: &ICC0::steal(),
            gcr: &GCR::steal(),
        }
    };

    // SAFETY: the caller must ensure that `data` points to a valid array of four `u32`s.
    let data = unsafe { &*data.cast() };

    // SAFETY:
    // - the caller must guarantee that the address is aligned and the word is within flash space
    // - the caller must guarantee that the word is in the erased state
    // - the caller must ensure that `sys_clk_freq` is correctly calculated per this function's
    //   safety comment
    // - the caller must ensure that, if it is overwriting instructions, the new instructions are valid
    unsafe {
        flc.write128(address as u32, data, sys_clk_freq);
    }
}

/// Erases the page at the given address in flash memory.
///
/// # Safety
///
/// - The caller must hold a shared reference to the [`FLC`], [`ICC0`], and [`GCR`] registers.
/// - `address` must point to a valid page within flash memory (`0x1000_0000..=0x1007_ffff`).
/// - `sys_clk_freq` must be equal to `freq / div` where `freq` is the frequency of
///   the current system clock, and `div` is the divider of the system clock.
/// - `sys_clk_freq` must be divisible by one million (`1_000_000`).
/// - If `address` erases a page in the currently-running program's instruction space,
///   it must be rewritten with [`write128`] before the program reaches those instructions.
///
/// # Panics
///
/// Panics if any of the following preconditions are not true:
/// - `address` must point to within a valid page in flash space (`0x1000_000..=0x1007_ffff`)
/// - `sys_clk_freq` must be divisible by one million (`1_000_000`).
#[export_name = "flc_page_erase_primitive"]
#[link_section = ".analogsucks"]
pub unsafe extern "C" fn page_erase(address: *mut u8, sys_clk_freq: u32) {
    // SAFETY: the caller must hold a valid reference to these registers during this call.
    let flc = unsafe {
        FlashController {
            flc: FLC::steal(),
            icc: &ICC0::steal(),
            gcr: &GCR::steal(),
        }
    };

    // SAFETY:
    // - the caller must provide a valid address.
    // - the caller must ensure that sys_clk_freq is calculated correctly per this function's
    //   safety comment.
    // - the caller must guarantee that the program won't execute erased instructions in this page.
    unsafe {
        flc.page_erase(address as u32, sys_clk_freq);
    }
}
