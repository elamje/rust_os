// rm std lib
#![no_std]
// normally env calls crt0 for C to set up stack & call rust main(), but we don't have access to c, so we overwrite crt0 entry point
#![no_main]

use core::panic::PanicInfo;

// macOS entry point
#[no_mangle]
pub extern "C" fn main() -> ! { // extern "C" use c calling conv.
    loop {}
}


// panic 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


