use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    pub fn close_session(&self, _data_transport: Json<DataTransport>) -> HttpResponse {
        HttpResponseHelper::internal_server_error()
            .error("Not Implemented")
            .build()
    }
}