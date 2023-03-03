use std::env;
use std::fs::File;
use std::path::Path;
use gl_generator::{Api, Fallbacks, Profile, Registry, StaticGenerator};

fn main() {
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=GLX");
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(StaticGenerator, &mut file)
        .unwrap();
}