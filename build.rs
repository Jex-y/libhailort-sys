use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new("hailort").build();

    let obj_path = dst.join("build/hailort/libhailort/src/libhailort.so");
    let lib_path = out_dir.clone().join("libhailort.a");

    let headers_path = dst.join("include/hailo/hailort.h");
    let headers_path_str = headers_path
        .to_str()
        .expect("Header path is not a valid string");

    // Link to the hailort library
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=hailort");

    // Generate .a file
    let ar_output = std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn ar");

    if !ar_output.status.success() {
        panic!("Failed to generate libhailort.a: {:?}", ar_output);
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
