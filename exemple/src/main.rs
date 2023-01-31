#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unaligned_references)]



include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    
    //let x = unsafe { };

    let x = unsafe { 
        usb_ifc_info{
            dev_vendor: 3,
            dev_product: 3,
            dev_class: 0,
            dev_subclass: 0,
            dev_protocol: 0,
            ifc_class: 0,
            ifc_subclass: 0,
            ifc_protocol: 0,
            has_bulk_in: 0,
            has_bulk_out: 0,
            writable: 0,
        } 
    };

    println!("{}", x.dev_vendor);
}
