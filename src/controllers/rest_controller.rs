use crate::bindings::{free, pocket_initialize, pocket_initialize_aes, pocket_login, pocket_stat_t, pocket_stat_t_OK};
use crate::models::rests::{Claims, DataTransport};
use crate::services::data::Data;
use crate::services::session::{Session, Sessions};
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::ffi::CString;
use std::str::FromStr;
use std::sync::Arc;
use crate::constants::data::EMPTY_CONFIG_JSON;
use crate::constants::Stats;
use crate::utils::aes_encrypt;

pub struct RestController {
    data: Data
}



impl RestController {
    
    fn new() -> Self {
        Self { 
            data: Data::init().unwrap()
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

        let data_dir_path = match self.data.dir_path.clone().as_path().to_str() {
            None => return HttpResponse::NotAcceptable().json(DataTransport{
                session_id: session.session_id,
                error: Some("data_dit_path not found".to_string()),
                ..DataTransport::default()
            }),
            Some(data_dir_path) => data_dir_path.to_string()
        };
        
        if let Ok(from_stored_data_config_json) = &self.data.load_config_json(&email)
        {
            unsafe {
                
                // if !pocket_initialize_aes(session.pocket, CString::new(passwd.clone()).unwrap().as_ptr()) {
                //     return HttpResponse::NotAcceptable().json(DataTransport{
                //         session_id: session.session_id,
                //         error: Some("Impossible init aes".to_string()),
                //         ..DataTransport::default()
                //     })
                // }    

                let mut store = Box::new(false);
                if !pocket_initialize(session.pocket,
                                      CString::from_str(&data_dir_path).unwrap().as_ptr(),
                                      CString::from_str(EMPTY_CONFIG_JSON).unwrap().as_ptr(),
                                      CString::from_str(&from_stored_data_config_json).unwrap().as_ptr(),
                                      CString::from_str(&passwd).unwrap().as_ptr(),
                                      store.as_mut()
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

        let session = match Sessions::share().get(data_transport.session_id.as_str()) {
            None => return HttpResponse::NotAcceptable().json(DataTransport{
                path: "/hello".to_string(),
                title: "Hello".to_string(),
                error: Some("session not found".to_string()),
                ..DataTransport::default()
            }),
            Some(session) => session
        };

        if session.pocket.is_null() {
            return HttpResponse::NotAcceptable().json(DataTransport{
                path: "/hello".to_string(),
                title: "Hello".to_string(),
                error: Some("data_dit_path not found".to_string()),
                ..DataTransport::default()
            })
        }

        let data_dir_path = match self.data.dir_path.clone().as_path().to_str() {
            None => return HttpResponse::NotAcceptable().json(DataTransport{
                session_id: session.session_id,
                error: Some("data_dit_path not found".to_string()),
                ..DataTransport::default()
            }),
            Some(data_dir_path) => data_dir_path.to_string()
        };

        unsafe {
            if !pocket_initialize_aes(session.pocket, CString::new(passwd.clone()).unwrap().as_ptr()) {
                return HttpResponse::NotAcceptable().json(DataTransport{
                    session_id: session.session_id,
                    error: Some("Impossible init aes".to_string()),
                    ..DataTransport::default()
                })
            }    
        }
        
        let config_json = aes_encrypt(session.pocket, &config_json);
        if config_json == "" {
            return HttpResponse::NotAcceptable().json(DataTransport{
                session_id: session.session_id,
                error: Some("Impossible encrypt config_json".to_string()),
                ..DataTransport::default()
            })
        }


        let mut store = Box::new(false);
        
        unsafe {
            if !pocket_initialize(session.pocket,
                                  CString::from_str(&data_dir_path).unwrap().as_ptr(),
                                  CString::from_str(&config_json).unwrap().as_ptr(),
                                  CString::from_str(EMPTY_CONFIG_JSON).unwrap().as_ptr(),
                                  CString::from_str(&passwd).unwrap().as_ptr(),
                                  store.as_mut()
            ) {
                return HttpResponse::NotAcceptable().json(DataTransport{
                    session_id: session.session_id,
                    error: Some("Server data wrong format".to_string()),
                    ..DataTransport::default()
                })
            }
        }

        if *store {
            if self.data.store_config_json(&email, &config_json).is_err() {
                return HttpResponse::NotAcceptable().json(DataTransport{
                    session_id: session.session_id,
                    error: Some("Impossible store config_json".to_string()),
                    ..DataTransport::default()
                })
            }
        }

        let mut data = String::new();
        data.push_str(email.as_str());
        data.push_str("|");
        data.push_str(passwd.as_str());
        
        HttpResponse::Ok().json(DataTransport {
            path: "/login".to_string(),
            title: "Login".to_string(),
            session_id: session.session_id,
            data: Some(data),
            ..data_transport.into_inner()
        })
    }
    
    
}