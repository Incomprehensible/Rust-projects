#![no_std]
#![no_main]

use rt::entry;

entry!(main);

fn main() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn HardFault(_ef: *const u32) -> ! {
    // user overrides this function, so it's invoken due to trap in main
    // instead of DefaultHandler
    loop {}
}
