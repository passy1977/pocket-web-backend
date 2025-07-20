use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;


impl RestController {
    

pub fn data(&self, data_transport: Json<DataTransport>) -> HttpResponse {
    

    let mut session = match Sessions::share().get(&*data_transport.session_id) {
        None => return HttpResponseHelper::forbidden()
            .error("Session not found")
            .build(),
        Some(session) => session
    };

    HttpResponseHelper::ok()
        .session_id(session.session_id)
        .build()
}

}