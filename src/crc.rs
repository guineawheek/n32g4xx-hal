/*
CRC registers.

*/

use crate::pac::{Crc,Rcc};
use crate::rcc::{Enable,Reset};

pub trait CrcExt {
    fn constrain(self) -> CrcEngine;
}

pub struct CrcEngine {
    pub(crate) regs: Crc,
}

pub struct Crc16Engine {
    pub(crate) regs: Crc,
}

pub struct Crc32Engine {
    pub(crate) regs: Crc,
}

pub struct Crc32Stream {
    engine: Crc32Engine
}

#[derive(Clone, Copy)]
pub struct Crc16State {
    pub value: u16,
    pub endianness: CrcEndianness
}

#[derive(Clone, Copy)]
pub enum CrcEndianness {
    StartFromMsb,
    StartFromLsb
}

impl Crc16State {
    pub fn new(endianness: CrcEndianness) -> Self {
        Self {
            value: 0,
            endianness: endianness
        }
    }
}

impl CrcExt for Crc {
    fn constrain(self) -> CrcEngine {
        let rcc = {unsafe {&(*Rcc::ptr())}};
        Crc::enable(rcc);
        Crc::reset(rcc);
        CrcEngine{regs: self}
    }
}


impl CrcEngine {
    /// Splits this CrcEngine into a Crc16 and Crc32 engine that can act independently.
    /// two of them.
    pub fn split(self) -> (Crc16Engine, Crc32Engine) {
        let crc = {unsafe {Crc::steal()}};
        (
            Crc16Engine {regs: crc},
            Crc32Engine {regs: self.regs}
        )
    }

    /// Computes a CRC32 on the given u32 slice.
    /// This produces a big-endian CRC.
    pub fn crc32(&mut self, data: &[u32]) -> u32 {
        self.regs.crc32ctrl().write(|w| w.reset().set_bit());
        for word in data {
            self.regs.crc32dat().write(|w| unsafe {w.crc32dat().bits(*word)});
        }
        self.regs.crc32dat().read().crc32dat().bits()
    }

    /// Compute a CRC16 on the given u16 slice.
    /// iv: initial value
    /// endianness: which way to consume the data
    /// data: data to consume
    pub fn crc16(&mut self, state: Crc16State, data: &[u8]) -> Crc16State {
        match state.endianness {
            CrcEndianness::StartFromMsb => self.regs.crc16ctrl().write(|w| w.endhl().clear_bit()),
            CrcEndianness::StartFromLsb =>  self.regs.crc16ctrl().write(|w| w.endhl().set_bit())
        };
        self.regs.crc16d().write(|w| unsafe {w.crc16d().bits(state.value)});
        for word in data {
            self.regs.crc16dat().write(|w| unsafe {w.crc16dat().bits(*word)});
        }        
        Crc16State {
            value: self.regs.crc16d().read().crc16d().bits(),
            endianness: state.endianness
        }
    }
}

impl Crc32Engine {
    /// stream a crc32 so you don't have to compute it all at once
    pub fn stream(self) -> Crc32Stream {
        self.regs.crc32ctrl().write(|w| w.reset().set_bit());
        Crc32Stream { engine: self }
    }

    /// Computes a CRC32 on the given u32 slice.
    /// This produces a big-endian CRC.
    pub fn crc32(&mut self, data: &[u32]) -> u32 {
        self.regs.crc32ctrl().write(|w| w.reset().set_bit());
        for word in data {
            self.regs.crc32dat().write(|w| unsafe {w.crc32dat().bits(*word)});
        }
        self.regs.crc32dat().read().crc32dat().bits()
    }

}

impl Crc32Stream {
    /// update the crc32 hardware register with new data
    pub fn update(&mut self, data: &[u32]) {
        for word in data {
            self.engine.regs.crc32dat().write(|w| unsafe {w.crc32dat().bits(*word)});
        }
    }
    /// read the current crc32 hash value
    pub fn value(&self) -> u32 {
        self.engine.regs.crc32dat().read().crc32dat().bits()
    }
    /// release the engine for use elsewhere
    pub fn finalize(self) -> Crc32Engine {
        self.engine
    }
}

impl Crc16Engine {
    /// Compute a CRC16 on the given u16 slice.
    /// States allow one to multiplex the periph between different subsystems.
    /// state: a Crc16State with an iv and endianness
    /// data: data to consume
    pub fn crc16(&mut self, state: Crc16State, data: &[u8]) -> Crc16State {
        match state.endianness {
            CrcEndianness::StartFromMsb => self.regs.crc16ctrl().write(|w| w.endhl().clear_bit()),
            CrcEndianness::StartFromLsb =>  self.regs.crc16ctrl().write(|w| w.endhl().set_bit())
        };
        self.regs.crc16d().write(|w| unsafe {w.crc16d().bits(state.value)});
        for word in data {
            self.regs.crc16dat().write(|w| unsafe {w.crc16dat().bits(*word)});
        }        
        Crc16State {
            value: self.regs.crc16d().read().crc16d().bits(),
            endianness: state.endianness
        }
    }
}