//! Multi device hardware abstraction on top of the peripheral access API for the Nations Technologies N32G4 series microcontrollers.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
#![no_std]
#![allow(non_camel_case_types)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(negative_impls)]
#![feature(min_specialization)]
#![feature(macro_metavar_expr)]
#![feature(more_qualified_paths)]
use enumflags2::{BitFlag, BitFlags};

pub use embedded_hal as hal;
pub use embedded_hal_02 as hal_02;

pub use nb;
pub use nb::block;

#[cfg(feature = "n32g401")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g401 peripherals.
pub use n32g4::n32g401 as pac;

#[cfg(feature = "n32g432")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g432 peripherals.
pub use n32g4::n32g432 as pac;

#[cfg(feature = "n32g435")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g435 peripherals.
pub use n32g4::n32g435 as pac;

#[cfg(feature = "n32g451")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g451 peripherals.
pub use n32g4::n32g451 as pac;

#[cfg(feature = "n32g452")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g452 peripherals.
pub use n32g4::n32g452 as pac;

#[cfg(feature = "n32g455")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g455 peripherals.
pub use n32g4::n32g455 as pac;

#[cfg(feature = "n32g457")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g457 peripherals.
pub use n32g4::n32g457 as pac;

#[cfg(feature = "n32g4fr")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the n32g4fr peripherals.
pub use n32g4::n32g4fr as pac;

pub mod adc;
pub mod afio;
pub mod bb;
#[cfg(any(feature = "n32g451",feature = "n32g452",feature = "n32g455",feature = "n32g457",feature = "n32g4fr"))]
pub mod bkp;
pub mod can;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod rcc;
pub mod time;
pub mod timer;
pub mod prelude;
pub mod pwr;
pub mod usb;
mod sealed {
pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}

pub trait ReadFlags {
    /// Enum of bit flags
    type Flag: BitFlag;

    /// Get all interrupts flags a once.
    fn flags(&self) -> BitFlags<Self::Flag>;
}

pub trait ClearFlags {
    /// Enum of manually clearable flags
    type Flag: BitFlag;

    /// Clear interrupts flags with `Self::Flags`s
    ///
    /// If event flag is not cleared, it will immediately retrigger interrupt
    /// after interrupt handler has finished.
    fn clear_flags(&mut self, flags: impl Into<BitFlags<Self::Flag>>);

    /// Clears all interrupts flags
    #[inline(always)]
    fn clear_all_flags(&mut self) {
        self.clear_flags(BitFlags::ALL)
    }
}

pub trait Listen {
    /// Enum of bit flags associated with events
    type Event: BitFlag;

    /// Start listening for `Event`s
    ///
    /// Note, you will also have to enable the appropriate interrupt in the NVIC to start
    /// receiving events.
    fn listen(&mut self, event: impl Into<BitFlags<Self::Event>>);

    /// Start listening for `Event`s, stop all other
    ///
    /// Note, you will also have to enable the appropriate interrupt in the NVIC to start
    /// receiving events.
    fn listen_only(&mut self, event: impl Into<BitFlags<Self::Event>>);

    /// Stop listening for `Event`s
    fn unlisten(&mut self, event: impl Into<BitFlags<Self::Event>>);

    /// Start listening all `Event`s
    #[inline(always)]
    fn listen_all(&mut self) {
        self.listen(BitFlags::ALL)
    }

    /// Stop listening all `Event`s
    #[inline(always)]
    fn unlisten_all(&mut self) {
        self.unlisten(BitFlags::ALL)
    }
}

