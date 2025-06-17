use crate::controllers::rests_controller::login_controller;
use crate::models::rests::DataTransport;
use actix_web::{web,  Responder};
pub async fn hello(info: web::Json<DataTransport>) -> impl Responder {
    login_controller(info)
}

pub async fn login(info: web::Json<DataTransport>) -> impl Responder {
    login_controller(info)
}

pub async fn registration(info: web::Json<DataTransport>) -> impl Responder {
    login_controller(info)
}

pub async fn main(info: web::Json<DataTransport>) -> impl Responder {
    login_controller(info)
}

pub async fn field_detail(info: web::Json<DataTransport>) -> impl Responder {
    login_controller(info)
}

pub async fn group_detail(info: web::Json<DataTransport>) -> impl Responder {
    login_controller(info)
}


pub mod server {
    use crate::services::http_server::{field_detail, group_detail, hello, login, main, registration};
    use actix_cors::Cors;
    use actix_files as fs;
    use actix_web::{web, App, HttpServer};
    use std::io;

    pub async fn start(ip: String, port: u16) -> io::Result<()> {

        println!("Starting server at http://{ip}:{port}");
        
        HttpServer::new(|| {
            App::new()
                .wrap(Cors::permissive())
                .service(fs::Files::new("/", "./statics").index_file("index.html"))
                    .route("/hello", web::post().to(hello))
                    .route("/login", web::post().to(login))
                    .route("/registration", web::post().to(registration))
                    .route("/main", web::post().to(main))
                    .route("/field_detail", web::post().to(field_detail))
                    .route("/group_detail", web::post().to(group_detail))
            
            })
            .bind((ip, port))?
            .run()
            .await
        
        
    }
}



