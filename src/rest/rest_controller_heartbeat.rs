use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::get_session;
use actix_web::web::Path;
use actix_web::HttpResponse;

impl RestController {

    pub fn heartbeat(&self, session_id: Path<String>) -> HttpResponse {
        let session = get_session!(session_id, "Session not found");
        
        HttpResponseHelper::ok()
        .path("")
        .title("")
        .session_id(session.session_id)
        .build()
    }

}