use std::{env, error::Error, fs::{self, File}, io::Write, path::PathBuf};
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
	// build dir for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
	
    // extend the lib search path
    println!("cargo:rustc-link-search={}", out_dir.display());
	
    // put linking script in the build dir
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;
	
    // link to 'librt.a' with 'asm.s' inside
    fs::copy("librt.a", out_dir.join("librt.a"))?;
    println!("cargo:rustc-link-lib=static=rt");

    // rebuild if 'librt.a' changed
    println!("cargo:rerun-if-changed=librt.a");

    /*// assemble 'asm.s' into o.file, then into lib file .a
    Build::new().file("asm.s").target("thumbv7m-none-eabi").compile("asm");
    //Build::new().file("asm.s").compile("asm");
    
    // rebuild if 'asm.s' changed
    println!("cargo:rerun-if-changed=asm.s");*/
    
    Ok(())
}
