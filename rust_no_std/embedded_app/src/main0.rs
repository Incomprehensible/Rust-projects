#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;

use rt::*;

entry!(main);
/*static RODATA: &[u8] = b"Rust is for the coolest programmers!!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1; */

fn main() -> ! {
	/*let _x = 42;
	let _y = RODATA;
	let z = unsafe { &BSS };
	let _w = unsafe { &DATA };
*/
    unsafe { intrinsics::abort() }
  //  loop {}
}

#[no_mangle]
pub extern "C" fn HardFault() -> ! {
    // user overrides this function, so it's invoken due to trap in main
    // instead of DefaultHandler
    loop {}
}
