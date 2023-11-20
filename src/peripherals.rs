//! Peripheral drivers for the MAX78000.

use core::ops::Deref;

use critical_section::Mutex;
use heapless::{pool::singleton::arc::Pool, Arc};

// Embedded HAL peripherals.
pub mod adc;
pub mod delay;
pub mod digital;
pub mod rng;
pub mod serial;
pub mod timer;
pub mod watchdog;

// Non embedded HAL peripherals.
pub mod aes;
pub mod bit_banding;
pub mod bootloader;
pub mod crc;
pub mod ecc;
pub mod flash_controller;
pub mod oscillator;
pub mod power;
pub mod raw;
pub mod rtc;
pub mod synchronization;

// TODO: Create peripheral manager and traits.
// TODO: Create a new pool for each peripheral? Use some attribute macro to do this?

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Peripheral errors.
pub enum PeripheralError {
    /// Peripheral is not enabled and operation requires it to be enabled.
    NotEnabled,
    /// Peripheral is not disabled and operation requires it to be disabled.
    NotDisabled,
    /// Peripheral failed to initialize.
    InitFailure,
    /// Peripheral is in an illegal state.
    IllegalState,
}

pub trait Peripheral {
    fn enable(&mut self) -> Result<(), PeripheralError>;
    fn disable(&mut self) -> Result<(), PeripheralError>;
    fn is_enabled(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct PeripheralWrapper<Ph: Peripheral, Po: Pool<Data = Mutex<Ph>>>(Arc<Po>);

impl<Ph: Peripheral, Po: Pool<Data = Mutex<Ph>>> PeripheralWrapper<Ph, Po> {
    pub(crate) fn new(peripheral: Arc<Po>) -> Self {
        Self { 0: peripheral }
    }

    // TODO: Create a with method to abstract away acquiring a critical section lock, have user pass in closure with &mut Ph
    // as the arg? Need to expose the peripheral itself to allow access to the peripheral's methods.

    /*
    pub fn enable(&self) -> Result<(), PeripheralError> {
        self.lock(|peripheral| peripheral.enable())
    }*/
}

impl<Ph: Peripheral, Po: Pool<Data = Mutex<Ph>>> Deref for PeripheralWrapper<Ph, Po> {
    type Target = Po::Data;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
pub trait PeripheralManager {
    fn get_peripheral<P>(&mut self) -> Result<PeripheralWrapper<P>, PeripheralError>;
    fn get_or_enable_peripheral<P>(&mut self) -> Result<PeripheralWrapper<P>, PeripheralError>;
}*/

// Array of boxed dyn peripherals.
//
// TODO: Impl deref trait for peripheral wrapper, target is mutex.
