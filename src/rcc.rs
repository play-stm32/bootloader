use stm32f1xx_hal::stm32;

/// init rcc clock
/// init hsi to make flash can be written
pub fn clock_init(rcc: &mut stm32::RCC, flash: &mut stm32::FLASH) {
    rcc.cr.write(|w| w.hseon().set_bit().hsion().set_bit());
    while !rcc.cr.read().hserdy().is_ready() {}
    while !rcc.cr.read().hsirdy().is_ready() {}
    rcc.cfgr.write(|w| w.hpre().div1().ppre1().div2().ppre2().div1());
    rcc.cfgr.write(|w| w.pllxtpre().div1().pllmul().mul9());
    flash.acr.write(|w| w.latency().ws2().prftbe().set_bit());
    rcc.cr.write(|w| w.pllon().set_bit());
    while rcc.cr.read().pllrdy().bit_is_clear() {}
    rcc.cfgr.write(|w| w.sw().pll());
    while rcc.cfgr.read().sws().bits() != 2 {}
}

/// disable clock
pub fn clock_disable(rcc: &mut stm32::RCC) {
    rcc.cr.write(|w| w.hseon().clear_bit().hsion().clear_bit());
}