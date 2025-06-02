use crate::bindings::{field_controller_t, pocket_field_controller_init, pocket_field_t};

mod constants;
mod bindings;


fn main() {
    println!("Hello, world!");
    unsafe {
        let pippo =  pocket_field_controller_init();
    }
}
