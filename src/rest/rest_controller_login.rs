use std::{ffi::CString, str::FromStr};

use actix_web::{web::Json, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{bindings::{pocket_initialize, pocket_login}, constants::Stats, models::rests::{Claims, DataTransport}, rest::rest_controller::RestController, services::session::{Session, Sessions}};


impl RestController {
    
pub fn login(&self, data_transport: Json<DataTransport>) -> HttpResponse {

        let (email, passwd) = match &data_transport.data {
            None => return HttpResponse::Forbidden().json(DataTransport{
                error: Some("No data send".to_string()),
                ..DataTransport::default()
            }),
            Some(data) => {
                let split: Vec<&str> = data.split("|").collect();

                if split.len() != 2 {
                    return HttpResponse::Forbidden().json(DataTransport{
                        error: Some("email and passwd are mandatory".to_string()),
                        ..DataTransport::default()
                    });
                }

                (split[0].to_string(), split[1].to_string())
            }
        };

        let session = match Sessions::share().get(&*data_transport.session_id) {
            None => return HttpResponse::Forbidden().json(DataTransport{
                error: Some("Session not found".to_string()),
                ..DataTransport::default()
            }),
            Some(session) => session
        };

        let data_dir_path = match self.data.dir_path.clone().as_path().to_str() {
            None => return HttpResponse::NotAcceptable().json(DataTransport{
                session_id: session.session_id,
                error: Some("data_dit_path not found".to_string()),
                ..DataTransport::default()
            }),
            Some(data_dir_path) => data_dir_path.to_string()
        };
        
        if let Ok(config_json) = &self.data.load_config_json(&email)
        {
            unsafe {
                    if !pocket_initialize(session.pocket,
                                      CString::from_str(&data_dir_path).unwrap().as_ptr(),
                                      CString::from_str(&config_json).unwrap().as_ptr(),
                                      true,
                                      CString::from_str(&passwd).unwrap().as_ptr()
                ) {
                    return HttpResponse::NotAcceptable().json(DataTransport{
                        session_id: session.session_id,
                        error: Some("Server data wrong format".to_string()),
                        ..DataTransport::default()
                    })
                }

                let rc = pocket_login(session.pocket,
                                      CString::from_str(&email).unwrap().as_ptr(),
                                      CString::from_str(&passwd).unwrap().as_ptr());

                if rc != Stats::Ok {
                    return HttpResponse::NotAcceptable().json(DataTransport{
                        session_id: session.session_id,
                        error: Some("Wrong email or passwd".to_string()),
                        data: Some(Stats::from(rc).to_string()),
                        ..DataTransport::default()
                    })
                }
            }
        } else {
            return HttpResponse::Ok().json(DataTransport {
                path: "/registration".to_string(),
                title: "Register new user".to_string(),
                data: Some(email),
                ..data_transport.into_inner()
            })
        }

        let claims = Claims {
            sub: "".to_string(),
            exp: 0,
            iss: self.data.jwt_iss.clone(),
            aud: self.data.jwt_aud.clone(),
        };

        let jwt = match encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.data.jwt_secret.as_bytes())) {
            Ok(token) => Some(token),
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
        };

        HttpResponse::Ok().json(DataTransport {
            path: "/home".to_string(),
            title: "Home".to_string(),
            jwt,
            ..data_transport.into_inner()
        })

    }

}