//! A HAL for the Analog Devices MAX78000.

#![warn(missing_docs)]
#![no_std]

pub use max78000;

#[cfg(feature = "rt")]
pub use max78000::interrupt;

pub mod critical_section;
pub mod peripherals;

// TODO: Define a critical section impl with synchronization module to synchronize betw. ARM and RISC-V cores. Feature gate it so it's not restrictive.
