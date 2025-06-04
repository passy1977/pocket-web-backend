use crate::bindings::{pocket_field_new, pocket_field_t};

mod constants;

#[allow(
    dead_code, 
    non_upper_case_globals, 
    non_camel_case_types, 
    non_snake_case,
    unused_imports,
    improper_ctypes
)]
mod bindings;

fn main() {

    unsafe {
        let pippo =  pocket_field_new();
        //println!("Hello, world! {:?}", );
    }

}
