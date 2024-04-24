use crate::pac::Sac;
use crate::rcc::{Enable,Reset};
pub struct CryptoEngine {
    pub(crate) regs: Sac,
}

impl CryptoEngine {
    pub fn new(regs: Sac) -> Self {
        // Sac::enable();
        Self { regs }
    }
}