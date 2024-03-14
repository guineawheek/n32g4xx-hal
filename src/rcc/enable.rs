use super::*;
use crate::bb;

macro_rules! bus_enable {
    ($PER:ident => $bit:literal) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::pclken(rcc), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::pclken(rcc), $bit);
                }
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                let rcc = pac::RCC::ptr();
                (Self::Bus::pclken(unsafe { &*rcc }).read().bits() >> $bit) & 0x1 != 0
            }
        }
    };
}

macro_rules! bus_reset {
    ($PER:ident => $bit:literal) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::prst(rcc), $bit);
                    bb::clear(Self::Bus::prst(rcc), $bit);
                }
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $bit:literal),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $bit);
            bus_reset!($PER => $bit);
        )+
    }
}



#[cfg(any(feature = "n32g401", feature = "n32g430"))]
bus! {
    GPIOA => (AHB, 7),
    GPIOB => (AHB, 8),
    GPIOC => (AHB, 9),
    GPIOD => (AHB, 10),
}

#[cfg(not(any(feature = "n32g401", feature = "n32g430")))]
bus! {
    GPIOA => (APB2, 2),
    GPIOB => (APB2, 3),
    GPIOC => (APB2, 4),
    GPIOD => (APB2, 5),
}

#[cfg(any(feature = "n32g451", feature = "n32g452", feature = "n32g455", feature = "n32g457", feature = "n32g4fr"))]
bus! {
    GPIOE => (APB2, 6),
    GPIOF => (APB2, 7),
    GPIOG => (APB2, 8),
}

// TODO: RNG/SAC Abstraction.
// #[cfg(not(any(feature = "n32g401", feature = "n32g430")))]
// bus! {
//     SAC => (AHB, 11),
//     RNG => (AHB, 9),
// }


bus! {
    CRC => (AHB, 6),
    ADC1 => (AHB, 12),
    DMA1 => (AHB, 0),
}

bus! {
    PWR => (APB1, 28),
    I2C2 => (APB1, 22),
    I2C1 => (APB1, 21),
    USART2 => (APB1, 17),
    WWDG => (APB1, 11),
    COMP => (APB1, 6),
    TIM6 => (APB1, 4),
    TIM5 => (APB1, 3),
    TIM4 => (APB1, 2),
    TIM3 => (APB1, 1),
    TIM2 => (APB1, 0),
    
}

bus! {
    USART1 => (APB2, 14),
    TIM8 => (APB2, 13),
    SPI1 => (APB2, 12),
    TIM1 => (APB2, 11),
    AFIO => (APB2, 0),
}


#[cfg(any(feature = "n32g452",feature = "n32g455",feature = "n32g457",feature = "n32g4fr"))]
bus! {
    QSPI => (AHB, 17),

}

#[cfg(feature = "n32g457")]
bus! {
    ETH => (AHB, 16),
}

#[cfg(any(feature = "n32g451",feature = "n32g452",feature = "n32g455",feature = "n32g457",feature = "n32g4fr"))]
bus! {
    ADC4 => (AHB, 15),
    ADC3 => (AHB, 14),
    ADC2 => (AHB, 13),
    SDIO => (AHB, 10),
    DMA2 => (AHB, 1),
}


#[cfg(any(feature = "n32g451",feature = "n32g452",feature = "n32g455",feature = "n32g457",feature = "n32g4fr"))]
bus! {
    I2C4 => (APB2, 20),
    I2C3 => (APB2, 19),
    UART7 => (APB2, 18),
    UART6 => (APB2, 17),
}

#[cfg(any(feature = "n32g401",feature = "n32g430",feature = "n32g432",feature = "n32g435"))]
bus! {
    SPI2 => (APB2, 19),
}

#[cfg(any(feature = "n32g401",feature = "n32g430"))]
bus! {
    UART4 => (APB2, 18),
    UART3 => (APB2, 17),
    BEEP => (APB2, 1),
}

#[cfg(any(feature = "n32g432",feature = "n32g435"))]
bus! {
    SPI2 => (APB2, 19),
    UART5 => (APB2, 18),
    UART4 => (APB2, 17),
}

#[cfg(any(feature = "n32g435",feature = "n32g455",feature = "n32g457"))]
bus! {
    OPAMP => (APB1, 31),
}


#[cfg(not(any(feature = "n32g401",feature = "n32g430")))]
bus! {
    DAC => (APB1, 29),
    USB => (APB1, 23),
    USART3 => (APB1, 18),
    TSC => (APB1, 10),
    TIM7 => (APB1, 5),
}

#[cfg(any(feature = "n32g451",feature = "n32g452",feature = "n32g455",feature = "n32g457",feature = "n32g4fr"))]
bus! {
    BKP => (APB1, 27),
    UART5 => (APB1, 20),
    UART4 => (APB1, 19),
    SPI3 => (APB1, 15),
    SPI2 => (APB1, 14),
}

#[cfg(any(feature = "n32g452",feature = "n32g455",feature = "n32g457",feature = "n32g4fr"))]
bus! {
    CAN2 => (APB1, 26),
}

#[cfg(not(any(feature = "n32g401")))]
bus! {
    CAN1 => (APB1, 25),
}

#[cfg(any(feature = "n32g401",feature = "n32g430"))]
bus! {
    TIM9 => (APB1, 9),
    AFEC => (APB1, 8),
}