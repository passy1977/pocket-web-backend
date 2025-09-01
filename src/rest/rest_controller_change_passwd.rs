use std::ffi::{CStr, CString};
use std::path::MAIN_SEPARATOR;
use std::str::FromStr;
use crate::bindings::pocket_change_passwd;
use crate::constants::data::EXPORT_DATA_CHANGE_PASSWD;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::get_session;
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    pub fn change_passwd(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let session = get_session!(data_transport.session_id, "Session not found");

        if let Some(data) = &data_transport.data {
            if data.is_empty() {
                HttpResponseHelper::ok()
                    .path("/change-passwd")
                    .title("Change password")
                    .session_id(session.session_id)
                    .build()
            } else {

                let mut full_path_file = match self.data.dir_path.clone().as_path().to_str() {
                    None => return HttpResponseHelper::not_acceptable()
                        .session_id(session.session_id)
                        .error("data_dit_path not found")
                        .build(),
                    Some(data_dir_path) => data_dir_path.to_string()
                };
                full_path_file.push(MAIN_SEPARATOR);
                full_path_file.push_str(EXPORT_DATA_CHANGE_PASSWD);


                let config_json = "{}".to_string();


                let full_path_file = CString::(full_path_file).as_ptr();
                let config_json = String::from_str(config_json.as_str()).unwrap().as_ptr();
                let data = String::from_str(data).unwrap().as_ptr();

                // pocket_change_passwd(session.pocket
                //     , String::from_str(full_path_file).as_ptr()
                //     , String::from_str(config_json)
                //     , String::from_str(data)
                // );

                HttpResponseHelper::internal_server_error()
                    .error("Todo")
                    .build() 
            }
        } else {
            HttpResponseHelper::internal_server_error()
            .error("Data it's mandatory")
            .build()          
        }


    }
}