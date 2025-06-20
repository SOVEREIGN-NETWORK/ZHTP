#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Export a simple function to test WASM compilation
#[no_mangle]
pub extern "C" fn execute_contract() -> i32 {
    42
}

#[no_mangle]
pub extern "C" fn validate_contract() -> i32 {
    1
}
