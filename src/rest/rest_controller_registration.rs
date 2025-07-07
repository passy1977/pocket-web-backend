use std::{ffi::CString, str::FromStr};

use actix_web::{web::Json, HttpResponse};

use crate::{bindings::pocket_initialize, models::rests::DataTransport, rest::rest_controller::RestController, services::session::Sessions, utils::aes_encrypt};

impl RestController {

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
            if !pocket_initialize(session.pocket,
                                  CString::from_str(&data_dir_path).unwrap().as_ptr(),
                                  CString::from_str(&config_json).unwrap().as_ptr(),
                                  false,
                                  CString::from_str(&passwd).unwrap().as_ptr(),
            ) {
                return HttpResponse::NotAcceptable().json(DataTransport{
                    session_id: session.session_id,
                    error: Some("Server data wrong format".to_string()),
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

        if self.data.store_config_json(&email, &config_json).is_err() {
            return HttpResponse::NotAcceptable().json(DataTransport{
                session_id: session.session_id,
                error: Some("Impossible store config_json".to_string()),
                ..DataTransport::default()
            })
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