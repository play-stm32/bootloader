#![feature(panic_info_message)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use stm32f1xx_hal::stm32;
use core::panic::PanicInfo;

const OS_ADDRESS: usize = 0x08002000;
const MSP_ADDRESS: *mut usize = OS_ADDRESS as *mut usize;
const VECTOR_ADDRESS: *mut usize = (OS_ADDRESS + 0x4) as *mut usize;

#[no_mangle]
#[inline(never)]
fn main() {
    boot_os(MSP_ADDRESS, VECTOR_ADDRESS);
}

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
pub unsafe extern "C" fn panic_fmt(_info: &PanicInfo) -> ! {
    loop {}
}