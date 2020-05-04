#![feature(panic_info_message)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]

/// this is a IAP bootloader only for my stm32
/// it should update firmware from tf card
/// you can edit these codes to make it work on your device

mod rcc;
mod tf;
mod usart;
mod upgrade;
mod flash;

use stm32f1xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use crate::usart::USART2;

const OS_START_ADDRESS: usize = 0x0800E000;
const OS_END_ADDRESS: usize = 0x0800FFFF;
const FLASH_START_ADDRESS: usize = 0x08000000;
const FLASH_PAGE_SIZE: usize = 0x400;
const FIRMWARE_SIZE: usize = 5500;

const MSP_ADDRESS: *mut usize = OS_START_ADDRESS as *mut usize;
const VECTOR_ADDRESS: *mut usize = (OS_START_ADDRESS + 0x4) as *mut usize;

#[no_mangle]
#[inline(never)]
fn main() {
    let mut dp = stm32::Peripherals::take().unwrap();
    rcc::clock_init(&mut dp.RCC, &mut dp.FLASH);
    tf::spi1_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.SPI1);
    usart::usart2_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART2);

    writeln!(USART2, "This is a IAP bootloader").unwrap();
    writeln!(USART2, "start to check for update in tf card").unwrap();

    upgrade::check_and_write();

    writeln!(USART2, "boot os").unwrap();
    rcc::clock_disable(&mut dp.RCC);
    boot_os(MSP_ADDRESS, VECTOR_ADDRESS);
}

/// boot the os
fn boot_os(msp: *mut usize, pc: *mut usize) {
    unsafe {
        llvm_asm!(
        "msr msp, $0
         mov pc, $1"
         ::"{r0}"(*msp), "{r1}"(*pc)
         ::)
    }
}

#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    writeln!(USART2, "{}", info.message().unwrap()).unwrap();
    loop {}
}