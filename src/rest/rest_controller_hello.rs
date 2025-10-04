use actix_web::{web::Path, HttpResponse};
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::{Session, Sessions};

impl RestController {
    

    pub fn hello(&self, session_id: Path<String>) -> HttpResponse {

        let session_id = match Sessions::share().get(session_id.as_str()) {
            None => {
                let session = Session::new();
                let session_id = session.session_id.clone();

                Sessions::share().add(session);
                session_id
            }
            Some(session) => session.session_id.clone()
        };
        
        HttpResponseHelper::ok().session_id(session_id).build()
    }

}