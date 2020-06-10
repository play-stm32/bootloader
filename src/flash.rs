use stm32f4xx_hal::stm32;

/// unlock fpec, make flash can be written
fn unlock_fpec() {
    let ptr = unsafe { &*stm32::FLASH::ptr() };

    ptr.keyr.write(|w| w.key().bits(0x45670123));
    ptr.keyr.write(|w| w.key().bits(0xCDEF89AB));
}

/// lock flash, then flash can't be edited
fn lock() {
    let ptr = unsafe { &*stm32::FLASH::ptr() };

    wait_free();
    ptr.cr.modify(|_r, w| w.lock().set_bit());
}

/// erase firmware flash
pub fn erase(start_sector: u8, end_sector: u8) {
    unlock_fpec();
    let ptr = unsafe { &*stm32::FLASH::ptr() };

    for sector in start_sector..=end_sector {
        wait_free();
        ptr.cr.modify(|_r, w| w.ser().set_bit());
        ptr.cr.modify(|_r, w| unsafe { w.snb().bits(sector) });
        ptr.cr.modify(|_r, w| w.strt().set_bit());
    }

    lock();
}

/// write to firmware flash
pub fn write(mut address: usize, buf: &[u8]) {
    unlock_fpec();
    for i in 0..buf.len() {
        write_byte(address, buf[i]);
        address += 0x1;
    }
    lock();
}

/// write to flash per byte
fn write_byte(address: usize, data: u8) {
    let ptr = unsafe { &*stm32::FLASH::ptr() };

    wait_free();
    ptr.cr.write(|w| w.pg().set_bit());
    unsafe { *(address as *mut u8) = data; }
}

/// wait flash free
fn wait_free() {
    let ptr = unsafe { &*stm32::FLASH::ptr() };
    while !ptr.sr.read().bsy().bit_is_clear() {}
}