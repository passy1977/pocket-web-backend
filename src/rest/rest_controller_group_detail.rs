use crate::services::session::Sessions;
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use actix_web::web::Json;
use actix_web::HttpResponse;
use crate::get_session;

impl RestController {

    pub fn group_detail(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let session = get_session!(data_transport.session_id, "Session not found");

        let id = match &data_transport.data {
            None => return HttpResponseHelper::internal_server_error()
                .error("Data cannot be empty")
                .build(),
            Some(data) =>
                match data.to_string().parse::<i64>() {
                    Ok(id) => id,
                    Err(e) => return HttpResponseHelper::internal_server_error()
                        .error(e.to_string())
                        .build()
                }
        };




        HttpResponseHelper::ok()
            .path("/group-detail")
            .title("Login")
            .session_id(session.session_id)
            .build()
    }

}