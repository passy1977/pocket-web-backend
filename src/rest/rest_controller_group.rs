use std::{ffi::CString, str::FromStr};

use actix_web::{web::Json, HttpResponse};

use crate::services::http_response_helper::HttpResponseHelper;
use crate::{bindings::{pocket_initialize, pocket_login}, constants::Stats, models::rests::{Claims, DataTransport}, rest::rest_controller::RestController, services::session::Sessions};

impl RestController {
    
pub fn group(&self, data_transport: Json<DataTransport>) -> HttpResponse {
    // let mut session_id_handler = "".to_string();
    // if !session_id.is_empty() {
    //     session_id_handler = session_id.clone();
    // }

    // let session_id = match Sessions::share().get(session_id_handler.as_str()) {
    //     None => {
    //         let session = Session::new();
    //         let session_id = session.session_id.clone();

    //         Sessions::share().add(session);
    //         session_id
    //     }
    //     Some(session) => session.session_id.clone()
    // };
    
    HttpResponseHelper::ok().session_id("ssss").build()
}

}