#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        unsafe {
            // pin 21 turned into output
            let addresss: u32 = 0x3F20_0008;
            core::ptr::write_volatile(addresss as *mut u32, 1 << 3);

            loop {
                // set pin 21 to high
                let high_address: u32 = 0x3F20_001C;
                core::ptr::write_volatile(high_address as *mut u32, 1 << 21);

                for _ in 0..50000 {
                    asm!("nop");
                }

                // set pin 21 to low
                let low_address: u32 = 0x3F20_0028;
                core::ptr::write_volatile(low_address as *mut u32, 1 << 21);

                for _ in 0..50000 {
                    asm!("nop");
                }
            }
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
