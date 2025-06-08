use crate::services::data::Data;
use crate::services::cli::Cli;

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
mod socket;
mod services;
mod utils;

fn main() {
    let data = Data::init();
}