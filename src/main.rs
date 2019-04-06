// rm std lib
#![no_std]
// normally env calls crt0 for C to set up stack & call rust main(), but we don't have access to c, so we overwrite crt0 entry point
#![no_main]

use core::panic::PanicInfo;


// panic 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point, linker looks for _start()
    loop{}
}


// Deprecated MacOS entry point before before we started targeting the custom OS
// macOS entry point
//#[no_mangle]
//pub extern "C" fn main() -> ! { // extern "C" use c calling conv.
//    loop {}
//}