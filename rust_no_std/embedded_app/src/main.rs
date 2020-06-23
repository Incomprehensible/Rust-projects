#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hio::{self, HStdout}};

use rt::entry;
use log::{error, warn, Log};

struct Logger {
	hstdout: HStdout,
}

impl Log for Logger {
	type Error = ();

	fn log(&mut self, address: u8) -> Result<(), ()> {
		self.hstdout.write_all(&[address])
	}
}

entry!(main);

fn main() -> !
{
	let hstdout = hio::hstdout().unwrap();
	let mut logger = Logger { hstdout };
	
	let _ = warn!(logger, "Logging shit");

	let _ = error!(logger, "No more logging");

	debug::exit(debug::EXIT_SUCCESS);

	loop {}
}
