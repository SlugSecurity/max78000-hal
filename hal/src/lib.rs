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
//! [`FlashController`]: peripherals::FlashController

#![warn(missing_docs)]
#![no_std]

pub use max78000;

#[cfg(feature = "rt")]
pub use self::max78000::Interrupt as interrupt;

#[cfg(feature = "rt")]
pub use cortex_m_rt::{interrupt, pre_init};

pub mod communication;
pub mod peripherals;

#[cfg(feature = "rt")]
#[pre_init]
unsafe fn pre_init() {
    // load the .analogsucks section into memory
    #[cfg(feature = "flc-ram")]
    core::arch::asm! {
        "ldr r0, =__sanalogsucks
         ldr r1, =__eanalogsucks
         ldr r2, =__sianalogsucks
         0:
         cmp r1, r0
         beq 1f
         ldm r2!, {{r3}}
         stm r0!, {{r3}}
         b 0b
         1:"
    }
}
