use stm32f4xx_hal::stm32;

/// KEY0 init (PE4)
pub fn init(rcc: &mut stm32::RCC, gpioe: &mut stm32::GPIOE) {
    rcc.ahb1enr.modify(|_r, w| w.gpioeen().set_bit());
    rcc.apb2enr.modify(|_r, w| w.syscfgen().set_bit());

    gpioe.moder.modify(|_r, w| w.moder4().input());
    gpioe.pupdr.modify(|_r, w| w.pupdr4().pull_up());
}

/// enable EXTI interrupt
pub fn enable_interrupt() {
    let syscfg_ptr = unsafe { &*stm32::SYSCFG::ptr() };
    let exti_ptr = unsafe { &*stm32::EXTI::ptr() };

    syscfg_ptr.exticr2.modify(|_r, w| unsafe { w.exti4().bits(0b0100) });
    exti_ptr.ftsr.modify(|_r, w| w.tr4().set_bit());
    exti_ptr.imr.write(|w| w.mr4().set_bit())
}

/// disable EXTI interrupt
pub fn disable_interrupt() {
    let syscfg_ptr = unsafe { &*stm32::SYSCFG::ptr() };
    let exti_ptr = unsafe { &*stm32::EXTI::ptr() };

    syscfg_ptr.exticr2.modify(|_r, w| unsafe { w.exti4().bits(0) });
    exti_ptr.ftsr.modify(|_r, w| w.tr4().clear_bit());
    exti_ptr.imr.write(|w| w.mr4().clear_bit());
    clean_interrupt_flag();
}

/// clean EXTI interrupt flag
fn clean_interrupt_flag() {
    let ptr = unsafe { &*stm32::EXTI::ptr() };

    ptr.pr.write(|w| w.pr4().set_bit());
}