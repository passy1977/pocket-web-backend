use crate::controllers::rests_controller::{hello_controller, login_controller};
use crate::models::rests::DataTransport;
use actix_web::{web, HttpResponse, Responder};
pub async fn hello() -> impl Responder {
    hello_controller()
}

pub async fn login(data_transport: web::Json<DataTransport>) -> impl Responder {
    login_controller(data_transport)
}

pub async fn registration(_info: web::Json<DataTransport>) -> impl Responder {
    HttpResponse::Forbidden().finish()
}

pub async fn main(_info: web::Json<DataTransport>) -> impl Responder {
    HttpResponse::Forbidden().finish()
}

pub async fn field_detail(_info: web::Json<DataTransport>) -> impl Responder {
    HttpResponse::Forbidden().finish()
}

pub async fn group_detail(_info: web::Json<DataTransport>) -> impl Responder {
    HttpResponse::Forbidden().finish()
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
                .route("/v5/pocket/hello", web::get().to(hello))
                .route("/v5/pocket/login", web::post().to(login))
                .route("/v5/pocket/registration", web::post().to(registration))
                .route("/v5/pocket/main", web::post().to(main))
                .route("/v5/pocket/field_detail", web::post().to(field_detail))
                .route("/v5/pocket/group_detail", web::post().to(group_detail))
                .service(fs::Files::new("/", "./statics").index_file("index.html"))
            })
            .bind((ip, port))?
            .run()
            .await
        
        
    }
}



