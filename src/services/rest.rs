use actix_web::{get, post, HttpResponse, Responder};
use crate::models::rests::DataTransport;

#[post("/v5/pocket/login")]
async fn login(request: DataTransport) -> impl Responder {
    
    
    
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}
