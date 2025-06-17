use crate::constants::DATA;
use crate::models::rests::{Claims, DataTransport};
use actix_web::web::Json;
use actix_web::HttpResponse;
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn login_controller(_: Json<DataTransport>) -> HttpResponse {
    
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
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    };

    HttpResponse::Ok().json(DataTransport {
        path: "/registration".to_string(),
        title: "Registration".to_string(),
        jwt,
        ..DataTransport::default()
    })
    
}