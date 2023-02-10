#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unaligned_references)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

include!("bindings.rs");

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!(concat!("/src", "/bindings.rs"));

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_what_we_want()-> String{
    let mut result = "".to_owned();
    if let Ok(lines) = read_lines("./cpp/fabgen.cpp") {
        // Consumes the iterator, returns an (Optional) String
        let mut flag = false;
        for line in lines {
            
            if let Ok(ip) = line {
                if ip == "// basic interoperability"{
                    flag = true;
                }
                if ip == "// type_tag based cast system"{
                    flag = false;
                }
                if flag{
                    if ip != "static int static_int = 9;"{
                        result.push_str(&ip);
                        result.push_str("\n");
                    }
                }
            }
        }
    }
    result
}

fn get_rid_of_mustach(mut text: String) -> String{
    let mut flag = false;
    //println!("{}", text);
    let mut i = 0;
    while i < text.len() {
        if text.chars().nth(i).unwrap() == '{'{
            flag = true;
        }
        if text.chars().nth(i).unwrap() == '}'{
            flag = false;
            text.replace_range(i..i+1, ";");
        }
        if flag{
            text.replace_range(i..i+1, "");
            i-=1;
        }
        i += 1;
    }
    text
}

fn main() {

    let text = get_rid_of_mustach(extract_what_we_want()); 
    

    //println!("{}", text);


    let mut file = OpenOptions::new().append(true).open("./cpp/fabgen.h").expect("cannot open file");
    file.write_all(text.as_bytes()).expect("write failed");
    println!("file append success");

    /*for _char in text.chars() {
        if _char == '{'{
            flag = true;
            text.rep;
        }
        if flag{

        }
    }*/


    //let mut pos = Vector2{x:1.0, y:2.0};
    /*unsafe {
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
    */

    
    
}
