#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unaligned_references)]

include!("bindings.rs");

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!(concat!("/src", "/bindings.rs"));

fn main() {

    //let mut pos = Vector2{x:1.0, y:2.0};
    unsafe {
        let mut constant = 9;
        let mut reference = 9;
        let mut low = 3;
        let mut high = 4;
        #[test]
            assert!(return_int() == 8);
            assert!(return_float() == 8.0);
            assert!(return_const_char_ptr() == "const char * -> string");
            assert!(return_int_by_pointer() == *constant);
            assert!(return_int_by_reference() == *reference);
            assert!(add_int_by_value(3, 4) == 7);
            assert!(add_int_by_pointer(*low, *high) == 7);
            assert!(add_int_by_reference(*low, *high) == 7);
            println!("test_basic_type_exchange passed");
        
    }
    

    print!("Hello, world!");
    
}
