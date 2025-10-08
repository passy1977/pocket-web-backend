use std::{ffi::CString, str::FromStr};

use actix_web::{web::Json, HttpResponse, HttpRequest};

use crate::{bindings::pocket_initialize, models::data_transport::DataTransport, rest::rest_controller::RestController, services::session::Sessions, utils::aes_encrypt};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::rate_limiter::check_rate_limit_or_reject;

impl RestController {

        pub fn registration(&self, req: HttpRequest, data_transport: Json<DataTransport>) -> HttpResponse {

        // Verifica rate limiting per l'endpoint di registration
        if let Some(response) = check_rate_limit_or_reject(&req, "/v5/pocket/registration", Some(data_transport.session_id.as_str())) {
            return response;
        }

        let (config_json, email , passwd, password_confirmation) = match &data_transport.data {
            None => return HttpResponseHelper::forbidden()
                .error("No data send")
                .build(),
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
            return HttpResponseHelper::not_acceptable()
                .error("passwd and password_confirmation are different")
                .build()
        }

        if config_json.is_empty() {
            return HttpResponseHelper::not_acceptable()
                .error("config_json is empty")
                .build()
        }

        let session = match Sessions::share().get(data_transport.session_id.as_str()) {
            None => return HttpResponseHelper::ok()
                .path("/hello")
                .title("Hello")
                .error("Session not found")
                .build(),
            Some(session) => session
        };

        if session.pocket.is_null() {
            return HttpResponseHelper::not_acceptable()
                .path("/hello")
                .title("Hello")
                .error("Pocket not initialized")
                .build()
        }

        let data_dir_path = match self.data.dir_path.clone().as_path().to_str() {
            None => return HttpResponseHelper::not_acceptable()
                .session_id(session.session_id)
                .error("data_dit_path not found")
                .build(),
            Some(data_dir_path) => data_dir_path.to_string()
        };
        
        unsafe {
            if !pocket_initialize(session.pocket,
                                  CString::from_str(&data_dir_path).unwrap().as_ptr(),
                                  CString::from_str(&config_json).unwrap().as_ptr(),
                                  false,
                                  CString::from_str(&passwd).unwrap().as_ptr(),
            ) {
                return HttpResponseHelper::not_acceptable()
                    .session_id(session.session_id)
                    .error("Server data wrong format")
                    .build()
            }
        }

        let config_json = aes_encrypt(session.pocket, &config_json);
        if config_json == "" {
            return HttpResponseHelper::not_acceptable()
                .session_id(session.session_id)
                .error("Impossible encrypt config_json")
                .build()
        }

        if self.data.store_config_json(&email, &config_json).is_err() {
            return HttpResponseHelper::not_acceptable()
                .session_id(session.session_id)
                .error("Impossible store config_json")
                .build()
        }
        
        let mut data = String::new();
        data.push_str(email.as_str());
        data.push_str("|");
        data.push_str(passwd.as_str());
            
        HttpResponseHelper::ok()
            .path("/login")
            .title("Login")
            .session_id(session.session_id)
            .data(data)
            .build()
    }
    

}