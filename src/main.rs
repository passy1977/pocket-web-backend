use crate::bindings::{exit, pocket_field_free, pocket_field_new};

mod constants;

#[allow(
    dead_code,
)]
mod models;

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
        let pocket_field = pocket_field_new();

        if !pocket_field.is_null() {
            let mut pocket_field_wrap = match (*pocket_field).to_field() {
                None => exit(1),
                Some(wrap) => wrap
            };
            pocket_field_free(pocket_field);

            pocket_field_wrap.title = Some("test".to_string());
            pocket_field_wrap.value = Some("ciao ciao".to_string());

            let back = pocket_field_wrap.to_pocket_field_t();
            pocket_field_free(back);

        }
    }
}