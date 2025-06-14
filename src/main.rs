use warp::Filter;
use std::net::{AddrParseError, Ipv4Addr};
use std::str::FromStr;
use warp::http::Method;
use crate::constants::DATA;
use crate::rests::server;
use crate::rests::server::start;
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
    let data = match Data::init() {
        Ok(data) => data,
        Err(err) => panic!("{}", err)
    };
    
    let ip = match Ipv4Addr::from_str(data.ip.as_ref()) {
        Ok(ip) => ip,
        Err(_) => panic!("Invalid IP Address provided!")
    };

    unsafe { DATA = Some(data); }
    
    start(ip, 8080).await;

}