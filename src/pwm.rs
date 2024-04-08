//! Pulse Width Modulation (PWM)
//!
//! PWM output is avaliable for the advanced control timers (`TIM1`, `TIM8`),
//! the general purpose timers (`TIM[2-5]`, `TIM[12-17]`) and the Low-power
//! timers (`LPTIM[1-5]`).
//!
//! Timers support up to 4 simultaneous PWM output channels
//!
//! ## Usage
//!
//! ```rust
//! let gpioa = ..; // Set up and split GPIOA
//! let pins = (
//!     gpioa.pa8.into_alternate_af1(),
//!     gpioa.pa9.into_alternate_af1(),
//!     gpioa.pa10.into_alternate_af1(),
//!     gpioa.pa11.into_alternate_af1(),
//! );
//! ```
//!
//! To see which pins can be used with which timers, see your device datasheet or see which pins implement the [Pins](trait.Pins.html) trait.
//!
//! Then call the `pwm` function on the corresponding timer:
//!
//! ```
//!   let device: pac::Peripherals = ..;
//!
//!   // Put the timer in PWM mode using the specified pins
//!   // with a frequency of 100 hz.
//!   let (c0, c1, c2, c3) = device.TIM1.pwm(
//!       pins,
//!       100.hz(),
//!       prec,
//!       &clocks
//!   );
//!
//!   // Set the duty cycle of channel 0 to 50%
//!   c0.set_duty(c0.get_max_duty() / 2);
//!   // PWM outputs are disabled by default
//!   c0.enable()
//! ```
//!
//! ## Advanced features
//!
//! Some timers support various advanced features. These features are
//! accessed by calling TIMx.[pwm_advanced](trait.PwmAdvExt.html#tymethod.pwm_advanced) to get a [PwmBuilder](struct.PwmBuilder.html)
//! and calling appropriate methods of the PwmBuilder before calling [PwmBuilder::finalize](struct.PwmBuilder.html#method.finalize)
//! to create the PWM channels and a [PwmControl](struct.PwmControl.html) struct exposing things like [FaultMonitor](trait.FaultMonitor.html) functionality.
//!
//! ```rust
//! let gpioa = ..; // Set up and split GPIOA
//! let pins = (
//!     gpioa.pa8.into_alternate_af1(),
//!     gpioa.pa9.into_alternate_af1(),
//!     gpioa.pa10.into_alternate_af1(),
//!     gpioa.pa11.into_alternate_af1(),
//! );
//! ```
//!
//! Then call the `pwm_advanced` function on the corresponding timer:
//!
//! ```
//!   let device: pac::Peripherals = ..;
//!
//!   // Put the timer in PWM mode using the specified pins
//!   // with a frequency of 100 hz, 2us deadtime between complementary edges,
//!   // center-aligned PWM, and an active-low fault input
//!   let (mut control, (c1, c2, c3, c4)) = device.TIM1
//!       .pwm_advanced(
//!           pins,
//!           prec,
//!           &clocks
//!       )
//!       .frequency(100.hz())
//!       .center_aligned()
//!       .with_break_pin(gpioe.pe15.into_alternate_af1(), Polarity::ActiveLow)
//!       .finalize();
//! ```
//!
//! Then change some PWM channels to active-low (reversing pin polarity relative to logic polarity)
//! or enable a complementary PWM output (both only on some timer channels).
//!
//! ```
//!   // Set channel 1 to complementary with both the regular and complementary output active low
//!   let mut c1 = c1
//!       .into_complementary(gpioe.pe8.into_alternate_af1())
//!       .into_active_low()
//!       .into_comp_active_low();
//!
//!   // Set the duty cycle of channel 1 to 50%
//!   c1.set_duty(c1.get_max_duty() / 2);
//!
//!   // PWM outputs are disabled by default
//!   c1.enable()
//!
//!   // If a PWM fault happened, you can clear it through the control structure
//!   if control.is_fault_active() {
//!       control.clear_fault();
//!   }
//! ```
//!
//! ## Fault (Break) inputs
//!
//! The [PwmBuilder::with_break_pin](struct.PwmBuilder.html#method.with_break_pin) method emables break/fault functionality as described in the reference manual.
//!
//! The [PwmControl](struct.PwmControl.html) will then implement [FaultMonitor](trait.FaultMonitor.html) which can be used to monitor and control the fault status.
//!
//! If the break input becomes active, all PWM will be stopped.
//!
//! The BKIN hardware respects deadtimes when going into the fault state while the BKIN2 hardware acts immediately.
//!
//! The fault state puts all PWM pins into high-impedance mode, so pull-ups or pull-downs should be used to set the pins to a safe state.
//!
//! Currently only one break input (BKIN or BKIN2) can be enabled, this could be changed to allow two break inputs at the same time.
//!
//! ## Complementary outputs
//!
//! Once a PWM channel has been created through TIMx.pwm(...) or TIMx.pwm_advanced(...).finalize(), it can be put into complementary mode or have its polarity changed.
//!
//! [Pwm::into_complementary](struct.Pwm.html#method.into_complementary) takes a pin that can be used as a complementary output
//!
//! For allowed pins, see the device datasheet or see which pins implement the [NPins](trait.NPins.html) trait.
//!
//! ## PWM alignment
//!
//! A timer with multiple PWM channels can have different alignments between PWM channels.
//!
//! All PWM-capable timers support left aligned PWM. In left aligned PWM, all channels go active at the start of a PWM cycle then go inactive when their duty cycles expire.
//!
//! Some timers also support right aligned and center aligned PWM. In right aligned PWM, all channels go inactive at the end of a PWM cycle and go active when their duty cycle remains until the end of the PWM cycle.
//!
//! In center aligned PWM, all channels start inactive, then go active when half their duty cycle remains until the center of the PWM period, then go inactive again once their duty cycle expires and remain inactive for the rest of the PWM cycle.
//! This produces a symmetrical PWM waveform, with increasing duty cycle moving both the inactive and active edge equally.
//! When a component is placed across multiple PWM channels with different duty cycles in center aligned mode, the component will see twice the ripple frequency as the PWM switching frequency.
//!
//! ## PWM channel polarity
//!
//! A PWM channel is active or inactive based on the duty cycle, alignment, etc. However, the actual GPIO signal level that represents active vs inactive is configurable.
//!
//! The [into_active_low](struct.Pwm.html#method.into_active_low) and [into_active_high](struct.Pwm.html#method.into_active_high) methods set the active signal level to low (VSS) or high (VDD).
//!
//! The complementary output is active when the regular output is inactive. The active signal level of the complementary output is set by the [into_comp_active_low](struct.Pwm.html#method.into_comp_active_low), and [into_comp_active_high](struct.Pwm.html#method.into_comp_active_high) methods.
//!
//! ## Deadtime
//!
//! All channels on a given timer share the same deadtime setting as set by [PwmBuilder::with_deadtime](struct.PwmBuilder.html#method.with_deadtime)
//!
//! PWM channels with complementary outputs can have deadtime added to the signal. Dead time is used to prevent cross-conduction in some power electronics topologies.
//!
//! With complementary outputs and dead time enabled on a PWM channel, when the regular output goes inactive (high or low based on into_active_high/into_active_low), the complementary output remains inactive until the deadtime passes.
//! Similarily, when the complementary output goes inactive, the regular output waits until the deadtime passes before it goes active.
//!
//! Deadtime is applied based on the logical active/inactive levels. Depending on the PWM polarity and complementary polarity, both pins can be high or low during deadtime; they will both be in the inactive state.
//!
//! The deadtime must be 4032 counts of the timer clock or less or the builder will assert/panic. For a 200MHz timer this is 20 microseconds; slower timers can have even longer deadtimes.
//!
//! ## Disabled or faulted state
//!
//! At initialization, when a PWM channel is disabled, or while a fault is active, the PWM outputs will be in a high impedance state.
//!
//! If needed, pull-up or pull-down resistors should be used to ensure that all power electronics are in a safe state while the GPIO pins are high impedance.
//!
//! Although the timers allow quite a bit of configuration here, that would require configuring the PWM pins before configuring other parts of the timer, which would be a challenge with how type states and traits are used for timer configuration.
//!
//! Additionally, the GPIO will always be high-impedance during power-up or in reset, so pull-ups or pull-downs to ensure safe state are always a good idea.
//!
//! ## Origin
//!
//! This code has been taken from the stm32h7xx-hal project and modified slightly to support
//! STM32G4xx MCUs. It has originally been licensed under the 0-clause BSD license.

use core::marker::PhantomData;
use core::mem::MaybeUninit;

use crate::gpio::*;
use crate::pac::Rcc;

use crate::pac::{Tim1, Tim2, Tim3, Tim4, Tim5, Tim6, Tim7, Tim8};

use crate::rcc::{Enable, BusTimerClock, Clocks, Reset};
use crate::time::{ExtU32, Hertz, NanoSecond, RateExtU32};

// This trait marks that a GPIO pin can be used with a specific timer channel
// TIM is the timer being used
// CHANNEL is a marker struct for the channel (or multi channels for tuples)
// Example: impl Pins<TIM1, C1> for PA8<Alternate<AF1>> { type Channel = Pwm<TIM1, C1>; }
/// Pins is a trait that marks which GPIO pins may be used as PWM channels; it should not be directly used.
/// See the device datasheet 'Pin descriptions' chapter for which pins can be used with which timer PWM channels (or look at Implementors)
pub trait Pins<TIM, CHANNEL, COMP> {
    type Channel;
}

/// NPins is a trait that marks which GPIO pins may be used as complementary PWM channels; it should not be directly used.
/// See the device datasheet 'Pin descriptions' chapter for which pins can be used with which timer PWM channels (or look at Implementors)
pub trait NPins<TIM, CHANNEL> {}

/// FaultPins is a trait that marks which GPIO pins may be used as PWM fault inputs; it should not be directly used.
/// See the device datasheet 'Pin descriptions' chapter for which pins can be used with which timer PWM channels (or look at Implementors)
pub trait FaultPins<TIM> {
    const INPUT: BreakInput;
}

/// Marker struct for PWM channel 1 on Pins trait and Pwm struct
pub struct C1;
/// Marker struct for PWM channel 2 on Pins trait and Pwm struct
pub struct C2;
/// Marker struct for PWM channel 3 on Pins trait and Pwm struct
pub struct C3;
/// Marker struct for PWM channel 4 on Pins trait and Pwm struct
pub struct C4;

/// Marker struct for pins and PWM channels that do not support complementary output
pub struct ComplementaryImpossible;
/// Marker struct for pins and PWM channels that support complementary output but are not using it
pub struct ComplementaryDisabled;
/// Marker struct for PWM channels that have complementary outputs enabled
pub struct ComplementaryEnabled;

/// Enum for IO polarity
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

/// Configuration enum to keep track of which break input corresponds with which FaultPins
#[derive(PartialEq)]
pub enum BreakInput {
    BreakIn,
    BreakIn2,
}

/// Internal enum that keeps track of the count settings before PWM is finalized
enum CountSettings<WIDTH> {
    Frequency(Hertz),
    Explicit { period: WIDTH, prescaler: u16 },
}

/// Marker struct for active high IO polarity
pub struct ActiveHigh;
/// Marker struct for active low IO polarity
pub struct ActiveLow;

/// Whether a PWM signal is left-aligned, right-aligned, or center-aligned
#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Pwm represents one PWM channel; it is created by calling TIM?.pwm(...) and lets you control the channel through the PwmPin trait
pub struct Pwm<TIM, CHANNEL, COMP, POL, NPOL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
    _complementary: PhantomData<COMP>,
    _polarity: PhantomData<POL>,
    _npolarity: PhantomData<NPOL>,
}

/// PwmBuilder is used to configure advanced PWM features
pub struct PwmBuilder<TIM, PINS, CHANNEL, FAULT, COMP, WIDTH> {
    _tim: PhantomData<TIM>,
    _pins: PhantomData<PINS>,
    _channel: PhantomData<CHANNEL>,
    _fault: PhantomData<FAULT>,
    _comp: PhantomData<COMP>,
    alignment: Alignment,
    base_freq: Hertz,
    count: CountSettings<WIDTH>,
    bkin_enabled: bool, // If the FAULT type parameter is FaultEnabled, either bkin or bkin2 must be enabled
    fault_polarity: Polarity,
    deadtime: NanoSecond,
}

/// Allows a PwmControl to monitor and control faults (break inputs) of a timer's PWM channels
pub trait FaultMonitor {
    /// Returns true if a fault is preventing PWM output
    fn is_fault_active(&self) -> bool;

    /// Enables PWM output, clearing fault state and immediately resuming PWM; if the break pin is still active, this can't clear the fault.
    fn clear_fault(&mut self);

    /// Disables PWM output, setting fault state; this can be used to stop all PWM from a timer in software detected faults
    fn set_fault(&mut self);
}

/// Exposes timer wide advanced features, such as [FaultMonitor](trait.FaultMonitor.html)
/// or future features like trigger outputs for synchronization with ADCs and other peripherals
pub struct PwmControl<TIM, FAULT> {
    _tim: PhantomData<TIM>,
    _fault: PhantomData<FAULT>,
}

/// Marker struct indicating that a PwmControl is in charge of fault monitoring
pub struct FaultEnabled;
/// Marker struct indicating that a PwmControl does not handle fault monitoring
pub struct FaultDisabled;

// automatically implement Pins trait for tuples of individual pins
macro_rules! pins_tuples {
    // Tuple of two pins
    ($(($CHA:ty, $CHB:ty)),*) => {
        $(
            impl<TIM, CHA, CHB, TA, TB> Pins<TIM, ($CHA, $CHB), (TA, TB)> for (CHA, CHB)
            where
                CHA: Pins<TIM, $CHA, TA>,
                CHB: Pins<TIM, $CHB, TB>,
            {
                type Channel = (Pwm<TIM, $CHA, TA, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHB, TB, ActiveHigh, ActiveHigh>);
            }
        )*
    };
    // Tuple of three pins
    ($(($CHA:ty, $CHB:ty, $CHC:ty)),*) => {
        $(
            pins_tuples! {
                PERM3: ($CHA, $CHB, $CHC),
                PERM3: ($CHA, $CHC, $CHB),
                PERM3: ($CHB, $CHA, $CHC),
                PERM3: ($CHB, $CHC, $CHA),
                PERM3: ($CHC, $CHA, $CHB),
                PERM3: ($CHC, $CHB, $CHA)
            }
        )*
    };
    // Permutate tuple of three pins
    ($(PERM3: ($CHA:ty, $CHB:ty, $CHC:ty)),*) => {
        $(
            impl<TIM, CHA, CHB, CHC, TA, TB, TC> Pins<TIM, ($CHA, $CHB, $CHC), (TA, TB, TC)> for (CHA, CHB, CHC)
            where
                CHA: Pins<TIM, $CHA, TA>,
                CHB: Pins<TIM, $CHB, TB>,
                CHC: Pins<TIM, $CHC, TC>,
            {
                type Channel = (Pwm<TIM, $CHA, TA, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHB, TB, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHC, TC, ActiveHigh, ActiveHigh>);
            }
        )*
    };
    // Tuple of four pins (permutates the last 3, leaves 4th in place)
    ($(($CHD:ty, $CHA:ty, $CHB:ty, $CHC:ty)),*) => {
        $(
            pins_tuples! {
                PERM4: ($CHD, $CHA, $CHB, $CHC),
                PERM4: ($CHD, $CHA, $CHC, $CHB),
                PERM4: ($CHD, $CHB, $CHA, $CHC),
                PERM4: ($CHD, $CHB, $CHC, $CHA),
                PERM4: ($CHD, $CHC, $CHA, $CHB),
                PERM4: ($CHD, $CHC, $CHB, $CHA)
            }
        )*
    };
    // Tuple of four pins (permutates the last 3, leaves 1st in place)
    ($(PERM4: ($CHA:ty, $CHB:ty, $CHC:ty, $CHD:ty)),*) => {
        $(
            impl<TIM, CHA, CHB, CHC, CHD, TA, TB, TC, TD> Pins<TIM, ($CHA, $CHB, $CHC, $CHD), (TA, TB, TC, TD)> for (CHA, CHB, CHC, CHD)
            where
                CHA: Pins<TIM, $CHA, TA>,
                CHB: Pins<TIM, $CHB, TB>,
                CHC: Pins<TIM, $CHC, TC>,
                CHD: Pins<TIM, $CHD, TD>,
            {
                type Channel = (Pwm<TIM, $CHA, TA, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHB, TB, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHC, TC, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHD, TD, ActiveHigh, ActiveHigh>);
            }
        )*
    }
}

pins_tuples! {
    (C1, C2),
    (C2, C1),
    (C1, C3),
    (C3, C1),
    (C1, C4),
    (C4, C1),
    (C2, C3),
    (C3, C2),
    (C2, C4),
    (C4, C2),
    (C3, C4),
    (C4, C3)
}

pins_tuples! {
    (C1, C2, C3),
    (C1, C2, C4),
    (C1, C3, C4),
    (C2, C3, C4)
}

pins_tuples! {
    (C1, C2, C3, C4),
    (C2, C1, C3, C4),
    (C3, C1, C2, C4),
    (C4, C1, C2, C3)
}

// Pin definitions, mark which pins can be used with which timers and channels
macro_rules! pins {
    // Single channel timer
    ($($TIMX:ty: OUT: [$($OUT:ty),*])+) => {
        $(
            $(
                impl Pins<$TIMX, C1, ComplementaryImpossible> for $OUT {
                    type Channel = Pwm<$TIMX, C1, ComplementaryImpossible, ActiveHigh, ActiveHigh>;
                }
            )*
        )+
    };
    // Dual channel timer $pm
    ($($TIMX:ty:
        CH1($COMP1:ty): [$($( #[ $pmeta1:meta ] )* $CH1:ty),*] CH2($COMP2:ty): [$($( #[ $pmeta2:meta ] )* $CH2:ty),*]
        CH1N: [$($( #[ $pmeta3:meta ] )* $CH1N:ty),*] CH2N: [$($( #[ $pmeta4:meta ] )* $CH2N:ty),*] BRK: [$($( #[ $pmeta5:meta ] )* $BRK:ty),*])+) => {
        $(
            $(
                $( #[ $pmeta1 ] )*
                impl Pins<$TIMX, C1, $COMP1> for $CH1 {
                    type Channel = Pwm<$TIMX, C1, $COMP1, ActiveHigh, ActiveHigh>;
                }
            )*
            $(
                $( #[ $pmeta2 ] )*
                impl Pins<$TIMX, C2, $COMP2> for $CH2 {
                    type Channel = Pwm<$TIMX, C2, $COMP2, ActiveHigh, ActiveHigh>;
                }
            )*
            $(
                $( #[ $pmeta3 ] )*
                impl NPins<$TIMX, C1> for $CH1N {}
            )*
            $(
                $( #[ $pmeta4 ] )*
                impl NPins<$TIMX, C2> for $CH2N {}
            )*
            $(
                $( #[ $pmeta5 ] )*
                impl FaultPins<$TIMX,> for $BRK {
                    const INPUT: BreakInput = BreakInput::BreakIn;
                }
            )*
        )+
    };
    // Quad channel timers
    ($($TIMX:ty:
       CH1($COMP1:ty): [$($( #[ $pmeta1:meta ] )* $CH1:ty),*] CH2($COMP2:ty): [$($( #[ $pmeta2:meta ] )* $CH2:ty),*]
       CH3($COMP3:ty): [$($( #[ $pmeta3:meta ] )* $CH3:ty),*] CH4($COMP4:ty): [$($( #[ $pmeta4:meta ] )* $CH4:ty),*]
       CH1N: [$($( #[ $pmeta5:meta ] )* $CH1N:ty),*] CH2N: [$($( #[ $pmeta6:meta ] )* $CH2N:ty),*]
       CH3N: [$($( #[ $pmeta7:meta ] )* $CH3N:ty),*] CH4N: [$($( #[ $pmeta8:meta ] )* $CH4N:ty),*]
       BRK: [$($( #[ $pmeta9:meta ] )* $BRK:ty),*])+) => {
        $(
            $(
                $( #[ $pmeta1 ] )*
                impl Pins<$TIMX, C1, $COMP1> for $CH1 {
                    type Channel = Pwm<$TIMX, C1, $COMP1, ActiveHigh, ActiveHigh>;
                }
            )*
            $(
                $( #[ $pmeta2 ] )*
                impl Pins<$TIMX, C2, $COMP2> for $CH2 {
                    type Channel = Pwm<$TIMX, C2, $COMP2, ActiveHigh, ActiveHigh>;
                }
            )*
            $(
                $( #[ $pmeta3 ] )*
                impl Pins<$TIMX, C3, $COMP3> for $CH3 {
                    type Channel = Pwm<$TIMX, C3, $COMP3, ActiveHigh, ActiveHigh>;
                }
            )*
            $(
                $( #[ $pmeta4 ] )*
                impl Pins<$TIMX, C4, $COMP4> for $CH4 {
                    type Channel = Pwm<$TIMX, C4, $COMP4, ActiveHigh, ActiveHigh>;
                }
            )*
            $(
                $( #[ $pmeta5 ] )*
                impl NPins<$TIMX, C1> for $CH1N {}
            )*
            $(
                $( #[ $pmeta6 ] )*
                impl NPins<$TIMX, C2> for $CH2N {}
            )*
            $(
                $( #[ $pmeta7 ] )*
                impl NPins<$TIMX, C3> for $CH3N {}
            )*
            $(
                $( #[ $pmeta8 ] )*
                impl NPins<$TIMX, C4> for $CH4N {}
            )*
            $(
                $( #[ $pmeta9 ] )*
                impl FaultPins<$TIMX> for $BRK {
                    const INPUT: BreakInput = BreakInput::BreakIn;
                }
            )*
        )+
    }
}

pins! {
    Tim1:
        CH1(ComplementaryDisabled): [
            PA8<Alternate<PushPull>>,
            PE9<Alternate<PushPull>>
        ]
        CH2(ComplementaryDisabled): [
            PA9<Alternate<PushPull>>,
            PE11<Alternate<PushPull>>
        ]
        CH3(ComplementaryDisabled): [
            PA10<Alternate<PushPull>>,
            PE13<Alternate<PushPull>>
        ]
        CH4(ComplementaryDisabled): [
            PA11<Alternate<PushPull>>,
            PE14<Alternate<PushPull>>
        ]
        CH1N: [
            PA7<Alternate<PushPull>>,
            PB13<Alternate<PushPull>>,
            PE8<Alternate<PushPull>>
        ]
        CH2N: [
            PB0<Alternate<PushPull>>,
            PB14<Alternate<PushPull>>,
            PE10<Alternate<PushPull>>
        ]
        CH3N: [
            PB1<Alternate<PushPull>>,
            PB15<Alternate<PushPull>>,
            PE12<Alternate<PushPull>>
        ]
        CH4N: [
        ]
        BRK: [
            PB12<Alternate<PushPull>>,
            PA6<Alternate<PushPull>>,
            PB5<Alternate<PushPull>>,
            PE15<Alternate<PushPull>>
        ]
    Tim2:
        CH1(ComplementaryImpossible): [
            PA0<Alternate<PushPull>>,
            PA15<Alternate<PushPull>>
        ]
        CH2(ComplementaryImpossible): [
            PA1<Alternate<PushPull>>,
            PB3<Alternate<PushPull>>
        ]
        CH3(ComplementaryImpossible): [
            PA2<Alternate<PushPull>>,
            PB10<Alternate<PushPull>>
        ]
        CH4(ComplementaryImpossible): [
            PA3<Alternate<PushPull>>,
            PB11<Alternate<PushPull>>
        ]
        CH1N: [
        ]
        CH2N: [
        ]
        CH3N: [
        ]
        CH4N: [
        ]
        BRK: [
        ]
    Tim3:
        CH1(ComplementaryImpossible): [
            PA6<Alternate<PushPull>>,
            PC6<Alternate<PushPull>>,
            PB4<Alternate<PushPull>>

        ]
        CH2(ComplementaryImpossible): [
            PA7<Alternate<PushPull>>,
            PC7<Alternate<PushPull>>,
            PB5<Alternate<PushPull>>
        ]
        CH3(ComplementaryImpossible): [
            PB0<Alternate<PushPull>>,
            PC8<Alternate<PushPull>>
        ]
        CH4(ComplementaryImpossible): [
            PB1<Alternate<PushPull>>,
            PC9<Alternate<PushPull>>
        ]
        CH1N: [
        ]
        CH2N: [
        ]
        CH3N: [
        ]
        CH4N: [
        ]
        BRK: [
        ]
    Tim4:
        CH1(ComplementaryImpossible): [
            PB6<Alternate<PushPull>>,
            PD12<Alternate<PushPull>>
        ]
        CH2(ComplementaryImpossible): [
            PB7<Alternate<PushPull>>,
            PD13<Alternate<PushPull>>
        ]
        CH3(ComplementaryImpossible): [
            PB8<Alternate<PushPull>>,
            PD14<Alternate<PushPull>>
        ]
        CH4(ComplementaryImpossible): [
            PB9<Alternate<PushPull>>,
            PD15<Alternate<PushPull>>
        ]
        CH1N: [
        ]
        CH2N: [
        ]
        CH3N: [
        ]
        CH4N: [
        ]
        BRK: [
        ]
    Tim8:
        CH1(ComplementaryDisabled): [
            PC6<Alternate<PushPull>>,
            PD14<Alternate<PushPull>>
        ]
        CH2(ComplementaryDisabled): [
            PC7<Alternate<PushPull>>,
            PD15<Alternate<PushPull>>
        ]
        CH3(ComplementaryDisabled): [
            PC8<Alternate<PushPull>>
        ]
        CH4(ComplementaryDisabled): [
            PC9<Alternate<PushPull>>
        ]
        CH1N: [
            PA7<Alternate<PushPull>>,
            PA15<Alternate<PushPull>>
        ]
        CH2N: [
            PB0<Alternate<PushPull>>,
            PC12<Alternate<PushPull>>
        ]
        CH3N: [
            PB1<Alternate<PushPull>>,
            PD2<Alternate<PushPull>>
        ]
        CH4N: [
        ]
        BRK: [
            PA6<Alternate<PushPull>>,
            PB3<Alternate<PushPull>>
        ]
}
// Period and prescaler calculator for 32-bit timers
// Returns (arr, psc)
fn calculate_frequency_32bit(base_freq: Hertz, freq: Hertz, alignment: Alignment) -> (u32, u16) {
    let divisor = if let Alignment::Center = alignment {
        freq * 2
    } else {
        freq
    };

    // Round to the nearest period
    let arr = (base_freq + (divisor / 2)) / divisor - 1;

    (arr, 0)
}

// Period and prescaler calculator for 16-bit timers
// Returns (arr, psc)
// Returns as (u32, u16) to be compatible but arr will always be a valid u16
fn calculate_frequency_16bit(base_freq: Hertz, freq: Hertz, alignment: Alignment) -> (u32, u16) {
    let ideal_period = calculate_frequency_32bit(base_freq, freq, alignment).0 + 1;

    // Division factor is (PSC + 1)
    let prescale = (ideal_period - 1) / (1 << 16);

    // This will always fit in a 16-bit value because u32::MAX / (1 << 16) fits in a 16 bit

    // Round to the nearest period
    let period = (ideal_period + (prescale >> 1)) / (prescale + 1) - 1;

    // It should be impossible to fail these asserts
    assert!(period <= 0xFFFF);
    assert!(prescale <= 0xFFFF);

    (period, prescale as u16)
}

// Deadtime calculator helper function
// Returns (BDTR.DTG, CR1.CKD)
fn calculate_deadtime(base_freq: Hertz, deadtime: NanoSecond) -> (u8, u8) {
    // tDTS is based on tCK_INT which is before the prescaler
    // It uses its own separate prescaler CR1.CKD

    // ticks = ns * GHz = ns * Hz / 1e9
    // Cortex-M7 has 32x32->64 multiply but no 64-bit divide
    // Divide by 100000 then 10000 by multiplying and shifting
    // This can't overflow because both values being multiplied are u32
    let deadtime_ticks = deadtime.ticks() as u64 * base_freq.raw() as u64;
    // Make sure we won't overflow when multiplying; DTG is max 1008 ticks and CKD is max prescaler of 4
    // so deadtimes over 4032 ticks are impossible (4032*10^9 before dividing)
    assert!(deadtime_ticks <= 4_032_000_000_000u64);
    let deadtime_ticks = deadtime_ticks * 42950;
    let deadtime_ticks = (deadtime_ticks >> 32) as u32;
    let deadtime_ticks = deadtime_ticks as u64 * 429497;
    let deadtime_ticks = (deadtime_ticks >> 32) as u32;

    // Choose CR1 CKD divider of 1, 2, or 4 to determine tDTS
    let (deadtime_ticks, ckd) = match deadtime_ticks {
        t if t <= 1008 => (deadtime_ticks, 1),
        t if t <= 2016 => (deadtime_ticks / 2, 2),
        t if t <= 4032 => (deadtime_ticks / 4, 4),
        _ => {
            panic!("Deadtime must be less than 4032 ticks of timer base clock.")
        }
    };

    // Choose BDTR DTG bits to match deadtime_ticks
    // We want the smallest value of DTG that gives a deadtime >= the requested deadtime
    for dtg in 0..=255 {
        let actual_deadtime: u32 = match dtg {
            d if d < 128 => d,
            d if d < 192 => 2 * (64 + (d & 0x3F)),
            d if d < 224 => 8 * (32 + (d & 0x1F)),
            _ => 16 * (32 + (dtg & 0x1F)),
        };

        if actual_deadtime >= deadtime_ticks {
            return (dtg as u8, ckd);
        }
    }

    panic!("This should be unreachable.");
}

// PwmExt trait
/// Allows the pwm() method to be added to the peripheral register structs from the device crate
pub trait PwmExt: Sized {
    /// The requested frequency will be rounded to the nearest achievable frequency; the actual frequency may be higher or lower than requested.
    fn pwm<PINS, T, U, V>(self, _pins: PINS, frequency: T, clock: &Clocks) -> PINS::Channel
    where
        PINS: Pins<Self, U, V>,
        T: Into<Hertz>;
}

pub trait PwmAdvExt<WIDTH>: Sized {
    fn pwm_advanced<PINS, CHANNEL, COMP>(
        self,
        _pins: PINS,
        clocks: &Clocks,
    ) -> PwmBuilder<Self, PINS, CHANNEL, FaultDisabled, COMP, WIDTH>
    where
        PINS: Pins<Self, CHANNEL, COMP>;
}

// Implement PwmExt trait for timer
macro_rules! pwm_ext_hal {
    ($TIMX:ident: $timX:ident) => {
        impl PwmExt for $TIMX {
            fn pwm<PINS, T, U, V>(self, pins: PINS, frequency: T, clocks: &Clocks) -> PINS::Channel
            where
                PINS: Pins<Self, U, V>,
                T: Into<Hertz>,
            {
                $timX(self, pins, frequency.into(), clocks)
            }
        }
    };
}

// Implement PWM configuration for timer
macro_rules! tim_hal {
    ($($TIMX:ident: ($timX:ident,
                     $typ:ty, $bits:expr $(, DIR: $cms:ident)* $(, BDTR: $bdtr:ident, $moe_set:ident)*),)+) => {
        $(
            pwm_ext_hal!($TIMX: $timX);

            /// Configures PWM
            fn $timX<PINS, T, U>(
                tim: $TIMX,
                _pins: PINS,
                freq: Hertz,
                clocks: &Clocks,
            ) -> PINS::Channel
            where
                PINS: Pins<$TIMX, T, U>,
            {
                unsafe {
                    let rcc_ptr = &(*Rcc::ptr());
                    $TIMX::enable(rcc_ptr);
                    $TIMX::reset(rcc_ptr);
                }

                let clk = $TIMX::timer_clock(&clocks);

                let (period, prescale) = match $bits {
                    16 => calculate_frequency_16bit(clk, freq, Alignment::Left),
                    _ => calculate_frequency_32bit(clk, freq, Alignment::Left),
                };

                // Write prescale
                tim.psc().write(|w| { unsafe { w.psc().bits(prescale as u16) } });

                // Write period
                tim.ar().write(|w| { unsafe { w.ar().bits(period as u16) } });

                // BDTR: Advanced-control timers
                $(
                    // Set CCxP = OCxREF / CCxNP = !OCxREF
                    // Refer to RM0433 Rev 6 - Table 324.
                    tim.$bdtr().write(|w|
                                   w.moen().$moe_set()
                    );
                )*

                tim.ctrl1().write(|w| w.cnten().set_bit());

                unsafe { MaybeUninit::<PINS::Channel>::uninit().assume_init() }
            }

            impl PwmAdvExt<$typ> for $TIMX {
                fn pwm_advanced<PINS, CHANNEL, COMP>(
                    self,
                    _pins: PINS,
                    clock: &Clocks,
                ) -> PwmBuilder<Self, PINS, CHANNEL, FaultDisabled, COMP, $typ>
                where
                    PINS: Pins<Self, CHANNEL, COMP>
                {
                    unsafe {
                        let rcc_ptr = &(*Rcc::ptr());
                        $TIMX::enable(rcc_ptr);
                        $TIMX::reset(rcc_ptr);
                    }

                    let clk = $TIMX::timer_clock(clock).raw();

                    PwmBuilder {
                        _tim: PhantomData,
                        _pins: PhantomData,
                        _channel: PhantomData,
                        _fault: PhantomData,
                        _comp: PhantomData,
                        alignment: Alignment::Left,
                        base_freq: clk.Hz(),
                        count: CountSettings::Explicit { period: 65535, prescaler: 0, },
                        bkin_enabled: false,
                        fault_polarity: Polarity::ActiveLow,
                        deadtime: 0.nanos(),
                    }
                }
            }

            impl<PINS, CHANNEL, FAULT, COMP>
                PwmBuilder<$TIMX, PINS, CHANNEL, FAULT, COMP, $typ>
            where
                PINS: Pins<$TIMX, CHANNEL, COMP>,
            {
                pub fn finalize(self) -> (PwmControl<$TIMX, FAULT>, PINS::Channel) {
                    let tim = unsafe { &*$TIMX::ptr() };

                    let (period, prescaler) = match self.count {
                        CountSettings::Explicit { period, prescaler } => (period as u32, prescaler),
                        CountSettings::Frequency( freq ) => {
                            match $bits {
                                16 => calculate_frequency_16bit(self.base_freq, freq, self.alignment),
                                _ => calculate_frequency_32bit(self.base_freq, freq, self.alignment),
                            }
                        },
                    };

                    // Write prescaler
                    tim.psc().write(|w| unsafe { w.psc().bits(prescaler as u16) });

                    // Write period
                    tim.ar().write(|w| unsafe { w.ar().bits(period as u16) });

                    $(
                        let (dtg, ckd) = calculate_deadtime(self.base_freq, self.deadtime);

                        match ckd {
                            1 => tim.ctrl1().modify(|_, w| unsafe { w.clkd().bits(0) }),
                            2 => tim.ctrl1().modify(|_, w| unsafe { w.clkd().bits(1) }),
                            4 => tim.ctrl1().modify(|_, w| unsafe { w.clkd().bits(2) }),
                            _ => panic!("Should be unreachable, invalid deadtime prescaler"),
                        }

                        let bkp = match self.fault_polarity {
                            Polarity::ActiveLow => false,
                            Polarity::ActiveHigh => true,
                        };

                        if self.bkin_enabled {
                            // BDTR:
                            //  BKF = 1 -> break pin filtering of 2 cycles of CK_INT (peripheral source clock)
                            //  AOE = 0 -> after a fault, master output enable MOE can only be set by software, not automatically
                            //  BKE = 1 -> break is enabled
                            //  BKP = 0 for active low, 1 for active high
                            // Safety: bkf is set to a constant value (1) that is a valid value for the field per the reference manual
                            unsafe { tim.$bdtr().write(|w| w.dtgn().bits(dtg).aoen().clear_bit().bken().set_bit().bkp().bit(bkp).moen().$moe_set()); }
                        }

                        else {
                            // Safety: the DTG field of BDTR allows any 8-bit deadtime value and the dtg variable is u8
                            unsafe {
                                tim.$bdtr().write(|w| w.dtgn().bits(dtg).aoen().clear_bit().moen().$moe_set());
                            }
                        }

                        // BDTR: Advanced-control timers
                        // Set CCxP = OCxREF / CCxNP = !OCxREF
                        // Refer to RM0433 Rev 6 - Table 324.
                        tim.$bdtr().modify(|_, w| w.moen().$moe_set());
                    )*


                    $(
                        match self.alignment {
                            Alignment::Left => { },
                            Alignment::Right => { tim.ctrl1().modify(|_, w| w.dir().set_bit()); }, // Downcounter
                            Alignment::Center => { tim.ctrl1().modify(|_, w| unsafe { w.$cms().bits(3) }); } // Center-aligned mode 3
                        }
                    )*

                    tim.ctrl1().modify(|_, w| w.cnten().set_bit());

                    unsafe {
                        MaybeUninit::<(PwmControl<$TIMX, FAULT>, PINS::Channel)>::uninit()
                            .assume_init()
                    }
                }

                /// Set the PWM frequency; will overwrite the previous prescaler and period
                /// The requested frequency will be rounded to the nearest achievable frequency; the actual frequency may be higher or lower than requested.
                pub fn frequency<T: Into<Hertz>>(mut self, freq: T) -> Self {
                    self.count = CountSettings::Frequency( freq.into() );

                    self
                }

                /// Set the prescaler; PWM count runs at base_frequency/(prescaler+1)
                pub fn prescaler(mut self, prescaler: u16) -> Self {
                    let period = match self.count {
                        CountSettings::Frequency(_) => 65535,
                        CountSettings::Explicit { period, prescaler: _ } => period,
                    };

                    self.count = CountSettings::Explicit { period, prescaler };

                    self
                }

                /// Set the period; PWM count runs from 0 to period, repeating every (period+1) counts
                pub fn period(mut self, period: $typ) -> Self {
                    let prescaler = match self.count {
                        CountSettings::Frequency(_) => 0,
                        CountSettings::Explicit { period: _, prescaler } => prescaler,
                    };

                    self.count = CountSettings::Explicit { period, prescaler };

                    self
                }


                // Timers with complementary and deadtime and faults
                $(
                    /// Set the deadtime for complementary PWM channels of this timer
                    pub fn with_deadtime<T: Into<NanoSecond>>(mut self, deadtime: T) -> Self {
                        // $bdtr is an Ident that only exists for timers with deadtime, so we can use it as a variable name to
                        // only implement this method for timers that support deadtime.
                        let $bdtr = deadtime.into();

                        self.deadtime = $bdtr;

                        self
                    }
                )*

                pub fn left_aligned( mut self ) -> Self {
                    self.alignment = Alignment::Left;

                    self
                }

                // Timers with advanced counting options, including center aligned and right aligned PWM
                $(
                    pub fn center_aligned( mut self ) -> Self {
                        // $cms is an Ident that only exists for timers with center/right aligned PWM, so we can use it as a variable name to
                        // only implement this method for timers that support center/right aligned PWM.
                        let $cms = Alignment::Center;

                        self.alignment = $cms;

                        self
                    }

                    pub fn right_aligned( mut self ) -> Self {
                        self.alignment = Alignment::Right;

                        self
                    }
                )*
            }

            // Timers with break/fault, dead time, and complimentary capabilities
            $(
                impl<PINS, CHANNEL, COMP> PwmBuilder<$TIMX, PINS, CHANNEL, FaultDisabled, COMP, $typ> {
                    /// Configure a break pin that will disable PWM when activated (active level based on polarity argument)
                    /// Note: not all timers have fault inputs; FaultPins<TIM> is only implemented for valid pins/timers.
                    pub fn with_break_pin<P: FaultPins<$TIMX>>(self, _pin: P, polarity: Polarity) -> PwmBuilder<$TIMX, PINS, CHANNEL, FaultEnabled, COMP, $typ> {
                        PwmBuilder {
                            _tim: PhantomData,
                            _pins: PhantomData,
                            _channel: PhantomData,
                            _fault: PhantomData,
                            _comp: PhantomData,
                            alignment: self.alignment,
                            base_freq: self.base_freq,
                            count: self.count,
                            bkin_enabled: self.bkin_enabled || P::INPUT == BreakInput::BreakIn,
                            fault_polarity: polarity,
                            deadtime: self.deadtime,
                        }
                    }
                }

                impl FaultMonitor for PwmControl<$TIMX, FaultEnabled> {
                    fn is_fault_active(&self) -> bool {
                        let tim = unsafe { &*$TIMX::ptr() };

                        !tim.$bdtr().read().moen().bit()
                    }

                    fn clear_fault(&mut self) {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.$bdtr().modify(|_, w| w.moen().set_bit());
                    }

                    fn set_fault(&mut self) {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.$bdtr().modify(|_, w| w.moen().clear_bit());
                    }
                }
            )*
        )+
    }
}

tim_hal! {
    Tim1: (tim1, u16, 16, DIR: camsel, BDTR: bkdt, set_bit),
    Tim2: (tim2, u16, 16, DIR: camsel),
    Tim3: (tim3, u16, 16, DIR: camsel),
    Tim4: (tim4, u16, 16, DIR: camsel),
    Tim5: (tim5, u16, 16, DIR: camsel),
}
tim_hal! {
    Tim8: (tim8, u16, 16, DIR: camsel, BDTR: bkdt, set_bit),
    Tim6: (tim7, u16, 16),
    Tim7: (tim6, u16, 16),
}

pub trait PwmPinEnable {
    fn ccer_enable(&mut self);
    fn ccer_disable(&mut self);
}

// Implement PwmPin for timer channels
macro_rules! tim_pin_hal {
    // Standard pins (no complementary functionality)
    ($($TIMX:ident:
       ($CH:ty, $ccxe:ident, $ccxp:ident, $ccmrx_output:ident, $ocxpe:ident, $ocxm:ident,
        $ccrx:ident, $typ:ident $(,$ccxne:ident, $ccxnp:ident)*),)+
    ) => {
        $(
            impl<COMP, POL, NPOL> embedded_hal_02::PwmPin for Pwm<$TIMX, $CH, COMP, POL, NPOL>
                where Pwm<$TIMX, $CH, COMP, POL, NPOL>: PwmPinEnable {
                type Duty = $typ;

                // You may not access self in the following methods!
                // See unsafe above

                fn disable(&mut self) {
                    self.ccer_disable();
                }

                fn enable(&mut self) {
                    let tim = unsafe { &*$TIMX::ptr() };

                    tim.$ccmrx_output().modify(|_, w|
                        unsafe { w.$ocxpe()
                            .set_bit() // Enable preload
                            .$ocxm()
                            .bits(0b110)} // PWM Mode
                    );

                    self.ccer_enable();
                }

                fn get_duty(&self) -> Self::Duty {
                    let tim = unsafe { &*$TIMX::ptr() };

                    // Even though the field is 20 bits long for 16-bit counters, only 16 bits are
                    // valid, so we convert to the appropriate type.
                    tim.$ccrx().read().ccr().bits()
                }

                fn get_max_duty(&self) -> Self::Duty {
                    let tim = unsafe { &*$TIMX::ptr() };

                    // Even though the field is 20 bits long for 16-bit counters, only 16 bits are
                    // valid, so we convert to the appropriate type.
                    let arr = tim.ar().read().ar().bits();

                    // One PWM cycle is ARR+1 counts long
                    // Valid PWM duty cycles are 0 to ARR+1
                    // However, if ARR is 65535 on a 16-bit timer, we can't add 1
                    // In that case, 100% duty cycle is not possible, only 65535/65536
                    if arr == Self::Duty::MAX {
                        arr
                    }
                    else {
                        arr + 1
                    }
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    let tim = unsafe { &*$TIMX::ptr() };

                    tim.$ccrx().write(|w| unsafe { w.ccr().bits(duty.into()) });
                }
            }

            // Enable implementation for ComplementaryImpossible
            impl<POL, NPOL> PwmPinEnable for Pwm<$TIMX, $CH, ComplementaryImpossible, POL, NPOL> {
                fn ccer_enable(&mut self) {
                    let tim = unsafe { &*$TIMX::ptr() };

                    tim.ccen().modify(|_, w| w.$ccxe().set_bit());
                }
                fn ccer_disable(&mut self) {
                    let tim = unsafe { &*$TIMX::ptr() };

                    tim.ccen().modify(|_, w| w.$ccxe().clear_bit());
                }
            }

            impl<COMP, NPOL> Pwm<$TIMX, $CH, COMP, ActiveHigh, NPOL> {
                pub fn into_active_low(self) -> Pwm<$TIMX, $CH, COMP, ActiveLow, NPOL> {
                    let tim = unsafe { &*$TIMX::ptr() };

                    tim.ccen().modify(|_, w| w.$ccxp().set_bit());

                    Pwm {
                        _channel: PhantomData,
                        _tim: PhantomData,
                        _complementary: PhantomData,
                        _polarity: PhantomData,
                        _npolarity: PhantomData,
                    }
                }
            }

            impl<COMP, NPOL> Pwm<$TIMX, $CH, COMP, ActiveLow, NPOL> {
                pub fn into_active_high(self) -> Pwm<$TIMX, $CH, COMP, ActiveHigh, NPOL> {
                    let tim = unsafe { &*$TIMX::ptr() };

                    tim.ccen().modify(|_, w| w.$ccxp().clear_bit());

                    Pwm {
                        _channel: PhantomData,
                        _tim: PhantomData,
                        _complementary: PhantomData,
                        _polarity: PhantomData,
                        _npolarity: PhantomData,
                    }
                }
            }

            // Complementary channels
            $(
                // Enable implementation for ComplementaryDisabled
                impl<POL, NPOL> PwmPinEnable for Pwm<$TIMX, $CH, ComplementaryDisabled, POL, NPOL> {
                    fn ccer_enable(&mut self) {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.ccen().modify(|_, w| w.$ccxe().set_bit());
                    }
                    fn ccer_disable(&mut self) {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.ccen().modify(|_, w| w.$ccxe().clear_bit());
                    }
                }

                // Enable implementation for ComplementaryEnabled
                impl<POL, NPOL> PwmPinEnable for Pwm<$TIMX, $CH, ComplementaryEnabled, POL, NPOL> {
                    fn ccer_enable(&mut self) {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.ccen().modify(|_, w| w.$ccxe().set_bit().$ccxne().set_bit());
                    }
                    fn ccer_disable(&mut self) {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.ccen().modify(|_, w| w.$ccxe().clear_bit().$ccxne().clear_bit());
                    }
                }

                impl<POL, NPOL> Pwm<$TIMX, $CH, ComplementaryDisabled, POL, NPOL> {
                    pub fn into_complementary<NPIN>(self, _npin: NPIN) -> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, NPOL>
                        where NPIN: NPins<$TIMX, $CH> {
                        // Make sure we aren't switching to complementary after we enable the channel
                        let tim = unsafe { &*$TIMX::ptr() };

                        let enabled = tim.ccen().read().$ccxe().bit();

                        assert!(!enabled);

                        Pwm {
                            _channel: PhantomData,
                            _tim: PhantomData,
                            _complementary: PhantomData,
                            _polarity: PhantomData,
                            _npolarity: PhantomData,
                        }
                    }
                }

                impl<POL> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveHigh> {
                    pub fn into_comp_active_low(self) -> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveLow> {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.ccen().modify(|_, w| w.$ccxnp().set_bit());

                        Pwm {
                            _channel: PhantomData,
                            _tim: PhantomData,
                            _complementary: PhantomData,
                            _polarity: PhantomData,
                            _npolarity: PhantomData,
                        }
                    }
                }

                impl<POL> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveLow> {
                    pub fn into_comp_active_high(self) -> Pwm<$TIMX, $CH, ComplementaryEnabled, POL, ActiveHigh> {
                        let tim = unsafe { &*$TIMX::ptr() };

                        tim.ccen().modify(|_, w| w.$ccxnp().clear_bit());

                        Pwm {
                            _channel: PhantomData,
                            _tim: PhantomData,
                            _complementary: PhantomData,
                            _polarity: PhantomData,
                            _npolarity: PhantomData,
                        }
                    }
                }
            )*
        )+
    };
}


// Quad channel timers
tim_pin_hal! {
    Tim1: (C1, cc1en, cc1p, ccmod1, oc1pen, oc1m, ccr1, u16, cc1nen, cc1np),
    Tim1: (C2, cc2en, cc2p, ccmod1, oc2pen, oc2m, ccr2, u16, cc2nen, cc2np),
    Tim1: (C3, cc3en, cc3p, ccmod2, oc3pen, oc3m, ccr3, u16, cc3nen, cc3np),
    Tim1: (C4, cc4en, cc4p, ccmod2, oc4pen, oc4m, ccr4, u16),
}
tim_pin_hal! {
    Tim2: (C1, cc1en, cc1p, ccmod1, oc1pen, oc1m, ccr1, u16),
    Tim2: (C2, cc2en, cc2p, ccmod1, oc2pen, oc2m, ccr2, u16),
    Tim2: (C3, cc3en, cc3p, ccmod2, oc3pen, oc3m, ccr3, u16),
    Tim2: (C4, cc4en, cc4p, ccmod2, oc4pen, oc4m, ccr4, u16),
}
tim_pin_hal! {
    Tim3: (C1, cc1en, cc1p, ccmod1, oc1pen, oc1m, ccr1, u16),
    Tim3: (C2, cc2en, cc2p, ccmod1, oc2pen, oc2m, ccr2, u16),
    Tim3: (C3, cc3en, cc3p, ccmod2, oc3pen, oc3m, ccr3, u16),
    Tim3: (C4, cc4en, cc4p, ccmod2, oc4pen, oc4m, ccr4, u16),
}
tim_pin_hal! {
    Tim4: (C1, cc1en, cc1p, ccmod1, oc1pen, oc1m, ccr1, u16),
    Tim4: (C2, cc2en, cc2p, ccmod1, oc2pen, oc2m, ccr2, u16),
    Tim4: (C3, cc3en, cc3p, ccmod2, oc3pen, oc3m, ccr3, u16),
    Tim4: (C4, cc4en, cc4p, ccmod2, oc4pen, oc4m, ccr4, u16),
}
tim_pin_hal! {
    Tim5: (C1, cc1en, cc1p, ccmod1, oc1pen, oc1m, ccr1, u16),
    Tim5: (C2, cc2en, cc2p, ccmod1, oc2pen, oc2m, ccr2, u16),
    Tim5: (C3, cc3en, cc3p, ccmod2, oc3pen, oc3m, ccr3, u16),
    Tim5: (C4, cc4en, cc4p, ccmod2, oc4pen, oc4m, ccr4, u16),
}
// Quad channel timers
tim_pin_hal! {
    Tim8: (C1, cc1en, cc1p, ccmod1, oc1pen, oc1m, ccr1, u16, cc1nen, cc1np),
    Tim8: (C2, cc2en, cc2p, ccmod1, oc2pen, oc2m, ccr2, u16, cc2nen, cc2np),
    Tim8: (C3, cc3en, cc3p, ccmod2, oc3pen, oc3m, ccr3, u16, cc3nen, cc3np),
    Tim8: (C4, cc4en, cc4p, ccmod2, oc4pen, oc4m, ccr4, u16),
}