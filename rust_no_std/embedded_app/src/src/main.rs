#![no_main]
#![no_std]

use cortex_m::interrupt;
use cortex_m_semihosting::{debug, hio::{self, HStdout}};

use rt::entry;
use log::{global_logger, GlobalLog, log};

struct Logger;

global_logger!(Logger); /* dynamically declaring type of global static logger */
						/* it's created inside macro block */

impl GlobalLog for Logger {
	fn log(&self, address: u8) {
		/* zone, free of interrupts */
		/* since we take mutable ref to static global var, it's required */
		interrupt::free(|_| unsafe {
			static mut HSTDOUT: Option<HStdout> = None;

			if HSTDOUT.is_none() {
				HSTDOUT = Some(hio::hstdout()?);
			}

			let hstdout = HSTDOUT.as_mut().unwrap();
			hstdout.write_all(&[address])
		}).ok(); /* ignore all errors inside a block */
	}
}

entry!(main);

fn main() -> !
{
	log!("Yo");
	log!("Ja mata");

	debug::exit(debug::EXIT_SUCCESS);

	loop {}
}
