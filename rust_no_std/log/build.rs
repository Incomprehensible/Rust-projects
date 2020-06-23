use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
	// build dir for the linker
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
	
    File::create(out_dir.join("log.x"))?.write_all(include_bytes!("log.x"))?;
	
	println!("cargo:rustc-link-search={}", out_dir.display());
    
    Ok(())
}
