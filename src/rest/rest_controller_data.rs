use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::rest::rest_controller_home::split_id_group_and_search;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;


impl RestController {
    

pub fn data(&self, data_transport: Json<DataTransport>) -> HttpResponse {
    let (group_id, search) = match split_id_group_and_search(&data_transport) {
        Ok((id_group, search)) => (id_group, search),
        Err(e) => return HttpResponseHelper::internal_server_error()
            .error(e)
            .build()
    };

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