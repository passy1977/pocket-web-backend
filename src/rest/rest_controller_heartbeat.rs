use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use actix_web::HttpResponse;
use actix_web::web::Path;

impl RestController {
    pub fn heartbeat(&self, session_id: Path<String>) -> HttpResponse {

        
        match Sessions::share().get(&session_id) {
            None => 
            {
                eprintln!("Heartbeat session expired for session_id: {}", &*session_id);
                HttpResponseHelper::ok()
                    .session_id(&*session_id)
                    .data("expired")
                    .error(std::format!("Session expired for session_id: {}", &*session_id))
                    .build()
            }
            
            Some(session) => {
                if session.remote_session_handling {
                    HttpResponseHelper::ok()
                        .path("")
                        .title("")
                        .session_id(session.session_id)
                        .data("remote_session_handling")
                        .build()
                } else {
                    HttpResponseHelper::ok()
                        .path("")
                        .title("")
                        .session_id(session.session_id)
                        .build()
                }
            }
        }
    }
}
