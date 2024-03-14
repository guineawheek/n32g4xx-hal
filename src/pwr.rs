use crate::pac::{PWR,RCC};
use crate::rcc::{Enable,Reset};
pub trait PwrExt {
    fn constrain(self) -> PWR;
}
#[cfg(any(feature="n32g451",feature="n32g452",feature="n32g455",feature="n32g457",feature="n32g4fr"))]
impl PwrExt for PWR {
    fn constrain(self) -> PWR {
        let rcc = unsafe { &(*RCC::ptr()) };
        PWR::enable(rcc);
        PWR::reset(rcc);
        self
    }
}