#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unaligned_references)]

use std::ffi::CStr;

include!("bindings.rs");

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!(concat!("/src", "/bindings.rs"));



fn main() {



    //let mut pos = Vector2{x:1.0, y:2.0};
    unsafe {
        let value = return_const_char_ptr();
        let string = CStr::from_ptr(value).to_str().unwrap();

        let mut x: i32 = 3;
        let mut y: i32 = 4;
        
        #[test]
            assert!(return_int() == 8);
            assert!(return_float() == 8.0);
            assert!(string == "const char * -> string");
            assert!(*return_int_by_pointer() == 9);
            assert!(*return_int_by_reference() == 9);
            assert!(add_int_by_value(3, 4) == 7);
            assert!(add_int_by_pointer(&mut x, &mut y) == 7);
            assert!(add_int_by_reference(&mut x, &mut y) == 7);

    }

    println!("Hello, world!");
    
}
