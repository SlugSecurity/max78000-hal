//! Contains types used to enumerate the common GPIO ports.

use max78000::{GPIO0, GPIO1, GPIO2};
use sealed::sealed;

/// Trait to disambiguate between different GPIO ports in the common
/// GPIO API. GPIO3 is not implemented with this because it's
/// is accessed in a different way.
///
/// # Note:
///
/// This trait is sealed and cannot be implemented outside this crate.
#[sealed]
pub trait GpioPortNum {
    /// The type of the peripheral associated with the port number.
    /// eg: GPIO0, GPIO1, GPIO2
    type Peripheral;

    /// Gets the associated port number as an integer.
    /// eg: GpioZero would return 0.
    fn get_port_num(&self) -> usize;
}

macro_rules! generate_gpio_port_num {
    ($port_num_type:ident, $port_periph:ty, $port_num:literal) => {
        #[doc = core::concat!("Struct representing GPIO port number ", $port_num)]
        pub struct $port_num_type;

        #[sealed]

        impl GpioPortNum for $port_num_type {
            type Peripheral = $port_periph;

            fn get_port_num(&self) -> usize {
                $port_num
            }
        }
    };
}

generate_gpio_port_num!(GpioZero, GPIO0, 0);
generate_gpio_port_num!(GpioOne, GPIO1, 1);
generate_gpio_port_num!(GpioTwo, GPIO2, 2);
