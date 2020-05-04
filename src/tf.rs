/// only for FAT format
use stm32f1xx_hal::stm32;
use embedded_sdmmc::{TimeSource, Timestamp};

#[derive(Debug)]
pub enum NeverError {}

/// init spi1 to make card work
pub fn spi1_init(
    rcc: &mut stm32::RCC,
    gpioa: &mut stm32::GPIOA,
    spi1: &mut stm32::SPI1,
) {
    rcc.apb2enr.modify(|_r, w| w.spi1en().set_bit());
    rcc.apb2enr.modify(|_r, w| w.iopaen().set_bit());

    gpioa.crl.modify(|_r, w|
        w.mode4().output50().cnf4().push_pull()
            .mode5().output50().cnf5().alt_push_pull()
            .mode6().output50().cnf6().alt_push_pull()
            .mode7().output50().cnf7().alt_push_pull());

    spi1.cr1.write(|w|
        w.bidimode().clear_bit()
            .rxonly().clear_bit()
            .ssm().set_bit()
            .ssi().set_bit()
            .lsbfirst().clear_bit()
            .mstr().set_bit()
            .cpha().set_bit()
            .cpol().set_bit()
            .spe().set_bit()
    );
}

pub struct SPI1;

/// impl send and read for SPI1
impl embedded_hal::spi::FullDuplex<u8> for SPI1 {
    type Error = NeverError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let spi1 = unsafe { &*stm32::SPI1::ptr() };
        while spi1.sr.read().rxne().bit_is_clear() {}
        Ok(spi1.dr.read().bits() as u8)
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let spi1 = unsafe { &*stm32::SPI1::ptr() };
        while spi1.sr.read().txe().bit_is_clear() {}
        unsafe {
            spi1.dr.write(|w| w.bits(word as u32));
        }
        Ok(())
    }
}

pub struct CS;

/// impl OutputPin for CS
impl embedded_hal::digital::v2::OutputPin for CS {
    type Error = NeverError;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let ptr = unsafe { &*stm32::GPIOA::ptr() };
        ptr.bsrr.write(|w| w.br4().set_bit());
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let ptr = unsafe { &*stm32::GPIOA::ptr() };
        ptr.bsrr.write(|w| w.bs4().set_bit());
        Ok(())
    }
}

pub struct Time;

/// impl TimeSource for Time
impl TimeSource for Time {
    /// can be free edited
    fn get_timestamp(&self) -> Timestamp {
        Timestamp::from_calendar(2000,1,1,1,1,1).unwrap()
    }
}