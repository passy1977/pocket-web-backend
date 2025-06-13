use crate::services::data::Data;

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
mod services;
mod utils;
mod traits;
mod controllers;
mod rests;
//https://medium.com/@AlexanderObregon/building-restful-apis-with-rust-and-warp-70a6159fd804

#[tokio::main]
async fn main() {
    match Data::init() {
        Ok(data) => {},
        Err(err) => panic!("{}", err)
    };
}