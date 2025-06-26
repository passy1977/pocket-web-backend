use std::ffi::c_char;
use std::ptr::null;
use crate::constants::DATA;
use crate::models::rests::{Claims, DataTransport};
use crate::services::data::Data;
use crate::services::session::{Session, Sessions};
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;
use crate::bindings::{pocket_initialize};

pub struct RestController {
    data: Option<Data>
}

impl RestController {
    
    fn new() -> Self {
        Self { 
            data: unsafe {
                match (&raw const DATA).read() {
                    None => None,
                    Some(data) => Some(data)
                }
            }
        }
    }

    pub fn share() -> Arc<Self> {
        static INSTANCE: once_cell::sync::Lazy<Arc<RestController>> = once_cell::sync::Lazy::new(|| {
            Arc::new(RestController::new())
        });

        INSTANCE.clone()
    }


    pub fn hello(&self, session_id: web::Path<String>) -> HttpResponse {

        let mut session_id_handler = "".to_string();
        if !session_id.is_empty() {
            session_id_handler = session_id.clone();
        }

        let session_id = match Sessions::share().get(session_id_handler.as_str()) {
            None => {
                let session = Session::new();
                let session_id = session.session_id.clone();

                Sessions::share().add(session);
                session_id
            }
            Some(session) => session.session_id.clone()
        };
        
        HttpResponse::Ok().json(crate::models::rests::DataTransport{
            session_id,
            ..DataTransport::default()   
        })
    }

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

        let data = match &self.data {
            None => return HttpResponse::Forbidden().json(DataTransport{
                error: Some("Data not found".to_string()),
                ..DataTransport::default()
            }),
            Some(data) => data
        };
        
        if let Ok(config_json) = data.load_config_json(&email)
        {

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
            ..data_transport.into_inner()
        })

    }

    pub fn registration(&self, data_transport: Json<DataTransport>) -> HttpResponse {

        let (config_json, email , passwd, password_confirmation) = match &data_transport.data {
            None => return HttpResponse::Forbidden().json(DataTransport{
                error: Some("Data not found".to_string()),
                ..DataTransport::default()
            }),
            Some(data) => {
                let split: Vec<&str> = data.split("|").collect();

                if split.len() != 4 {
                    return HttpResponse::Forbidden().json(DataTransport{
                        error: Some("config_json, passwd and password_confirmation are mandatory".to_string()),
                        ..DataTransport::default()
                    });
                }

                (split[0].to_string(), split[1].to_string(), split[2].to_string(), split[3].to_string())
            }
        };
        
        if passwd != password_confirmation {
            return HttpResponse::NotAcceptable().json(DataTransport{
                error: Some("passwd and password_confirmation are different".to_string()),
                ..DataTransport::default()
            })
        }

        if config_json.is_empty() {
            return HttpResponse::NotAcceptable().json(DataTransport{
                error: Some("config_json is empty".to_string()),
                ..DataTransport::default()
            })
        }

        let (data, from_stored_data_config_json) = unsafe {
            match (&raw const DATA).read() {
                None => return HttpResponse::InternalServerError().body("DATA not ready"),
                Some(data) => {

                    let from_stored_data_config_json =  match data.load_config_json(&email) {
                        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
                        Ok(fron_stored_data_config_json) => fron_stored_data_config_json
                    };

                    (data, from_stored_data_config_json)
                }
            }
        };


        let session = match Sessions::share().get(data_transport.session_id.as_str()) {
            None => return HttpResponse::NotAcceptable().json(DataTransport{
                error: Some("session not found".to_string()),
                ..DataTransport::default()
            }),
            Some(session) => session
        };

        unsafe {

            if !pocket_initialize(session.pocket,
                                  data.dir_path.clone().as_path().to_str().unwrap().as_ptr() as *const i8,
                                  config_json.as_ptr() as *const i8,
                                  from_stored_data_config_json.as_ptr() as *const i8,
                                  passwd.as_ptr() as *const i8,
            ) {
                return HttpResponse::NotAcceptable().json(DataTransport{
                    error: Some("Server Data wrong format".to_string()),
                    ..DataTransport::default()
                })
            }
        }

        if data.store_config_json(&email, &config_json).is_err() {
            return HttpResponse::NotAcceptable().json(DataTransport{
                error: Some("Impossible store config_json".to_string()),
                ..DataTransport::default()
            })
        }

        HttpResponse::Ok().json(DataTransport {
            session_id: session.session_id,
            data: Some("{email}|{passwd}".to_string()),
            ..data_transport.into_inner()
        })
    }
    
    
}