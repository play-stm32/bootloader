use stm32f4xx_hal::stm32;

/// init tim2
pub fn init(rcc: &mut stm32::RCC, tim: &mut stm32::TIM2) {
    rcc.apb1enr.modify(|_r, w| w.tim2en().set_bit());

    // 84MHz / 8400 = 10KHz
    // 1 / 10KHz * 10000 = 1
    tim.psc.write(|w| w.psc().bits(8400));
    tim.arr.write(|w| w.arr().bits(10000));

    // load arr, update request source
    tim.cr1.modify(|_r, w| w.arpe().set_bit().urs().set_bit());

    // update interrupt enable
    tim.dier.modify(|_r, w| w.uie().set_bit());

    // update generation
    tim.egr.write(|w| w.ug().set_bit());
}

/// enable tim count
pub fn enable_count() {
    let ptr = unsafe { &*stm32::TIM2::ptr() };

    // enable
    ptr.cr1.modify(|_r, w| w.cen().set_bit());
}

/// clean flag
pub fn clean_interrupt_flag() {
    let ptr = unsafe { &*stm32::TIM2::ptr() };

    // clean update interrupt flag
    ptr.sr.modify(|_r, w| w.uif().clear_bit());
}

/// disable tim count
pub fn disable_count() {
    let ptr = unsafe { &*stm32::TIM2::ptr() };

    // disable
    ptr.cr1.modify(|_r, w| w.cen().clear_bit());
}