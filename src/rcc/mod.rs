//! Clock configuration.
//!
//! This module provides functionality to configure the Rcc to generate the requested clocks.
//!
//! # Example
//!
//! ```
//! let dp = pac::Peripherals::take().unwrap();
//! let rcc = dp.Rcc.constrain();
//! let clocks = rcc
//!     .cfgr
//!     .use_hse(8.MHz())
//!     .sysclk(168.MHz())
//!     .pclk1(24.MHz())
//!     .i2s_clk(86.MHz())
//!     .require_pll48clk()
//!     .freeze();
//!     // Test that the I2S clock is suitable for 48000kHz audio.
//!     assert!(clocks.i2s_clk().unwrap() == 48.MHz().into());
//! ```
//!
//! # Limitations
//!
//! Unlike the clock configuration tool provided by ST, the code does not extensively search all
//! possible configurations. Instead, it often relies on an iterative approach to reduce
//! computational complexity. On most MCUs the code will first generate a configuration for the 48
//! MHz clock and the system clock without taking other requested clocks into account, even if the
//! accuracy of these clocks is affected. **If you specific accuracy requirements, you should
//! always check the resulting frequencies!**
//!
//! Whereas the hardware often supports flexible clock source selection and many clocks can be
//! sourced from multiple PLLs, the code implements a fixed mapping between PLLs and clocks. The 48
//! MHz clock is always generated by the main PLL, the I2S clocks are always generated by the I2S
//! PLL (unless a matching external clock input is provided), and similarly the SAI clocks are
//! always generated by the SAI PLL. It is therefore not possible to, for example, specify two
//! different I2S frequencies unless you also provide a matching I2S_CKIN signal for one of them.
//!
//! Some MCUs have limited clock generation hardware and do not provide either I2S or SAI PLLs even
//! though I2S or SAI are available. On the STM32F410, the I2S clock is generated by the main PLL,
//! and on the STM32F413/423 SAI clocks are generated by the I2S PLL. On these MCUs, the actual
//! frequencies may substantially deviate from the requested frequencies.

use crate::pac::rcc::cfg::{Ahbpres,Sclksw, Apb1pres};
use crate::pac::{self, rcc, Rcc};

use fugit::HertzU32 as Hertz;
use fugit::RateExtU32;

use pll::MainPll;

mod pll;

mod enable;
use crate::pac::rcc::RegisterBlock as RccRB;

/// Bus associated to peripheral
pub trait RccBus: crate::Sealed {
    /// Bus type;
    type Bus;
}

/// Enable/disable peripheral
#[allow(clippy::missing_safety_doc)]
pub trait Enable: RccBus {
    /// Enables peripheral
    fn enable(rcc: &RccRB);

    /// Disables peripheral
    fn disable(rcc: &RccRB);

    /// Check if peripheral enabled
    fn is_enabled() -> bool;

    /// Check if peripheral disabled
    #[inline]
    fn is_disabled() -> bool {
        !Self::is_enabled()
    }

    /// # Safety
    ///
    /// Enables peripheral. Takes access to Rcc internally
    unsafe fn enable_unchecked() {
        let rcc = &*pac::Rcc::ptr();
        Self::enable(rcc);
    }

    /// # Safety
    ///
    /// Disables peripheral. Takes access to Rcc internally
    unsafe fn disable_unchecked() {
        let rcc = pac::Rcc::ptr();
        Self::disable(&*rcc);
    }
}

/// Low power enable/disable peripheral
#[allow(clippy::missing_safety_doc)]
pub trait LPEnable: RccBus {
    /// Enables peripheral in low power mode
    fn enable_in_low_power(rcc: &RccRB);

    /// Disables peripheral in low power mode
    fn disable_in_low_power(rcc: &RccRB);

    /// Check if peripheral enabled in low power mode
    fn is_enabled_in_low_power() -> bool;

    /// Check if peripheral disabled in low power mode
    #[inline]
    fn is_disabled_in_low_power() -> bool {
        !Self::is_enabled_in_low_power()
    }

    /// # Safety
    ///
    /// Enables peripheral in low power mode. Takes access to Rcc internally
    unsafe fn enable_in_low_power_unchecked() {
        let rcc = pac::Rcc::ptr();
        Self::enable_in_low_power(&*rcc);
    }

    /// # Safety
    ///
    /// Disables peripheral in low power mode. Takes access to Rcc internally
    unsafe fn disable_in_low_power_unchecked() {
        let rcc = pac::Rcc::ptr();
        Self::disable_in_low_power(&*rcc);
    }
}

/// Reset peripheral
#[allow(clippy::missing_safety_doc)]
pub trait Reset: RccBus {
    /// Resets peripheral
    fn reset(rcc: &RccRB);

    /// # Safety
    ///
    /// Resets peripheral. Takes access to Rcc internally
    unsafe fn reset_unchecked() {
        let rcc = pac::Rcc::ptr();
        Self::reset(&*rcc);
    }
}

/// Extension trait that constrains the `Rcc` peripheral
pub trait RccExt {
    /// Constrains the `Rcc` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> RccCon;
}

/// Frequency on bus that peripheral is connected in
pub trait BusClock {
    /// Calculates frequency depending on `Clock` state
    fn clock(clocks: &Clocks) -> Hertz;
}

/// Frequency on bus that timer is connected in
pub trait BusTimerClock {
    /// Calculates base frequency of timer depending on `Clock` state
    fn timer_clock(clocks: &Clocks) -> Hertz;
}

impl<T> BusClock for T
where
    T: RccBus,
    T::Bus: BusClock,
{
    fn clock(clocks: &Clocks) -> Hertz {
        T::Bus::clock(clocks)
    }
}

impl<T> BusTimerClock for T
where
    T: RccBus,
    T::Bus: BusTimerClock,
{
    default fn timer_clock(clocks: &Clocks) -> Hertz {
        T::Bus::timer_clock(clocks)
    }
}

impl BusTimerClock for crate::pac::Tim1
where
{
    fn timer_clock(clocks: &Clocks) -> Hertz {
        APB2::timer_clock(clocks) * 2
    }
}

impl BusTimerClock for crate::pac::Tim8
where
{
    fn timer_clock(clocks: &Clocks) -> Hertz {
        APB2::timer_clock(clocks) * 2
    }
}


impl BusTimerClock for APB1 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.pclk1()
    }
}

impl BusTimerClock for APB2 {
    fn timer_clock(clocks: &Clocks) -> Hertz {
        clocks.pclk2()
    }
}
macro_rules! bus_struct {
    ($( $(#[$attr:meta])* $busX:ident => ($EN:ident, $en:ident, $RST:ident, $rst:ident, $doc:literal),)+) => {
        $(
            $(#[$attr])*
            #[doc = $doc]
            pub struct $busX {
                _0: (),
            }

            $(#[$attr])*
            impl $busX {
                pub(crate) fn pclken(rcc: &RccRB) -> &rcc::$EN {
                    &rcc.$en()
                }

                pub(crate) fn prst(rcc: &RccRB) -> &rcc::$RST {
                    &rcc.$rst()
                }
            }
        )+
    };
}

bus_struct! {
    APB1 => (Apb1pclken, apb1pclken, Apb1prst, apb1prst, "Advanced Peripheral Bus 1 (APB1) registers"),
    APB2 => (Apb2pclken, apb2pclken, Apb2prst, apb2prst, "Advanced Peripheral Bus 2 (APB2) registers"),
    AHB => (Ahbpclken, ahbpclken,  Ahbprst, ahbprst, "Advanced High-performance Bus (AHB) registers"),
}



impl BusClock for AHB {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.hclk
    }
}

impl BusClock for APB1 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk1
    }
}

impl BusClock for APB2 {
    fn clock(clocks: &Clocks) -> Hertz {
        clocks.pclk2
    }
}


impl RccExt for Rcc {
    fn constrain(self) -> RccCon {
        RccCon {
            cfgr: CFGR {
                hse: None,
                hse_bypass: false,
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
            },
        }
    }
}

/// Constrained Rcc peripheral
pub struct RccCon {
    pub cfgr: CFGR,
}

/// Built-in high speed clock frequency
pub const HSI: u32 = 16_000_000; // Hz

/// Minimum system clock frequency
pub const SYSCLK_MIN: u32 = 32_000_000;

#[cfg(feature = "n32g401")]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 72_000_000;

#[cfg(feature = "n32g430")]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 128_000_000;

#[cfg(any(feature = "n32g432",feature="n32g435"))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 108_000_000;

#[cfg(any(feature = "n32g451",feature="n32g452",feature="n32g455",feature="n32g457",feature="n32g4fr"))]
/// Maximum system clock frequency
pub const SYSCLK_MAX: u32 = 144_000_000;


/// Maximum APB2 peripheral clock frequency
pub const PCLK2_MAX: u32 = SYSCLK_MAX / 2;

/// Maximum APB1 peripheral clock frequency
pub const PCLK1_MAX: u32 = SYSCLK_MAX / 4;

pub struct CFGR {
    hse: Option<u32>,
    hse_bypass: bool,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

impl CFGR {
    /// Uses HSE (external oscillator) instead of HSI (internal RC oscillator) as the clock source.
    /// Will result in a hang if an external oscillator is not connected or it fails to start.
    pub fn use_hse(mut self, freq: Hertz) -> Self {
        self.hse = Some(freq.raw());
        self
    }

    /// Bypasses the high-speed external oscillator and uses an external clock input on the OSC_IN
    /// pin.
    ///
    /// For this configuration, the OSC_IN pin should be connected to a clock source with a
    /// frequency specified in the call to use_hse(), and the OSC_OUT pin should not be connected.
    ///
    /// This function has no effect unless use_hse() is also called.
    pub fn bypass_hse_oscillator(self) -> Self {
        Self {
            hse_bypass: true,
            ..self
        }
    }

    pub fn hclk(mut self, freq: Hertz) -> Self {
        self.hclk = Some(freq.raw());
        self
    }

    pub fn pclk1(mut self, freq: Hertz) -> Self {
        self.pclk1 = Some(freq.raw());
        self
    }

    pub fn pclk2(mut self, freq: Hertz) -> Self {
        self.pclk2 = Some(freq.raw());
        self
    }

    pub fn sysclk(mut self, freq: Hertz) -> Self {
        self.sysclk = Some(freq.raw());
        self
    }

    #[inline(always)]
    fn pll_setup(&self, pllsrcclk: u32, pllsysclk: Option<u32>) -> PllSetup {
        let main_pll = MainPll::fast_setup(pllsrcclk, self.hse.is_some(), pllsysclk);

        PllSetup {
            use_pll: main_pll.use_pll,
            pllsysclk: main_pll.pllsysclk,
        }
    }

 

    fn flash_setup(sysclk: u32) {
        use crate::pac::Flash;


        let flash_latency_step = 24_000_000;

        unsafe {
            let flash = &(*Flash::ptr());
            // Adjust flash wait states
            flash.ac().modify(|_, w| {
                w.latency().bits(((sysclk - 1) / flash_latency_step) as u8);
                w.prftbfe().set_bit();
                w.icahen().set_bit()
            })
        }
    }

    /// Initialises the hardware according to CFGR state returning a Clocks instance.
    /// Panics if overclocking is attempted.
    pub fn freeze(self) -> Clocks {
        self.freeze_internal(false)
    }

    /// Initialises the hardware according to CFGR state returning a Clocks instance.
    /// Allows overclocking.
    ///
    /// # Safety
    ///
    /// This method does not check if the clocks are bigger or smaller than the officially
    /// recommended.
    pub unsafe fn freeze_unchecked(self) -> Clocks {
        self.freeze_internal(true)
    }

    fn freeze_internal(self, unchecked: bool) -> Clocks {
        let rcc = unsafe { &*Rcc::ptr() };

        let pllsrcclk = self.hse.unwrap_or(HSI);
        let sysclk = self.sysclk.unwrap_or(pllsrcclk);
        let sysclk_on_pll = sysclk != pllsrcclk;

        let plls = self.pll_setup(pllsrcclk, sysclk_on_pll.then_some(sysclk));
        let sysclk = if sysclk_on_pll {
            plls.pllsysclk.unwrap()
        } else {
            sysclk
        };

        assert!(unchecked || !sysclk_on_pll || (SYSCLK_MIN..=SYSCLK_MAX).contains(&sysclk));

        let hclk = self.hclk.unwrap_or(sysclk);
        let (hpre_bits, hpre_div) = match (sysclk + hclk - 1) / hclk {
            0 => unreachable!(),
            1 => (Ahbpres::Div1, 1),
            2 => (Ahbpres::Div2, 2),
            3..=5 => (Ahbpres::Div4, 4),
            6..=11 => (Ahbpres::Div8, 8),
            12..=39 => (Ahbpres::Div16, 16),
            40..=95 => (Ahbpres::Div64, 64),
            96..=191 => (Ahbpres::Div128, 128),
            192..=383 => (Ahbpres::Div256, 256),
            _ => (Ahbpres::Div512, 512),
        };

        // Calculate real AHB clock
        let hclk = sysclk / hpre_div;

        let pclk1 = self
            .pclk1
            .unwrap_or_else(|| core::cmp::min(PCLK1_MAX, hclk));
        let (ppre1_bits, ppre1) = match (hclk + pclk1 - 1) / pclk1 {
            0 => unreachable!(),
            1 => (Apb1pres::Div1, 1u8),
            2 => (Apb1pres::Div2, 2),
            3..=5 => (Apb1pres::Div4, 4),
            6..=11 => (Apb1pres::Div8, 8),
            _ => (Apb1pres::Div16, 16),
        };

        // Calculate real APB1 clock
        let pclk1 = hclk / u32::from(ppre1);

        assert!(unchecked || pclk1 <= PCLK1_MAX);

        let pclk2 = self
            .pclk2
            .unwrap_or_else(|| core::cmp::min(PCLK2_MAX, hclk));
        let (ppre2_bits, ppre2) = match (hclk + pclk2 - 1) / pclk2 {
            0 => unreachable!(),
            1 => (Apb1pres::Div1, 1u8),
            2 => (Apb1pres::Div2, 2),
            3..=5 => (Apb1pres::Div4, 4),
            6..=11 => (Apb1pres::Div8, 8),
            _ => (Apb1pres::Div16, 16),
        };

        // Calculate real APB2 clock
        let pclk2 = hclk / u32::from(ppre2);

        assert!(unchecked || pclk2 <= PCLK2_MAX);

        Self::flash_setup(sysclk);

        if self.hse.is_some() {
            // enable HSE and wait for it to be ready
            rcc.ctrl().modify(|_, w| {
                if self.hse_bypass {
                    w.hsebp().set_bit();
                }
                w.hseen().set_bit()
            });
            while rcc.ctrl().read().hserdf().bit_is_clear() {}
        }

        if plls.use_pll {
            // Enable PLL
            rcc.ctrl().modify(|_, w| w.pllen().set_bit());

            // Wait for PLL to stabilise
            while rcc.ctrl().read().pllrdf().bit_is_clear() {}
        }

        // Set scaling factors
        rcc.cfg().modify(|_, w| {
            w.apb2pres().variant(ppre2_bits);
            w.apb1pres().variant(ppre1_bits);
            w.ahbpres().variant(hpre_bits)
        });

        // Wait for the new prescalers to kick in
        // "The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after write"
        cortex_m::asm::delay(16);

        let usb_pres = match hclk {
            144_000_000 => 0x3,
            96_000_000 => 0x2,
            48_000_000 => 0x1,
            72_000_000 => 0x0,
            _ => 0x3,
        };

        rcc.cfg().modify(|_,w| {
            unsafe { w.usbpres().bits(usb_pres) }
        });
        

        
        let (adc_1m_sel,adc_1m_pres) = if self.hse.is_none() || pllsrcclk > 32_000_000 {
            (false,(HSI / 1_000_000) - 1)
        } else {
            (true, (pllsrcclk / 1_000_000) - 1)
        };
        rcc.cfg2().modify(|_,w| unsafe { w.adc1msel().variant(adc_1m_sel).adc1mpres().bits(adc_1m_pres as u8) });

        let (trng_1m_sel,trng_1m_pres) = match self.hse {
            Some(2_000_000) => (true , 0b00001),
            Some(4_000_000) => (true , 0b00011),
            Some(6_000_000) => (true , 0b00101),
            Some(8_000_000) => (true , 0b00111),
            Some(10_000_000) => (true , 0b01001),
            Some(12_000_000) => (true , 0b01011),
            Some(14_000_000) => (true , 0b01101),
            Some(16_000_000) => (true , 0b01111),
            Some(18_000_000) => (true , 0b10001),
            Some(20_000_000) => (true , 0b10011),
            Some(22_000_000) => (true , 0b10101),
            Some(24_000_000) => (true , 0b10111),
            Some(26_000_000) => (true , 0b11001),
            Some(28_000_000) => (true , 0b11011),
            Some(30_000_000) => (true , 0b11101),
            Some(32_000_000) => (true , 0b11111),
            _ => (false, 0b00110)
        };
        rcc.cfg2().modify(|_,w| unsafe { w.adchpres().bits(0b0001).adcpllpres().bits(0b10001)});
        rcc.cfg3().modify(|_,w| unsafe { w.trng1msel().variant(trng_1m_sel).trng1mpres().bits(trng_1m_pres) });
        rcc.cfg().modify(|_,w| {
            unsafe { w.usbpres().bits(usb_pres) }
        });

        // Select system clock source
        rcc.cfg().modify(|_, w| {
            w.sclksw().variant(if sysclk_on_pll {
                Sclksw::Pll
            } else if self.hse.is_some() {
                Sclksw::Hse
            } else {
                Sclksw::Hsi
            })
        });

        let clocks = Clocks {
            hclk: hclk.Hz(),
            pclk1: pclk1.Hz(),
            pclk2: pclk2.Hz(),
            sysclk: sysclk.Hz(),
        };

        clocks
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct PllSetup {
    use_pll: bool,
    pllsysclk: Option<u32>,
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Clocks {
    pub hclk: Hertz,
    pub pclk1: Hertz,
    pub pclk2: Hertz,
    pub sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB1
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
