use crate::constants::DATA;
use crate::models::rests::{Claims, DataTransport};
use actix_web::web::Json;
use actix_web::HttpResponse;
use jsonwebtoken::{encode, EncodingKey, Header};
use ulid::Ulid;
use crate::bindings::pocket_field_new;
use crate::services::session::{Sessions, Session};

pub fn hello_controller() -> HttpResponse {
    
    Sessions::share().add_session(Session{
        user_id: "".to_string(),
        jwt: "".to_string(),
        timestamp_last_update: 0,
    });
    
    HttpResponse::Ok().json(DataTransport {
        path: "/login".to_string(),
        title: "Login".to_string(),
        ..DataTransport::default()
    })
    
}

pub fn login_controller(_: Json<DataTransport>) -> HttpResponse {
    
    let data = unsafe {
        match (&raw const DATA).read() {
            None => return HttpResponse::InternalServerError().body("DATA not ready"),
            Some(data) => data
        }
    };
    
    let claims = Claims {
        exp: 0,
        iss: data.jwt_iss.clone(),
        aud: data.jwt_aud.clone(),
    };

    let jwt = match encode(&Header::default(), &claims, &EncodingKey::from_secret(data.jwt_secret.as_bytes())) {
        Ok(token) => Some(token),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    };

    HttpResponse::Ok().json(DataTransport {
        path: "/home".to_string(),
        title: "Home".to_string(),
        jwt,
        ..DataTransport::default()
    })
    
}