use stm32f4xx_hal::stm32;
use core::fmt::{Write, Error};
use core::str;

/// init usart1 to send message
pub fn init(
    rcc: &mut stm32::RCC,
    gpioa: &mut stm32::GPIOA,
    usart1: &mut stm32::USART1,
) {
    rcc.apb2enr.modify(|_r, w| w.usart1en().set_bit());
    rcc.ahb1enr.modify(|_r, w| w.gpioaen().set_bit());

    // PA9(Tx) alternate push
    gpioa.afrh.write(|w| w.afrh9().af7());
    gpioa.moder.modify(|_r, w| w.moder9().alternate());
    gpioa.ospeedr.modify(|_r, w| w.ospeedr9().high_speed());
    gpioa.pupdr.modify(|_r, w| w.pupdr9().pull_up());
    gpioa.otyper.modify(|_r, w| w.ot9().push_pull());

    // configurate usart baudrate
    // USARTDIV = FCLK (PCLK2 for USART1) / baudrate / 16
    //          = 84M / 115200 / 16 = 45.57291
    // DIV_MANTISSA = USARTDIV (integer part)
    //              = 45
    // DIV_FRACTION = USARTDIV (fraction part) * 16
    //              = 0.57291 * 16 = 9.16656
    usart1.brr.write(|w| w.div_mantissa().bits(45).div_fraction().bits(9));
    usart1.cr1.write(|w| w.ue().set_bit().te().set_bit());
}

pub struct USART1;

/// impl Write for USART1, can use write! and writeln!
impl Write for USART1 {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let ptr = unsafe { &*stm32::USART1::ptr() };

        for ch in s.bytes() {
            while ptr.sr.read().txe().bit_is_clear() {}
            unsafe {
                ptr.dr.write(|w| w.bits(ch as u32));
            }
        }

        Ok(())
    }
}