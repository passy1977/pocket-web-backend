use std::ffi::CStr;
use crate::bindings::{pocket_field_new};

mod constants;

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
        let pippo = pocket_field_new();

        // Dereferenzia il puntatore per accedere ai campi della struttura
        if !pippo.is_null() {
            let field = *pippo; // dereference using `&*`

            // Sicurezza del puntatore per title e value
            let title: Option<String> = if !field.title.is_null() {
                CStr::from_ptr(field.title).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

            let value: Option<String> = if !field.value.is_null() {
                CStr::from_ptr(field.value).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

            // Stampa la proprietà deleted
            println!("Hello, world! {:?}", field.deleted);

            // Stampa i valori title e value se non sono nullo
            if let Some(title_str) = title {
                println!("Title: {}", title_str);
            } else {
                println!("Title is null");
            }

            if let Some(value_str) = value {
                println!("Value: {}", value_str);
            } else {
                println!("Value is null");
            }
        } else {
            eprintln!("pocket_field_new() returned a null pointer!");
        }
    }
}