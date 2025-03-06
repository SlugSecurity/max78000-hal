//! A HAL for the Analog Devices MAX78000.
//!
//! # Runtime
//!
//! If the `rt` feature is enabled, this crate uses the runtime of the [`cortex_m_rt`]
//! crate.  Note that the HAL uses the [`pre_init`] hook internally, and it is not possible
//! for users of the hal to specify their own `pre_init` routine.
//!
//! # Flash Controller
//!
//! If the `flc-ram` feature is enabled, this crate will expose the [`FlashController`]
//! peripheral.  Certain routines for flash operations need to be located in RAM (instead
//! of flash memory), so users of this feature will need to add the following section to
//! their linker script `link.x`:
//!
//! ```ld
//! .analogsucks : ALIGN(4)
//! {
//!   . = ALIGN(4);
//!   __sanalogsucks = .;
//!   *(.analogsucks .analogsucks.*);
//! } > ANALOGSUCKS
//!   . = ALIGN(4);
//!   __eanalogsucks = .;
//! } > ANALOGSUCKS AT>FLASH
//!
//! __sianalogsucks = LOADADDR(.analogsucks);
//! ```
//!
//! where the `ANALOGSUCKS` is a memory section in RAM defined in `memory.x`.
//!
//! [`pre_init`]: cortex_m_rt::pre_init
//! [`FlashController`]: peripherals::flash_controller::FlashController

#![warn(missing_docs)]
#![no_std]

pub use max78000;

#[cfg(feature = "rt")]
pub use self::max78000::Interrupt as interrupt;

#[cfg(feature = "rt")]
pub use cortex_m_rt::{interrupt, pre_init};

pub mod communication;
pub mod peripherals;

/// `__pre_init` symbol, ran before initializing memory in [`cortex-m-rt`].  See
/// [`pre_init`] for more.
///
/// # Safety
///
/// - Only assembly is allowed, because RAM has not been initialized, so any Rust
///   code that touches memory is undefined behavior.
///
/// [`pre_init`]: cortex_m_rt::pre_init
#[cfg(feature = "rt")]
#[pre_init]
unsafe fn pre_init() {
    // load the .analogsucks section into memory
    #[cfg(feature = "flc-ram")]
    core::arch::asm! {
        "ldr {0}, =__sanalogsucks
         ldr {1}, =__eanalogsucks
         ldr {2}, =__sianalogsucks
         0:
         cmp {1}, {0}
         beq 1f
         ldm {2}!, {{{3}}}
         stm {0}!, {{{3}}}
         b 0b
         1:",
         out(reg) _,
         out(reg) _,
         out(reg) _,
         out(reg) _,
    }
}
