use stm32f4xx_hal::stm32;

/// init rcc clock
pub fn clock_init(rcc: &mut stm32::RCC, flash: &mut stm32::FLASH) {
    // enable hse
    // enable hsi to make flash can be written
    rcc.cr.modify(|_r, w| w.hseon().set_bit().hsion().set_bit());

    // wait until hse is stable
    // wait until hsi is stable
    while rcc.cr.read().hserdy().bit_is_clear() {}
    while rcc.cr.read().hsirdy().bit_is_clear() {}

    // AHB = SYSCLK / 1 = 168MHz(MAX)
    // APB1 = SYSCLK / 4 = 42MHz(MAX)
    // APB2 = SYSCLK / 2 = 84MHz(MAX)
    rcc.cfgr.modify(|_r, w| w.hpre().div1().ppre1().div4().ppre2().div2());

    // VOC(input) = PLL(input) / PLLM(8) = 1MHz
    // VOC(output) = VOC(input) * PLLN(336) = 336MHz
    // PLL(output) = VOC(output) / PLLP(2) = 168MHz
    // PLL2(output) = VOC(output) / PLLQ(7) = 48MHz
    rcc.pllcfgr.modify(|_r, w| unsafe {
        w.pllsrc().set_bit().pllm().bits(8).plln().bits(336).pllp().div2().pllq().bits(7)
    });

    // HCLK = 168MHz, set latency ws5, enable prefetch
    flash.acr.modify(|_r, w| w.latency().ws5().prften().set_bit());

    // pll enable
    rcc.cr.modify(|_r, w| w.pllon().set_bit().plli2son().set_bit());

    // wait until pll is stable
    while rcc.cr.read().pllrdy().bit_is_clear() {}
    while rcc.cr.read().plli2son().bit_is_clear() {}

    // switch sysclk to pll
    rcc.cfgr.modify(|_r, w| w.sw().pll());

    // wait until sysclk is stable, 0b02 = 2
    while rcc.cfgr.read().sws().bits() != 2 {}
}