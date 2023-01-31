use std::env;
use std::path::PathBuf;

fn main() {





    // cc::Build::new()
    //     .file("src/lib.c")
    //     .compile("lib");
    let bindings = bindgen::Builder::default()
        .header("cpp/lib.h")
        .clang_arg("-x").clang_arg("c++").clang_arg("-std=c++14")
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(std::path::PathBuf::from(out_path).join("bindings.rs"))
        .unwrap();
}