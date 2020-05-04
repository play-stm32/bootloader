use stm32f1xx_hal::stm32;
use crate::{OS_START_ADDRESS, OS_END_ADDRESS, FLASH_START_ADDRESS, FLASH_PAGE_SIZE, FIRMWARE_SIZE};

/// unlock fpec, make flash can be write
fn unlock_fpec() {
    let ptr = unsafe {
        &*stm32::FLASH::ptr()
    };

    ptr.keyr.write(|w| unsafe { w.key().bits(0x45670123) });
    ptr.keyr.write(|w| unsafe { w.key().bits(0xCDEF89AB) });
}

/// lock flash, then flash can't be edited
fn lock() {
    let ptr = unsafe {
        &*stm32::FLASH::ptr()
    };

    wait_free();
    ptr.cr.modify(|_r, w| w.lock().set_bit());
}

/// erase firmware flash
pub fn erase() {
    unlock_fpec();
    let ptr = unsafe {
        &*stm32::FLASH::ptr()
    };

    let start_page = (OS_START_ADDRESS - FLASH_START_ADDRESS) / FLASH_PAGE_SIZE;
    let end_page = (OS_END_ADDRESS + 1 - FLASH_START_ADDRESS) / FLASH_PAGE_SIZE;

    for page in start_page..end_page {
        wait_free();
        ptr.ar.write(|w| unsafe { w.far().bits((FLASH_START_ADDRESS + FLASH_PAGE_SIZE * page) as u32) });
        ptr.cr.modify(|_r, w| w.per().set_bit().strt().set_bit());
    }

    lock();
}

/// write to firmware flash
pub fn write(mut address: usize, buf: [u8; FIRMWARE_SIZE], len: usize) {
    unlock_fpec();
    for i in 0..len {
        let data = if i % 2 != 0 {
            (buf[i] as u16) << 8 | (buf[i - 1] as u16)
        } else if i == len - 1 {
            buf[i] as u16
        } else {
            continue;
        };
        write_byte(address, data);
        address += 0x2;
    }
    lock();
}

/// write to flash per byte
fn write_byte(address: usize, data: u16) {
    let ptr = unsafe {
        &*stm32::FLASH::ptr()
    };

    wait_free();
    ptr.cr.write(|w| w.pg().set_bit());
    unsafe { *(address as *mut u16) = data; }
}

/// wait flash free
fn wait_free() {
    let ptr = unsafe {
        &*stm32::FLASH::ptr()
    };
    while !ptr.sr.read().bsy().bit_is_clear() {}
}