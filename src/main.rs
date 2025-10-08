use crate::services::data::Data;
use crate::services::http_server::server;
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
mod rest;

//https://medium.com/@AlexanderObregon/building-restful-apis-with-rust-and-warp-70a6159fd804


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = match Data::init() {
        Ok(data) => data,
        Err(err) => panic!("{}", err)
    };
    


    server::start(data.address.clone(), data.port, data.max_threads).await
}