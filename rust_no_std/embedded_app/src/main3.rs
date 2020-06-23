#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hio};

use rt::entry;

entry!(main);

fn main() -> !
{
	let mut hstdout = hio::hstdout().unwrap();

	#[export_name = "Dummy Hello"]
	#[link_section = ".log"]
	static A: u8 = 0;

	let addr = &A as *const u8 as usize as u8;
	hstdout.write_all(&[addr]).unwrap();

	#[export_name = "BYe Dick"]
	#[link_section = ".log"]
	static B: u8 = 0;

	let addr = &B as *const u8 as usize as u8;
	hstdout.write_all(&[addr]).unwrap();

	debug::exit(debug::EXIT_SUCCESS);

	loop {}
}
