//! A HAL for the Analog Devices MAX78000.

#![warn(missing_docs)]
#![no_std]

pub use max78000;

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;

pub mod peripherals;
