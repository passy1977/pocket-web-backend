use std::ffi::CString;
use std::{fs, io::{Error, ErrorKind}};
use std::path::{Path, MAIN_SEPARATOR};
use crate::bindings::{pocket_change_passwd, pocket_logout, pocket_stat_t_OK, pocket_user_t};
use crate::constants::data::EXPORT_DATA_CHANGE_PASSWD;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::get_session;
use crate::utils::aes_decrypt;
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    fn delete_file(&self, file: &String) -> Result<(), Error> {
        let path = Path::new(file);

        if path.exists() && path.is_file() {
            fs::remove_file(path)?;
        } else {
            return Err(Error::new(ErrorKind::NotFound, "Impossible remove configuration file"));
        }

        Ok(())
    }

    pub fn change_passwd(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        if let Some(data) = &data_transport.data {
            if data.is_empty() {
                session.update_timestamp_last_update();
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
                        unsafe {
                            if pocket_logout(session.pocket) == pocket_stat_t_OK {

                                if self.delete_file(&full_path_file).is_err() {
                                    eprintln!("Impossible delete config file")
                                }

                                Sessions::share().remove(&session.session_id, true);

                                return HttpResponseHelper::ok()
                                .path("/login")
                                .data("logout")
                                .session_id(session.session_id).build();
                            } else {
                        
                                if self.delete_file(&full_path_file).is_err() {
                                    eprintln!("Impossible delete config file")
                                }

                                return HttpResponseHelper::internal_server_error()
                                .session_id(session.session_id)
                                .error("Something's wrong server internal error, changing passwd failed")
                                .build()
                            }
                            
                        }
                    } else {
                        
                        if self.delete_file(&full_path_file).is_err() {
                            eprintln!("Impossible delete config file")
                        }

                        return HttpResponseHelper::internal_server_error()
                        .session_id(session.session_id)
                        .error("Something's wrong in data config parsing, changing passwd failed")
                        .build()
                    }

                } else {
                    if self.delete_file(&full_path_file).is_err() {
                        eprintln!("Impossible delete config file")
                    }
                
                    return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("config_json cannot be load")
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