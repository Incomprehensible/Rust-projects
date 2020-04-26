#![no_std]
#![no_main]

use rt::*;

entry!(main);
static RODATA: &[u8] = b"Rust is for the coolest programmers!!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

//#[no_mangle]
fn main() -> ! {
	let _x = 42;
	let _y = RODATA;
	let z = unsafe { &BSS };
	let _w = unsafe { &DATA };

    loop {}
}
