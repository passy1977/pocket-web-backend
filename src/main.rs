extern crate core;
use std::ffi::c_uint;

unsafe extern "C" {
    fn hello(i: c_uint);
}


fn main() {
    println!("Hello, world!");
    unsafe { 
        hello(32); 
    }
}
