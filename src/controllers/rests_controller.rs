use actix_web::HttpResponse;
use actix_web::web::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::constants::{DATA, jwt::SECRET};
use crate::models::rests::{Claims, DataTransport};
use crate::services::data::Data;
use crate::utils::Result;

pub fn login(request: Json<DataTransport>) -> HttpResponse {
    
    let data = unsafe {
        match (&raw const DATA).read() {
            None => return HttpResponse::InternalServerError().body("DATA not ready"),
            Some(data) => data
        }
    };
    
    let claims = Claims {
        sub: "".to_string(),
        company: "".to_string(),
        exp: 0,
        iss: data.jwt_iss.clone(),
        aud: data.jwt_aud.clone(),
    };

    let jwt = match encode(&Header::default(), &claims, &EncodingKey::from_secret(data.jwt_secret.as_bytes())) {
        Ok(token) => Some(token),
        Err(err) => return HttpResponse::InternalServerError().body("Impossible generate d token")
    };

    HttpResponse::Ok().json(DataTransport {
        path: "/registration".to_string(),
        title: "Registration".to_string(),
        jwt,
        ..DataTransport::default()
    })
    
}