use std::{ffi::CString, str::FromStr};

use actix_web::{web::Json, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::services::http_response_helper::HttpResponseHelper;
use crate::{bindings::{pocket_initialize, pocket_login}, constants::Stats, models::rests::{Claims, DataTransport}, rest::rest_controller::RestController, services::session::Sessions};

impl RestController {
    
pub fn login(&self, data_transport: Json<DataTransport>) -> HttpResponse {

        let (email, passwd) = match &data_transport.data {
            None => return HttpResponseHelper::forbidden()
                .error("No data send")
                .build(),
            Some(data) => {
                let split: Vec<&str> = data.split("|").collect();

                if split.len() != 2 {
                    return HttpResponseHelper::forbidden()
                        .error("email and passwd are mandatory")
                        .build()
                }

                (split[0].to_string(), split[1].to_string())
            }
        };

        let session = match Sessions::share().get(&*data_transport.session_id) {
            None => return HttpResponseHelper::forbidden()
                .error("Session not found")
                .build(),
            Some(session) => session
        };

        let data_dir_path = match self.data.dir_path.clone().as_path().to_str() {
            None => return HttpResponseHelper::not_acceptable()
                .session_id(session.session_id)
                .error("data_dit_path not found")
                .build(),
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
                        return HttpResponseHelper::not_acceptable()
                            .session_id(session.session_id)
                            .error("Server data wrong format")
                            .build()
                }

                let rc = pocket_login(session.pocket,
                                      CString::from_str(&email).unwrap().as_ptr(),
                                      CString::from_str(&passwd).unwrap().as_ptr());

                if rc != Stats::Ok {
                    return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("Wrong email or passwd")
                        .data(Stats::from(rc).to_string())
                        .build()
                }
            }
        } else {
            return HttpResponseHelper::ok()
                .path("/registration")
                .title("Register new user")
                .data(email)
                .build();
        }

        let claims = Claims {
            sub: "".to_string(),
            exp: 0,
            iss: self.data.jwt_iss.clone(),
            aud: self.data.jwt_aud.clone(),
        };

        let jwt = match encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.data.jwt_secret.as_bytes())) {
            Ok(token) => Some(token),
            Err(err) => return HttpResponseHelper::internal_server_error()
                .error(err.to_string())
                .build()
        };


     HttpResponseHelper::ok()
        .path("/home")
         .title("Home")
         .jwt(jwt.unwrap())
         .session_id(session.session_id)
        .build()
    }

}