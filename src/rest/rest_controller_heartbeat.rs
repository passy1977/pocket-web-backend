use crate::bindings::{pocket_heartbeat, pocket_is_no_network};
use crate::perform_timestamp_last_update;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use actix_web::{HttpResponse, HttpRequest};
use actix_web::web::Path;
use crate::services::rate_limiter::check_rate_limit_or_reject;

impl RestController {
    pub fn heartbeat(&self, req: HttpRequest, session_id: Path<String>) -> HttpResponse {
        
        // Check rate limiting for heartbeat endpoint
        if let Some(response) = check_rate_limit_or_reject(&req, "/v5/pocket/heartbeat", Some(&session_id)) {
            return response;
        }
        
        match Sessions::share().get(&session_id) {
            None => 
            {
                #[cfg(debug_assertions)]
                eprintln!("Heartbeat session expired for session_id: {}", &*session_id);
                HttpResponseHelper::ok()
                    .session_id(&*session_id)
                    .data("expired")
                    .error(std::format!("Session expired for session_id: {}", &*session_id))
                    .build()
            }
            
            Some(mut session) => {
                if !session.is_valid() {
                    #[cfg(debug_assertions)]
                    eprintln!("Heartbeat invalid session for session_id: {}", &*session_id);
                    Sessions::share().remove(&*session_id, true);
                    return HttpResponseHelper::ok()
                        .session_id(&*session_id)
                        .data("expired")
                        .error(std::format!("Invalid session for session_id: {}", &*session_id))
                        .build();
                }

                if session.remote_session_handling {
                    if unsafe { pocket_heartbeat(session.pocket, &mut session.timestamp_last_update) } {
                        HttpResponseHelper::ok()
                        .path("")
                        .title("")
                        .session_id(session.session_id)
                        .build()
                    } else {
                        if unsafe { !pocket_is_no_network(session.pocket) } {
                            #[cfg(debug_assertions)]
                            eprintln!("Heartbeat remote session expired for session_id: {}", &*session_id);
                            Sessions::share().remove(&*session_id, true);
                            HttpResponseHelper::ok()
                                .session_id(&*session_id)
                                .data("expired")
                                .error(std::format!("Remote session expired for session_id: {}", &*session_id))
                                .build()
                        } else {
                            session.remote_session_handling = false;
                            session.update_timestamp_last_update();
                            perform_timestamp_last_update!(session);

                            HttpResponseHelper::ok()
                                .path("")
                                .title("")
                                .session_id(session.session_id)
                                .build()
                        }
                    }
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
