use std::ffi::CString;
use std::str::FromStr;
use actix_web::{HttpResponse, HttpRequest};
use actix_web::web::Json;
use crate::bindings::{pocket_initialize, pocket_is_no_network, pocket_login};
use crate::constants::Stats;
use crate::models::data_transport::DataTransport;
use crate::perform_timestamp_last_update;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::rate_limiter::check_rate_limit_or_reject;
use crate::services::session::Sessions;


impl RestController {
    
pub fn login(&self, req: HttpRequest, data_transport: Json<DataTransport>) -> HttpResponse {

    // Check rate limiting for login endpoint
    if let Some(response) = check_rate_limit_or_reject(&req, "/v5/pocket/login", Some(data_transport.session_id.as_str())) {
        return response;
    }

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

    if Sessions::share().check_if_already_logged(&email) {
        return HttpResponseHelper::forbidden()
            .error("Account already logged in")
            .build()
    }

    let mut session = match Sessions::share().get(&*data_transport.session_id) {
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

            session.email = Some(email.clone());
            session.remote_session_handling = !pocket_is_no_network(session.pocket);

            Sessions::share().remove(&session.session_id, false);
            Sessions::share().add(session.clone());
                        

        }
    } else {
        return HttpResponseHelper::ok()
            .path("/registration")
            .title("Register new user")
            .session_id(session.session_id)
            .data(email)
            .build();
    }
    
    perform_timestamp_last_update!(session);
    HttpResponseHelper::ok()
        .path("/home")
        .title("Home")
        .session_id(session.session_id)
        .build()
    }

}