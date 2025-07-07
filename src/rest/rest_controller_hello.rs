use actix_web::{web, HttpResponse};

use crate::{ models::rests::DataTransport, rest::rest_controller::RestController, services::session::{Session, Sessions}};


impl RestController {
    

    pub fn hello(&self, session_id: web::Path<String>) -> HttpResponse {

        let mut session_id_handler = "".to_string();
        if !session_id.is_empty() {
            session_id_handler = session_id.clone();
        }

        let session_id = match Sessions::share().get(session_id_handler.as_str()) {
            None => {
                let session = Session::new();
                let session_id = session.session_id.clone();

                Sessions::share().add(session);
                session_id
            }
            Some(session) => session.session_id.clone()
        };
        
        HttpResponse::Ok().json(crate::models::rests::DataTransport{
            session_id,
            ..DataTransport::default()   
        })
    }

}