
use core::marker::PhantomData;

use super::*;
use crate::gpio::{ Floating, NoPin};

pub struct Remapper<MODULE, PINS> {
    _mod : PhantomData<MODULE>,
    _pins : PhantomData<PINS>
}
pub trait RemapIO<PER, Remapper : Remap>  {
}

pub trait Remap {
    fn remap( afio : &mut crate::pac::AFIO);
}

impl<PER,Mapper> !RemapIO<PER,Mapper> for NoPin {
}

pub mod spi1 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, Floating, NoPin, PushPull};
    use crate::{gpio::alt::altmap::pin, pac::SPI1 as SPI};

    pub(crate) struct SPI1NoRemapRemapper();
    pub(crate) struct SPI1PartialRemapOneRemapper();
    pub(crate) struct SPI1PartialRemapTwoRemapper();
    pub(crate) struct SPI1FullRemapRemapper();

    impl Remap for SPI1NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.spi1_rmp_0().clear_bit());
            afio.rmp_cfg3().write(|w| w.spi1_rmp_1().clear_bit());
        }
    }
    impl Remap for SPI1PartialRemapOneRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.spi1_rmp_0().set_bit());
            afio.rmp_cfg3().write(|w| w.spi1_rmp_1().clear_bit());
        }
    }

    impl Remap for SPI1PartialRemapTwoRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.spi1_rmp_0().clear_bit());
            afio.rmp_cfg3().write(|w| w.spi1_rmp_1().set_bit());
        }
    }

    impl Remap for SPI1FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.spi1_rmp_0().set_bit());
            afio.rmp_cfg3().write(|w| w.spi1_rmp_1().set_bit());
        }
    }

    impl<T> RemapIO<SPI,SPI1NoRemapRemapper> for crate::gpio::PA4<T> {
    }
    impl<T> RemapIO<SPI,SPI1NoRemapRemapper> for crate::gpio::PA5<T> {
    }
    impl<T> RemapIO<SPI,SPI1NoRemapRemapper> for crate::gpio::PA6<T> {
    }
    impl<T> RemapIO<SPI,SPI1NoRemapRemapper> for crate::gpio::PA7<T> {
    }

    impl<T> RemapIO<SPI,SPI1PartialRemapOneRemapper> for crate::gpio::PA15<T> {
    }
    impl<T> RemapIO<SPI,SPI1PartialRemapOneRemapper> for crate::gpio::PB3<T> {
    }
    impl<T> RemapIO<SPI,SPI1PartialRemapOneRemapper> for crate::gpio::PB4<T> {
    }
    impl<T> RemapIO<SPI,SPI1PartialRemapOneRemapper> for crate::gpio::PB5<T> {
    }

    impl<T> RemapIO<SPI,SPI1PartialRemapTwoRemapper> for crate::gpio::PB2<T> {
    }
    impl<T> RemapIO<SPI,SPI1PartialRemapTwoRemapper> for crate::gpio::PA5<T> {
    }
    impl<T> RemapIO<SPI,SPI1PartialRemapTwoRemapper> for crate::gpio::PA6<T> {
    }
    impl<T> RemapIO<SPI,SPI1PartialRemapTwoRemapper> for crate::gpio::PA7<T> {
    }

    impl<T> RemapIO<SPI,SPI1FullRemapRemapper> for crate::gpio::PB2<T> {
    }
    impl<T> RemapIO<SPI,SPI1FullRemapRemapper> for crate::gpio::PE7<T> {
    }
    impl<T> RemapIO<SPI,SPI1FullRemapRemapper> for crate::gpio::PE8<T> {
    }
    impl<T> RemapIO<SPI,SPI1FullRemapRemapper> for crate::gpio::PE9<T> {
    }

    pin! {
        <Nss> default: PushPull for no:NoPin, [
            PA4,
            PA15,
            PB2,
        ],

        <Sck> default: PushPull for no:NoPin, [
            PA5,
            PB3,
            PE7,
        ],
        <Miso> default: Floating for no:NoPin, [
            PA6,
            PB4,
            PE8,
        ],

        <Mosi> default: PushPull for no:NoPin, [
            PA7,
            PB5,
            PE9,
        ],

    }

    impl SpiCommon for SPI {
        type Sck = Sck;
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
    }
}

pub mod spi2 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, Floating, NoPin, PushPull};
    use crate::{gpio::alt::altmap::pin, pac::SPI2 as SPI};

    pub(crate) struct SPI2NoRemapRemapper();
    pub(crate) struct SPI2PartialRemapRemapper();
    pub(crate) struct SPI2FullRemapRemapper();

    impl Remap for SPI2NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi2_rmp().bits(0b00)});
        }
    }

    impl Remap for SPI2PartialRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi2_rmp().bits(0b01)});
        }
    }

    impl Remap for SPI2FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi2_rmp().bits(0b11)});
        }
    }

    impl<T> RemapIO<SPI,SPI2NoRemapRemapper> for crate::gpio::PB12<T> {
    }
    impl<T> RemapIO<SPI,SPI2NoRemapRemapper> for crate::gpio::PB13<T> {
    }
    impl<T> RemapIO<SPI,SPI2NoRemapRemapper> for crate::gpio::PB14<T> {
    }
    impl<T> RemapIO<SPI,SPI2NoRemapRemapper> for crate::gpio::PB15<T> {
    }

    impl<T> RemapIO<SPI,SPI2PartialRemapRemapper> for crate::gpio::PC6<T> {
    }
    impl<T> RemapIO<SPI,SPI2PartialRemapRemapper> for crate::gpio::PC7<T> {
    }
    impl<T> RemapIO<SPI,SPI2PartialRemapRemapper> for crate::gpio::PC8<T> {
    }
    impl<T> RemapIO<SPI,SPI2PartialRemapRemapper> for crate::gpio::PC9<T> {
    }

    impl<T> RemapIO<SPI,SPI2FullRemapRemapper> for crate::gpio::PE10<T> {
    }
    impl<T> RemapIO<SPI,SPI2FullRemapRemapper> for crate::gpio::PE11<T> {
    }
    impl<T> RemapIO<SPI,SPI2FullRemapRemapper> for crate::gpio::PE12<T> {
    }
    impl<T> RemapIO<SPI,SPI2FullRemapRemapper> for crate::gpio::PE13<T> {
    }

    pin! {
        <Nss> default: PushPull for no:NoPin, [
            PB12,
            PC6,
            PE10,
        ],

        <Sck> default: PushPull for no:NoPin, [
            PB13,
            PC7,
            PE11,
        ],
        <Miso> default: Floating for no:NoPin, [
            PB14,
            PC8,
            PE12,
        ],

        <Mosi> default: PushPull for no:NoPin, [
            PB15,
            PC9,
            PE13,
        ],

    }

    impl SpiCommon for SPI {
        type Sck = Sck;
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
    }
}

pub mod spi3 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, Floating, NoPin, PushPull};
    use crate::{gpio::alt::altmap::pin, pac::SPI3 as SPI};

    pub(crate) struct SPI3NoRemapRemapper();
    pub(crate) struct SPI3PartialRemapOneRemapper();
    pub(crate) struct SPI3PartialRemapTwoRemapper();
    pub(crate) struct SPI3FullRemapRemapper();

    impl Remap for SPI3NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi3_rmp().bits(0b00)});
        }
    }

    impl Remap for SPI3PartialRemapOneRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi3_rmp().bits(0b01)});
        }
    }

    impl Remap for SPI3PartialRemapTwoRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi3_rmp().bits(0b10)});
        }
    }

    impl Remap for SPI3FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.spi3_rmp().bits(0b11)});
        }
    }

    impl<T> RemapIO<SPI,SPI3NoRemapRemapper> for crate::gpio::PA15<T> {
    }
    impl<T> RemapIO<SPI,SPI3NoRemapRemapper> for crate::gpio::PB3<T> {
    }
    impl<T> RemapIO<SPI,SPI3NoRemapRemapper> for crate::gpio::PB4<T> {
    }
    impl<T> RemapIO<SPI,SPI3NoRemapRemapper> for crate::gpio::PB5<T> {
    }

    impl<T> RemapIO<SPI,SPI3PartialRemapOneRemapper> for crate::gpio::PD2<T> {
    }
    impl<T> RemapIO<SPI,SPI3PartialRemapOneRemapper> for crate::gpio::PC10<T> {
    }
    impl<T> RemapIO<SPI,SPI3PartialRemapOneRemapper> for crate::gpio::PC11<T> {
    }
    impl<T> RemapIO<SPI,SPI3PartialRemapOneRemapper> for crate::gpio::PC12<T> {
    }

    impl<T> RemapIO<SPI,SPI3PartialRemapTwoRemapper> for crate::gpio::PD8<T> {
    }
    impl<T> RemapIO<SPI,SPI3PartialRemapTwoRemapper> for crate::gpio::PD9<T> {
    }
    impl<T> RemapIO<SPI,SPI3PartialRemapTwoRemapper> for crate::gpio::PD11<T> {
    }
    impl<T> RemapIO<SPI,SPI3PartialRemapTwoRemapper> for crate::gpio::PD12<T> {
    }

    impl<T> RemapIO<SPI,SPI3FullRemapRemapper> for crate::gpio::PC2<T> {
    }
    impl<T> RemapIO<SPI,SPI3FullRemapRemapper> for crate::gpio::PC3<T> {
    }
    impl<T> RemapIO<SPI,SPI3FullRemapRemapper> for crate::gpio::PA0<T> {
    }
    impl<T> RemapIO<SPI,SPI3FullRemapRemapper> for crate::gpio::PA1<T> {
    }

    pin! {
        <Nss> default: PushPull for no:NoPin, [
            PA15,
            PD2,
            PD8,
            PC2,
        ],

        <Sck> default: PushPull for no:NoPin, [
            PB3,
            PC10,
            PD9,
            PC3,
        ],
        <Miso> default: Floating for no:NoPin, [
            PB4,
            PC10,
            PD9,
            PA0,
        ],

        <Mosi> default: PushPull for no:NoPin, [
            PB5,
            PC12,
            PD12,
            PA1,
        ],

    }

    impl SpiCommon for SPI {
        type Sck = Sck;
        type Miso = Miso;
        type Mosi = Mosi;
        type Nss = Nss;
    }
}


pub mod usart1 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};

    pub(crate) struct USART1NoRemapRemapper();
    pub(crate) struct USART1FullRemapRemapper();

    impl Remap for USART1NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.usart1_rmp().clear_bit())
        }
    }

    impl Remap for USART1FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.usart1_rmp().clear_bit())
        }
    }

    impl RemapIO<USART,USART1NoRemapRemapper> for crate::gpio::PA9 {
    }
    impl RemapIO<USART,USART1NoRemapRemapper> for crate::gpio::PA10 {
    }


    impl RemapIO<USART,USART1FullRemapRemapper> for crate::gpio::PB6 {
    }
    impl RemapIO<USART,USART1FullRemapRemapper> for crate::gpio::PB7 {
    }

    // pin! {
    //     <Ck, PushPull> for [
    //         PA8,
    //     ],

    //     <Cts, PushPull> for [
    //         PA11,
    //     ],

    //     <Rts, PushPull> for [
    //         PA12,
    //     ],
    // }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PA10,
            PB7,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA9,
            PB6,
        ],
    }

    use crate::{gpio::alt::altmap::pin, pac::USART1 as USART};
    impl SerialAsync for USART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }

    // impl SerialSync for USART {
    //     type Rx<Itype> = Rx<Input<Itype>>;
    //     type Tx<Otype> = Tx<Otype>;
    //     type Ck = Ck;

    // }

    // impl SerialRs232 for USART {
    //     type Rx<Itype> = Rx<Input<Itype>>;
    //     type Tx<Otype> = Tx<Otype>;
    //     type Cts = Cts;
    //     type Rts = Rts;
    // }
}

pub mod usart2 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};
    use crate::{gpio::alt::altmap::pin, pac::USART2 as USART};

    pub(crate) struct USART2NoRemapRemapper();
    pub(crate) struct USART2PartialRemapOneRemapper();
    pub(crate) struct USART2PartialRemapTwoRemapper();
    pub(crate) struct USART2FullRemapRemapper();

    impl Remap for USART2NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.usart1_rmp().clear_bit())
        }
    }
    impl Remap for USART2PartialRemapOneRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.usart1_rmp().clear_bit())
        }
    }
    impl Remap for USART2PartialRemapTwoRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.usart1_rmp().clear_bit())
        }
    }
    impl Remap for USART2FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| w.usart1_rmp().clear_bit())
        }
    }

    impl<T> RemapIO<USART,USART2NoRemapRemapper> for crate::gpio::PA0<T> {
    }
    impl<T> RemapIO<USART,USART2NoRemapRemapper> for crate::gpio::PA1<T> {
    }
    impl<T> RemapIO<USART,USART2NoRemapRemapper> for crate::gpio::PA2<T> {
    }
    impl<T> RemapIO<USART,USART2NoRemapRemapper> for crate::gpio::PA3<T> {
    }
    impl<T> RemapIO<USART,USART2NoRemapRemapper> for crate::gpio::PA4<T> {
    }

    impl<T> RemapIO<USART,USART2PartialRemapOneRemapper> for crate::gpio::PD3<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapOneRemapper> for crate::gpio::PD4<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapOneRemapper> for crate::gpio::PD5<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapOneRemapper> for crate::gpio::PD6<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapOneRemapper> for crate::gpio::PD7<T> {
    }

    impl<T> RemapIO<USART,USART2PartialRemapTwoRemapper> for crate::gpio::PC6<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapTwoRemapper> for crate::gpio::PC7<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapTwoRemapper> for crate::gpio::PC8<T> {
    }
    impl<T> RemapIO<USART,USART2PartialRemapTwoRemapper> for crate::gpio::PC9<T> {
    }

    impl<T> RemapIO<USART,USART2FullRemapRemapper> for crate::gpio::PA15<T> {
    }
    impl<T> RemapIO<USART,USART2FullRemapRemapper> for crate::gpio::PB3<T> {
    }
    impl<T> RemapIO<USART,USART2FullRemapRemapper> for crate::gpio::PB4<T> {
    }
    impl<T> RemapIO<USART,USART2FullRemapRemapper> for crate::gpio::PB5<T> {
    }
    impl<T> RemapIO<USART,USART2FullRemapRemapper> for crate::gpio::PA4<T> {
    }

    // pin! {
    //     <Ck, PushPull> for [
    //         PA4,
    //         PD7,
    //     ],

    //     <Cts, PushPull> for [
    //         PA0,
    //         PD3,
    //         PC6,
    //         PA15,
    //     ],

    //     <Rts, PushPull> for [
    //         PA1,
    //         PD4,
    //         PC7,
    //         PB3,
    //     ],
    // }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PA3,
            PD6,
            PC9,
            PB5,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA2,
            PD5,
            PC8,
            PB4,
        ],
    }

    impl SerialAsync for USART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }

    // impl SerialSync for USART {
    //     type Rx<Itype> = Rx<Input<Itype>>;
    //     type Tx<Otype> = Tx<Otype>;
    //     type Ck = Ck;

    // }

    // impl SerialRs232 for USART {
    //     type Rx<Itype> = Rx<Input<Itype>>;
    //     type Tx<Otype> = Tx<Otype>;
    //     type Cts = Cts;
    //     type Rts = Rts;
    // }
}


pub mod usart3 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};
    use crate::{gpio::alt::altmap::pin, pac::USART3 as USART};

    pub(crate) struct USART3NoRemapRemapper();

    pub(crate) struct USART3PartialRemapRemapper();
    pub(crate) struct USART3FullRemapRemapper();

    impl Remap for USART3NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.usart3_rmp().bits(0)})
        }
    }

    impl Remap for USART3PartialRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.usart3_rmp().bits(1)})
        }
    }

    impl Remap for USART3FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.usart3_rmp().bits(3)})
        }
    }

    impl<T> RemapIO<USART,USART3NoRemapRemapper> for crate::gpio::PB10<T> {
    }
    impl<T> RemapIO<USART,USART3NoRemapRemapper> for crate::gpio::PB11<T> {
    }
    impl<T> RemapIO<USART,USART3NoRemapRemapper> for crate::gpio::PB12<T> {
    }
    impl<T> RemapIO<USART,USART3NoRemapRemapper> for crate::gpio::PB13<T> {
    }
    impl<T> RemapIO<USART,USART3NoRemapRemapper> for crate::gpio::PB14<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PC10<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PC11<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PC12<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PB13<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PB14<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PD8<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PD9<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PD10<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PD11<T> {
    }
    impl<T> RemapIO<USART,USART3PartialRemapRemapper> for crate::gpio::PD12<T> {
    }
    // pin! {
    //     <Ck, PushPull> for [
    //         PB12,
    //         PC12,
    //         PD10,
    //     ],

    //     <Cts, PushPull> for [
    //         PB13,
    //         PD11,
    //     ],

    //     <Rts, PushPull> for [
    //         PB14,
    //         PD12,
    //     ],
    // }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PB10,
            PC10,
            PD8,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PB11,
            PC11,
            PD10,
        ],
    }

    impl SerialAsync for USART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }

    // impl SerialSync for USART {
    //     type Rx<Itype> = Rx<Input<Itype>>;
    //     type Tx<Otype> = Tx<Otype>;
    //     type Ck = Ck;

    // }

    // impl SerialRs232 for USART {
    //     type Rx<Itype> = Rx<Input<Itype>>;
    //     type Tx<Otype> = Tx<Otype>;
    //     type Cts = Cts;
    //     type Rts = Rts;
    // }
}


pub mod uart4 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};
    use crate::{gpio::alt::altmap::pin, pac::UART4 as UART};

    pub struct UART4NoRemapRemapper();
    pub struct UART4PartialRemapOneRemapper();
    pub struct UART4PartialRemapTwoRemapper();
    pub struct UART4FullRemapRemapper();

    impl Remap for UART4NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart4_rmp().bits(0)})
        }
    }

    impl Remap for UART4PartialRemapOneRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart4_rmp().bits(1)})
        }
    }

    impl Remap for UART4PartialRemapTwoRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart4_rmp().bits(2)})
        }
    }

    impl Remap for UART4FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart4_rmp().bits(3)})
        }
    }

    impl<T> RemapIO<UART,UART4NoRemapRemapper> for crate::gpio::PC10<T> {
    }
    impl<T> RemapIO<UART,UART4NoRemapRemapper> for crate::gpio::PC11<T> {
    }
    impl<T> RemapIO<UART,UART4PartialRemapOneRemapper> for crate::gpio::PB2<T> {
    }
    impl<T> RemapIO<UART,UART4PartialRemapOneRemapper> for crate::gpio::PE7<T> {
    }
    impl<T> RemapIO<UART,UART4PartialRemapTwoRemapper> for crate::gpio::PA13<T> {
    }
    impl<T> RemapIO<UART,UART4PartialRemapTwoRemapper> for crate::gpio::PA14<T> {
    }
    impl<T> RemapIO<UART,UART4FullRemapRemapper> for crate::gpio::PD0<T> {
    }
    impl<T> RemapIO<UART,UART4FullRemapRemapper> for crate::gpio::PD1<T> {
    }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PC11,
            PE7,
            PA14,
            PD1,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PC10,
            PB2,
            PA13,
            PD0,
        ],
    }


    impl SerialAsync for UART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }
}

pub mod uart5 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};
    use crate::{gpio::alt::altmap::pin, pac::UART5 as UART};

    pub(crate) struct UART5NoRemapRemapper();
    pub(crate) struct UART5PartialRemapOneRemapper();
    pub(crate) struct UART5PartialRemapTwoRemapper();
    pub(crate) struct UART5FullRemapRemapper();

    impl Remap for UART5NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart5_rmp().bits(0)})
        }
    }

    impl Remap for UART5PartialRemapOneRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart5_rmp().bits(1)})
        }
    }

    impl Remap for UART5PartialRemapTwoRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart5_rmp().bits(2)})
        }
    }

    impl Remap for UART5FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart5_rmp().bits(3)})
        }
    }

    impl<T> RemapIO<UART,UART5NoRemapRemapper> for crate::gpio::PC12<T> {
    }
    impl<T> RemapIO<UART,UART5NoRemapRemapper> for crate::gpio::PD2<T> {
    }
    impl<T> RemapIO<UART,UART5PartialRemapOneRemapper> for crate::gpio::PB13<T> {
    }
    impl<T> RemapIO<UART,UART5PartialRemapOneRemapper> for crate::gpio::PB14<T> {
    }
    impl<T> RemapIO<UART,UART5PartialRemapTwoRemapper> for crate::gpio::PE8<T> {
    }
    impl<T> RemapIO<UART,UART5PartialRemapTwoRemapper> for crate::gpio::PE9<T> {
    }
    impl<T> RemapIO<UART,UART5FullRemapRemapper> for crate::gpio::PB8<T> {
    }
    impl<T> RemapIO<UART,UART5FullRemapRemapper> for crate::gpio::PB9<T> {
    }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PD2,
            PB14,
            PE9,
            PB9,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PC12,
            PB13,
            PE8,
            PB8,
        ],
    }

    impl SerialAsync for UART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }
}

pub mod uart6 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};
    use crate::{gpio::alt::altmap::pin, pac::UART6 as UART};

    pub(crate) struct UART6NoRemapRemapper();
    pub(crate) struct UART6PartialRemapRemapper();
    pub(crate) struct UART6FullRemapRemapper();

    impl Remap for UART6NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart6_rmp().bits(0)})
        }
    }

    impl Remap for UART6PartialRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart6_rmp().bits(1)})
        }
    }

    impl Remap for UART6FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart6_rmp().bits(3)})
        }
    }

    impl<T> RemapIO<UART,UART6NoRemapRemapper> for crate::gpio::PE2<T> {
    }
    impl<T> RemapIO<UART,UART6NoRemapRemapper> for crate::gpio::PE3<T> {
    }
    impl<T> RemapIO<UART,UART6PartialRemapRemapper> for crate::gpio::PC0<T> {
    }
    impl<T> RemapIO<UART,UART6PartialRemapRemapper> for crate::gpio::PC1<T> {
    }
    impl<T> RemapIO<UART,UART6FullRemapRemapper> for crate::gpio::PB0<T> {
    }
    impl<T> RemapIO<UART,UART6FullRemapRemapper> for crate::gpio::PB1<T> {
    }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PE3,
            PC1,
            PB1,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PE2,
            PC0,
            PB0,
        ],
    }

    impl SerialAsync for UART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }
}

pub mod uart7 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull,Input};
    use crate::{gpio::alt::altmap::pin, pac::UART7 as UART};

    pub(crate) struct UART7NoRemapRemapper();
    pub(crate) struct UART7PartialRemapRemapper();
    pub(crate) struct UART7FullRemapRemapper();

    impl Remap for UART7NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart7_rmp().bits(0)})
        }
    }

    impl Remap for UART7PartialRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart7_rmp().bits(1)})
        }
    }

    impl Remap for UART7FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg3().write(|w| unsafe { w.uart7_rmp().bits(3)})
        }
    }

    impl RemapIO<UART,UART7NoRemapRemapper> for crate::gpio::PC12 {
    }
    impl RemapIO<UART,UART7NoRemapRemapper> for crate::gpio::PD2 {
    }
    impl RemapIO<UART,UART7PartialRemapRemapper> for crate::gpio::PB13 {
    }
    impl RemapIO<UART,UART7PartialRemapRemapper> for crate::gpio::PB14 {
    }
    impl RemapIO<UART,UART7FullRemapRemapper> for crate::gpio::PB8 {
    }
    impl RemapIO<UART,UART7FullRemapRemapper> for crate::gpio::PB9 {
    }

    pin! {
        <Rx> default: Floating for no:NoPin, [
            PC5,
            PC3,
            PG1,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PC4,
            PC2,
            PG0,
        ],
    }

    impl SerialAsync for UART {
        type Rx<Itype> = Rx<Input<Itype>>;
        type Tx<Otype> = Tx<Otype>;
    }
}


pub mod tim2 {
    use self::altmap::{Remap, RemapIO};
    use super::*;
    use crate::gpio::{self, NoPin, PushPull};
    use crate::{gpio::alt::altmap::pin, pac::TIM2 as TIM};

    pub(crate) struct TIM2NoRemapRemapper();
    pub(crate) struct TIM2PartialRemapOneRemapper();

    pub(crate) struct TIM2PartialRemapTwoRemapper();

    pub(crate) struct TIM2FullRemapRemapper();

    impl Remap for TIM2NoRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.tim2_rmp().bits(0)})
        }
    }

    impl Remap for TIM2PartialRemapOneRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.tim2_rmp().bits(1)})
        }
    }

    impl Remap for TIM2PartialRemapTwoRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.tim2_rmp().bits(2)})
        }
    }


    impl Remap for TIM2FullRemapRemapper {
        fn remap( afio : &mut crate::pac::AFIO) {
            afio.rmp_cfg().write(|w| unsafe { w.tim2_rmp().bits(3)})
        }
    }

    impl RemapIO<TIM,TIM2NoRemapRemapper> for crate::gpio::PA0 {
    }
    impl RemapIO<TIM,TIM2NoRemapRemapper> for crate::gpio::PA1 {
    }
    impl RemapIO<TIM,TIM2NoRemapRemapper> for crate::gpio::PA2 {
    }
    impl RemapIO<TIM,TIM2NoRemapRemapper> for crate::gpio::PA3 {
    }

    impl RemapIO<TIM,TIM2PartialRemapOneRemapper> for crate::gpio::PA15 {
    }
    impl RemapIO<TIM,TIM2PartialRemapOneRemapper> for crate::gpio::PB3 {
    }
    impl RemapIO<TIM,TIM2PartialRemapOneRemapper> for crate::gpio::PA2 {
    }
    impl RemapIO<TIM,TIM2PartialRemapOneRemapper> for crate::gpio::PA3 {
    }

    impl RemapIO<TIM,TIM2PartialRemapTwoRemapper> for crate::gpio::PA0 {
    }
    impl RemapIO<TIM,TIM2PartialRemapTwoRemapper> for crate::gpio::PA1 {
    }
    impl RemapIO<TIM,TIM2PartialRemapTwoRemapper> for crate::gpio::PB10 {
    }
    impl RemapIO<TIM,TIM2PartialRemapTwoRemapper> for crate::gpio::PB11 {
    }

    impl RemapIO<TIM,TIM2FullRemapRemapper> for crate::gpio::PA15 {
    }
    impl RemapIO<TIM,TIM2FullRemapRemapper> for crate::gpio::PB3 {
    }
    impl RemapIO<TIM,TIM2FullRemapRemapper> for crate::gpio::PB10 {
    }
    impl RemapIO<TIM,TIM2FullRemapRemapper> for crate::gpio::PB11 {
    }

    pin! {
        <Ch1> default: PushPull for no:NoPin, [
            PA0,
            PA15,
        ],

        <Ch2> default: PushPull for no:NoPin, [
            PA1,
            PB3,
        ],

        <Ch3> default: PushPull for no:NoPin, [
            PA2,
            PB10,
        ],

        <Ch4> default: PushPull for no:NoPin, [
            PA3,
            PB11,
        ],
    }

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }

    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }

    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }

    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
}
