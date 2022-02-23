#![no_std]

#[no_mangle]
pub extern "C" fn test(a: u32, b: u32) -> u32 {
    nonbinary::add(a, b)
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}