use actix_web::{App, HttpServer};
use crate::constants::DATA;
use crate::controllers::rests_controller::login;
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
//https://medium.com/@AlexanderObregon/building-restful-apis-with-rust-and-warp-70a6159fd804

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = match Data::init() {
        Ok(data) => data,
        Err(err) => panic!("{}", err)
    };
    
    let port = data.port;
    let ip = data.ip.clone();
        

    unsafe { DATA = Some(data); }

    println!("Starting server at http://{ip}:{port}");
    
    HttpServer::new(|| {
        App::new().service(login) 
    }).bind((ip, port))?
        .run()
        .await

}