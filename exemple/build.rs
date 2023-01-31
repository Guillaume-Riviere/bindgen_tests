use std::env;
use std::path::PathBuf;

fn main() {
    // cc::Build::new()
    //     .file("src/lib.c")
    //     .compile("lib");

    let bindings = bindgen::Builder::default()
        .header("cpp/lib1.h")
        .generate_inline_functions(true)
        .layout_tests(false)
        .clang_arg("-x").clang_arg("c++").clang_arg("-std=c++14")
        //.allowlist_function("lib1_.*")
        .allowlist_file("cpp/lib1.h")
        .generate()
        .unwrap();


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(std::path::PathBuf::from(out_path).join("bindings.rs"))
        .unwrap();


    // create a bindings.rs file at the root 
    // of the crate with the contents of the bindings

    let file = std::fs::File::create("src/bindings.rs").unwrap();
    bindings.write(Box::new(file)).unwrap();
}