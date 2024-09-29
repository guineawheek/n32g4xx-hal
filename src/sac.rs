use crate::pac::Sac;
pub struct CryptoEngine {
    pub(crate) _regs: Sac,
}

impl CryptoEngine {
    pub fn new(_regs: Sac) -> Self {
        // Sac::enable();
        Self { _regs }
    }
}