#![feature(panic_info_message)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]

/// this is a IAP bootloader only for my stm32
/// it should update firmware from tf card
/// you can edit these codes to make it work on your device

mod rcc;
mod sdcard;
mod usb_ttl;
mod flash;
mod upgrade;
mod tim;
mod interrupt;
mod button;
mod led;

use stm32f4xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use crate::usb_ttl::USART1;

static mut SECOND: u8 = 0;
static mut UPGRADE_FLAG: bool = false;

const OS_START_ADDRESS: usize = 0x08020000;
const MSP_ADDRESS: *mut usize = OS_START_ADDRESS as *mut usize;
const VECTOR_ADDRESS: *mut usize = (OS_START_ADDRESS + 0x4) as *mut usize;

#[no_mangle]
fn main() {
    let mut dp = stm32::Peripherals::take().unwrap();
    rcc::clock_init(&mut dp.RCC, &mut dp.FLASH);
    sdcard::init(&mut dp.RCC, &mut dp.GPIOC, &mut dp.GPIOD);
    usb_ttl::init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART1);
    led::init(&mut dp.RCC, &mut dp.GPIOF);
    button::init(&mut dp.RCC, &mut dp.GPIOE);
    tim::init(&mut dp.RCC, &mut dp.TIM2);
    interrupt::nvic_enable();
    led::green_light();

    writeln!(USART1, "This is a IAP bootloader").unwrap();
    writeln!(USART1, "start to check for upgrade from sd card").unwrap();
    writeln!(USART1, "").unwrap();

    upgrade::check_and_upgrade();

    interrupt::nvic_disable();
    button::disable_interrupt();
    tim::clean_interrupt_flag();
    tim::disable_count();
    led::green_dark();
    led::red_dark();

    boot_os(MSP_ADDRESS, VECTOR_ADDRESS);
}

/// boot the os
fn boot_os(msp: *mut usize, pc: *mut usize) {
    writeln!(USART1, "boot os").unwrap();
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
    writeln!(USART1, "{}, {}", info.message().unwrap(), info.location().unwrap()).unwrap();
    loop {}
}