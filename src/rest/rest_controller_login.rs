use std::ffi::CString;
use std::str::FromStr;
use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::bindings::{pocket_initialize, pocket_login};
use crate::constants::Stats;
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;

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
                if session.pocket.is_null() || !pocket_initialize(session.pocket,
                                    CString::from_str(&data_dir_path).unwrap().as_ptr(),
                                    CString::from_str(&config_json).unwrap().as_ptr(),
                                    true,
                                    CString::from_str(&passwd).unwrap().as_ptr()
            ) || !(*session.pocket).is_valid() {
                    return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("Wrong password or server not available")
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
            .session_id(session.session_id)
            .data(email)
            .build();
    }
    
    HttpResponseHelper::ok()
        .path("/home")
        .title("Home")
        .session_id(session.session_id)
        .build()
    }

}