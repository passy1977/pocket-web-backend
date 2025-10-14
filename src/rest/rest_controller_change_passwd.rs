use std::ffi::CString;
use std::path::MAIN_SEPARATOR;
use crate::bindings::{pocket_change_passwd, pocket_stat_t_OK, pocket_user_t};
use crate::constants::data::EXPORT_DATA_CHANGE_PASSWD;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::{delete_file, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_session, perform_timestamp_last_update};
use crate::utils::aes_decrypt;
use actix_web::{web::Json, HttpResponse, HttpRequest};
use crate::services::rate_limiter::check_rate_limit_or_reject;

impl RestController {

    pub fn change_passwd(&self, req: HttpRequest, data_transport: Json<DataTransport>) -> HttpResponse {
        
        // Check rate limiting for change_passwd endpoint
        if let Some(response) = check_rate_limit_or_reject(&req, "/v5/pocket/change_passwd", Some(data_transport.session_id.as_str())) {
            return response;
        }
        
        let mut session = get_session!(data_transport.session_id, "Session not found");

        if let Some(data) = &data_transport.data {
            if data.is_empty() {
                perform_timestamp_last_update!(session);
                HttpResponseHelper::ok()
                    .path("/change-passwd")
                    .title("Change password")
                    .session_id(session.session_id)
                    .build()

            } else {
                let pwd_split: Vec<&str> = data.split("|").collect();

                if pwd_split.len() < 2 {
                    return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("Passwd and newPasswd are mandatory")
                        .build()
                }
                
                let user = unsafe {(*((*session.pocket).user as *const pocket_user_t)).to_user() };

                if user.passwd.trim() != pwd_split[0].trim() {
                    return HttpResponseHelper::forbidden()
                        .session_id(session.session_id)
                        .error("Passwd and old passwd don't match")
                        .build()
                }


                let mut full_path_file = match self.data.dir_path.clone().as_path().to_str() {
                    None => return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("data_dir_path not found")
                        .build(),
                    Some(data_dir_path) => data_dir_path.to_string()
                };
                full_path_file.push(MAIN_SEPARATOR);
                full_path_file.push_str(EXPORT_DATA_CHANGE_PASSWD);


                if let Ok(config_json) = &self.data.load_config_json(&user.email) {
                    let status = unsafe { 

                        let config_json = aes_decrypt(session.pocket, config_json);

                        pocket_change_passwd(session.pocket
                        , CString::new(full_path_file.clone()).unwrap().as_ptr()
                        , CString::new(config_json.as_str()).unwrap().as_ptr()
                        , CString::new(pwd_split[1].trim()).expect("").as_ptr()
                    ) };

                    if status == pocket_stat_t_OK {

                        if delete_file(&full_path_file).is_err() {
                            eprintln!("Impossible delete export file")
                        }

                        Sessions::share().remove(&session.session_id, true);

                        return HttpResponseHelper::ok()
                        .path("/login")
                        .data("logout")
                        .session_id(session.session_id).build();

                    } else {

                        if delete_file(&full_path_file).is_err() {
                            eprintln!("Impossible delete export file")
                        }

                        return HttpResponseHelper::internal_server_error()
                        .session_id(session.session_id)
                        .error("Something's wrong in data parsing, changing passwd failed")
                        .build()
                    }

                } else {
                    if delete_file(&full_path_file).is_err() {
                        eprintln!("Impossible delete export file")
                    }

                    return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("config_json export be load")
                        .build()
                };
            }
        } else {
            HttpResponseHelper::internal_server_error()
            .error("Data it's mandatory")
            .build()          
        }


    }
}