
use crate::{models::rests::DataTransport, rest::rest_controller::RestController};
use actix_web::{web, Responder};


pub async fn hello(session_id: web::Path<String>) -> impl Responder {
    RestController::share().hello(session_id)
}

pub async fn login(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().login(data_transport)
}

pub async fn registration(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().registration(data_transport)
}

pub async fn home(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().home(data_transport)
}

pub async fn field_detail(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().field_detail(data_transport)
}

pub async fn group_detail(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().group_detail(data_transport)
}


pub async fn debug(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().debug(data_transport)
}

pub async fn data(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().data(data_transport)
}



pub mod server {
    use crate::services::http_server::{debug, field_detail, group_detail, hello, login, home, registration, data};
    use actix_cors::Cors;
    use actix_files as fs;
    use actix_web::{web, App, HttpServer};
    use std::io;
    use actix_web::middleware::Logger;

    pub async fn start(ip: String, port: u16) -> io::Result<()> {

        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        println!("Starting server at http://{ip}:{port}");
        
        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .wrap(Cors::permissive())
                .route("/v5/pocket/hello/{session_id}", web::get().to(hello))
                .route("/v5/pocket/login", web::post().to(login))
                .route("/v5/pocket/registration", web::post().to(registration))
                .route("/v5/pocket/home", web::post().to(home))
                .route("/v5/pocket/data", web::post().to(data))
                .route("/v5/pocket/field_detail", web::post().to(field_detail))
                .route("/v5/pocket/group_detail", web::post().to(group_detail))
                .route("/v5/pocket/debug", web::post().to(debug))
                .service(fs::Files::new("/", "./statics").index_file("index.html"))
            })
            .bind((ip, port))?
            .run()
            .await
        
        
    }
}



