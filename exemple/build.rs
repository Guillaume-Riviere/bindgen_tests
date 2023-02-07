use std::env;
use std::path::PathBuf;
use std::fs;



fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let src = ["cpp/liba.cpp"];

    cc::Build::new()
        .cpp(true)
        .files(src.iter())
        .compile("mybar");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .layout_tests(false)
        .clang_arg("-xc++").clang_arg("-std=c++11").clang_arg("-Wc++11-extensions")
        //.clang_arg("-Ivendor/cpp")
        // .allowlist_function("barfunc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // write bindings in src/bindings.rs
    bindings.write_to_file("src/bindings.rs").expect("Couldn't write bindings!");
}