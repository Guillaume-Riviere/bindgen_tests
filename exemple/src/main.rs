#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unaligned_references)]

//include!("bindings.rs");

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {

    //let mut pos = Vector2{x:1.0, y:2.0};

    print!("Hello, world!");
    
}
