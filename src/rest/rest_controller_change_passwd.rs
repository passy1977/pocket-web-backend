use crate::models::data_transport::{self, DataTransport};
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use actix_web::web::Json;
use actix_web::HttpResponse;
use crate::{get_session};
use crate::services::session::Sessions;

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