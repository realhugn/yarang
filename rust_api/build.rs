use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=./c_lib/build");
    println!("cargo:rustc-link-lib=static=c_lib");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=c_lib/include/add.h");

    let bindings = bindgen::Builder::default()
        .header("../build/src/add.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
