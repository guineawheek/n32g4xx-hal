use crate::dma::DMAChannel;
macro_rules! chmap_setup {
    (
        $(
        $PER:ty: (
            $dmaunit:tt::($($dmach:ident$(,)*)+) => (R => $rmp_rx:expr, W => $rmp_tx:expr)
        ),
        )+
    ) => {
        $(
            $(
                impl crate::dma::CompatibleChannel<$PER,crate::dma::R> for crate::dma::$dmaunit::$dmach {
                    fn configure_channel(&mut self) {
                        unsafe { self.st().chsel().modify(|_,w| w.ch_sel().bits($rmp_rx)) }
                    }
                }
    
                impl crate::dma::CompatibleChannel<$PER,crate::dma::W> for crate::dma::$dmaunit::$dmach {
                    fn configure_channel(&mut self) {
                        unsafe { self.st().chsel().modify(|_,w| w.ch_sel().bits($rmp_tx)) }
                    }
                }
            )+
        )+
    }
}

//ADCs
chmap_setup!(
    crate::pac::ADC1: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 10, W => 15)),
    crate::pac::ADC2: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 10, W => 15)),
    crate::pac::ADC3: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 10, W => 15)),
    crate::pac::ADC4: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 10, W => 15)),
);

//US?ARTs
chmap_setup!(
    crate::pac::USART1: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 23, W => 16)),
    crate::pac::USART2: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 29, W => 34)),
    crate::pac::USART3: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 11, W => 5)),
    crate::pac::UART4: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 14, W => 24)),
    crate::pac::UART5: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 40, W => 1)),
    crate::pac::UART6: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 14, W => 12)),
    crate::pac::UART7: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 27, W => 30)),
);

//I2Cs
chmap_setup!(
    crate::pac::I2C1: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 38, W => 33)),
    crate::pac::I2C2: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 28, W => 22)),
    crate::pac::I2C3: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 6, W => 2)),
);

//SPIs
chmap_setup!(
    crate::pac::SPI1: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 10, W => 15)),
    crate::pac::SPI2: (dma1::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 21, W => 25)),
    crate::pac::SPI3: (dma2::(C1,C2,C3,C4,C5,C6,C7,C8) => (R => 4, W => 11)),
);

