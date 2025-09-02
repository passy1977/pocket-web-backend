use crate::get_session;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use actix_web::web::Json;
use actix_web::HttpResponse;


impl RestController {

    pub fn close_session(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        session.update_timestamp_last_update();
        HttpResponseHelper::internal_server_error()
            .error("Not Implemented")
            .build()
    }
}