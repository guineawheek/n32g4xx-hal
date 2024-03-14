//! # Controller Area Network (CAN) Interface
//!
//! ## Alternate function remapping
//!
//! TX: Alternate Push-Pull Output
//! RX: Input
//!
//! ### CAN1
//!
//! | Function | NoRemap | Remap |
//! |----------|---------|-------|
//! | TX       | PA12    | PB9   |
//! | RX       | PA11    | PB8   |
//!
//! ### CAN2
//!
//! | Function | NoRemap | Remap |
//! |----------|---------|-------|
//! | TX       | PB6     | PB13  |
//! | RX       | PB5     | PB12  |

use crate::gpio::{self, Alternate, Input};
use crate::pac::{self, RCC,AFIO};

pub trait Pins: crate::Sealed {
    type Instance;
    fn remap(afio: &mut AFIO);
}

impl<INMODE, OUTMODE> crate::Sealed
    for (gpio::PA12<Alternate<OUTMODE>>, gpio::PA11<Input<INMODE>>)
{
}
impl<INMODE, OUTMODE> Pins for (gpio::PA12<Alternate<OUTMODE>>, gpio::PA11<Input<INMODE>>) {
    type Instance = pac::CAN1;

    fn remap(afio: &mut AFIO) {
        afio.rmp_cfg().modify(|_, w| unsafe { w.can1_rmp().bits(0) });
    }
}

impl<INMODE, OUTMODE> crate::Sealed for (gpio::PB9<Alternate<OUTMODE>>, gpio::PB8<Input<INMODE>>) {}
impl<INMODE, OUTMODE> Pins for (gpio::PB9<Alternate<OUTMODE>>, gpio::PB8<Input<INMODE>>) {
    type Instance = pac::CAN1;

    fn remap(afio: &mut AFIO) {
        afio.rmp_cfg().modify(|_, w| unsafe { w.can1_rmp().bits(0b10) });
    }
}

impl<INMODE, OUTMODE> crate::Sealed
    for (gpio::PB13<Alternate<OUTMODE>>, gpio::PB12<Input<INMODE>>)
{
}

impl<INMODE, OUTMODE> Pins for (gpio::PB13<Alternate<OUTMODE>>, gpio::PB12<Input<INMODE>>) {
    type Instance = pac::CAN2;

    fn remap(afio: &mut AFIO) {
        afio.rmp_cfg3().modify(|_, w| unsafe { w.can2_rmp().bits(0) });
    }
}

impl<INMODE, OUTMODE> crate::Sealed for (gpio::PB6<Alternate<OUTMODE>>, gpio::PB5<Input<INMODE>>) {}
impl<INMODE, OUTMODE> Pins for (gpio::PB6<Alternate<OUTMODE>>, gpio::PB5<Input<INMODE>>) {
    type Instance = pac::CAN2;

    fn remap(afio: &mut AFIO) {
        afio.rmp_cfg3().modify(|_, w| unsafe { w.can2_rmp().bits(0b01) });
    }
}

/// Interface to the CAN peripheral.
pub struct Can<Instance> {
    _peripheral: Instance,
}

impl<Instance> Can<Instance>
where
    Instance: crate::rcc::Enable,
{
     pub fn new(can: Instance) -> Can<Instance> {
        let rcc = unsafe { &(*RCC::ptr()) };
        Instance::enable(rcc);

        Can { _peripheral: can }
    }

    /// Routes CAN TX signals and RX signals to pins.
    pub fn assign_pins<P>(&self, _pins: P, afio: &mut AFIO)
    where
        P: Pins<Instance = Instance>,
    {
        P::remap(afio);
    }
}

unsafe impl bxcan::Instance for Can<pac::CAN1> {
    const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN1::ptr() as *mut bxcan::RegisterBlock;
}

unsafe impl bxcan::Instance for Can<pac::CAN2> {
    const REGISTERS: *mut bxcan::RegisterBlock = pac::CAN2::ptr() as *mut bxcan::RegisterBlock;
}

unsafe impl bxcan::FilterOwner for Can<pac::CAN1> {
    const NUM_FILTER_BANKS: u8 = 14;
}

unsafe impl bxcan::FilterOwner for Can<pac::CAN2> {
    const NUM_FILTER_BANKS: u8 = 14;
}
