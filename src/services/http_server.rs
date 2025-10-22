
use crate::{models::data_transport::DataTransport, rest::rest_controller::RestController};
use actix_multipart::Multipart;
use actix_web::{web, Responder, HttpRequest};


pub async fn hello(session_id: web::Path<String>) -> impl Responder {
    RestController::share().hello(session_id)
}

pub async fn login(req: HttpRequest, data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().login(req, data_transport)
}

pub async fn registration(req: HttpRequest, data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().registration(req, data_transport)
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

// Funzione debug disponibile solo in modalità debug/development
#[cfg(debug_assertions)]
pub async fn debug(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().debug(data_transport)
}

pub async fn data(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().data(data_transport)
}

pub async fn change_passwd(req: HttpRequest, data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().change_passwd(req, data_transport)
}

pub async fn import_data(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().import_data(data_transport)
}

pub async fn logout(data_transport: web::Json<DataTransport>) -> impl Responder {
    RestController::share().logout(data_transport)
}

pub async fn heartbeat(req: HttpRequest, session_id: web::Path<String>) -> impl Responder {
    RestController::share().heartbeat(req, session_id)
}

pub async fn upload(multipart: Multipart) -> impl Responder {
    RestController::share().upload(multipart).await
}

pub mod server {
    use crate::services::{http_server::{heartbeat, upload}, session::Sessions};
    use crate::utils::configure_cors;
    
    //Available only in dev mode
    #[cfg(debug_assertions)]
    use super::debug;
    
    use super::{field_detail, group_detail, hello, login, home, registration, data, change_passwd, import_data, logout};
    use actix_files as fs;
    use actix_web::{web, App, HttpServer};
    use std::io;
    use actix_web::middleware::Logger;
    use crate::constants::conf::PORT;
    use crate::services::data::Url;

    pub async fn start(url: Url, max_threads: usize) -> io::Result<()> {

        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        
        Sessions::share().start_validator();

        let origin = if let Some(port) = url.port {
            println!("Server will start at http://{}:{}", url.address, port);
            format!("{}://{}:{}", url.scheme, url.address, port)
        } else {
            println!("Server will start at http://{}", url.address);
            format!("{}://{}", url.scheme, url.address)
        };


        println!("Starting server at {origin}");

        HttpServer::new(move || {
            let app = App::new()
                .wrap(Logger::default())
                .wrap(configure_cors(origin.clone()))
                .route("/v5/pocket/hello/{session_id}", web::get().to(hello))
                .route("/v5/pocket/login", web::post().to(login))
                .route("/v5/pocket/registration", web::post().to(registration))
                .route("/v5/pocket/home", web::put().to(home))
                .route("/v5/pocket/data", web::post().to(data))
                .route("/v5/pocket/field_detail", web::put().to(field_detail))
                .route("/v5/pocket/group_detail", web::put().to(group_detail));

            // Endpoint debug available only in dev mode
            #[cfg(debug_assertions)]
            let app = app.route("/v5/pocket/debug", web::post().to(debug));

            app.route("/v5/pocket/change_passwd", web::put().to(change_passwd))
                .route("/v5/pocket/import_data", web::put().to(import_data))
                .route("/v5/pocket/logout", web::put().to(logout))
                .route("/v5/pocket/upload", web::post().to(upload))
                .route("/v5/pocket/heartbeat/{session_id}", web::get().to(heartbeat))
                .service(fs::Files::new("/", "./statics").index_file("index.html"))
            })
            .bind((url.address, url.port.unwrap_or(PORT)))?
            .workers(max_threads)
            .run()
            .await
            
    }
}



