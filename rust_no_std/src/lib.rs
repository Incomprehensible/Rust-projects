#![no_std]

use core::panic::PanicInfo;
use core::ptr;

/* in order to link reset_handler and it's pointer reset vector we need their symbols to have external linkage
we can do it with "pub" (aka global) word. also no private modules in-between krate and symbols
reset handler is a func that is executed first after sys reset or first power up
its the first hardware stack frame so it can not return to anywhere. we make it so using divergent func syntax -> !
to refer to symbols from linking script we must turn off mangling to keep names stable */

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
	/* initialize RAM so bss and data don't contain trash */
	extern "C" {
		static mut _sbss: u8;
		static mut _ebss: u8;

		static mut _sdata: u8;
		static mut _edata: u8;
		static _sidata: u8;
	}

	let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
	ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

	let count = &_sdata as *const u8 as usize - &_edata as *const u8 as usize;
	ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

	/* Call user's entry point */
	extern "Rust" {
		fn main() -> !;
	}

	main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[macro_export]
macro_rules! entry {
	($path:path) => {
		#[export_name = "main"]
		pub unsafe fn __main() -> ! {
			let f: fn() -> ! = $path;

			f()
		}
	}
}

/* we could align sections' vars' RAM addresses at will and then zero out sections not bytewise but wordwise */
/* in linker script it would be . = ALIGN(4) at the start and the end of sections */

/* user uses entry!(path) macro to specify the function he wants to envoke as his own main
we take his path (his fn) and check that it's divergent function as it should be, wrapping it into
fixed format function pointer. then we call his function via macro. we need #export_name so
our Reset actually refers our macro exported as main, and macro calls user inner main. that's why
user main is not marked as pub. if we did not export main macro to user and relied on user to make 
his own main proper divergent, user would mark it as pub main and mark it no mangle */

/* #![no_main] has no effect on library crates */