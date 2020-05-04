use stm32f1xx_hal::stm32;
use core::fmt::{Write, Error};
use core::str;

/// init usart2 to send message
pub fn usart2_init(
    rcc: &mut stm32::RCC,
    gpioa: &mut stm32::GPIOA,
    usart2: &mut stm32::USART2,
) {
    rcc.apb1enr.modify(|_r, w| w.usart2en().set_bit());
    rcc.apb2enr.modify(|_r, w| w.iopaen().set_bit());

    gpioa.crl.modify(|_r, w| {
        w.mode2().output50().cnf2().alt_push_pull()
    });

    usart2.brr.write(|w| w.div_mantissa().bits(19).div_fraction().bits(8));
    usart2.cr1.write(|w| w.ue().set_bit().te().set_bit().re().set_bit());
}

pub struct USART2;

/// impl Write for USART2, can use write! and writeln!
impl Write for USART2 {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let usart2 = unsafe { &*stm32::USART2::ptr() };

        for ch in s.bytes() {
            while usart2.sr.read().txe().bit_is_clear() {}
            unsafe {
                usart2.dr.write(|w| w.bits(ch as u32));
            }
        }

        Ok(())
    }
}