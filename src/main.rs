use crate::services::data::Data;
use crate::socket::ws_server::start;

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

//openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout ssl_privkey.pem -out ssl_cert.pem

#[tokio::main]
async fn main() {
    match Data::init() {
        Ok(data) => start(&data).await,
        Err(err) => panic!("{}", err)
    };
}