use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::bindings::{pocket_field_controller_init, pocket_field_controller_new, pocket_group_controller_init, pocket_group_controller_new};
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;

impl RestController {
    
pub fn main(&self, data_transport: Json<DataTransport>) -> HttpResponse {
    let (group_id, search) = match &data_transport.data {
        None => return HttpResponseHelper::forbidden()
            .error("No data send")
            .build(),
        Some(data) => {
            let split: Vec<&str> = data.split("|").collect();

            if split.len() != 2 {
                return HttpResponseHelper::forbidden()
                    .error("group_id is mandatory")
                    .build()
            }

            (split[0].to_string(), split[1].to_string())
        }
    };

    let mut session = match Sessions::share().get(&*data_transport.session_id) {
        None => return HttpResponseHelper::forbidden()
            .error("Session not found")
            .build(),
        Some(session) => session
    };

    let group_controller = unsafe {

        let group_controller = if session.group_controller.is_null() {
            session.group_controller = pocket_group_controller_new(session.pocket);
            if session.group_controller.is_null() {
                return HttpResponseHelper::internal_server_error()
                        .error("Group controller null")
                        .build()
            }

            pocket_group_controller_init(session.group_controller);

            Sessions::share().remove(&session.session_id);

            Sessions::share().add(session.clone());

            session.group_controller
        } else {
            session.group_controller
        };

        group_controller
    };

    let field_controller = unsafe {

        let field_controller = if session.field_controller.is_null() {
            session.field_controller = pocket_field_controller_new(session.pocket);
            if session.field_controller.is_null() {
                return HttpResponseHelper::internal_server_error()
                        .error("Field controller null")
                        .build()
            }

            pocket_field_controller_init(session.field_controller);

            Sessions::share().remove(&session.session_id);

            Sessions::share().add(session.clone());

            session.field_controller
        } else {
            session.field_controller
        };

        field_controller
    };
    
    HttpResponseHelper::ok().session_id("ssss").build()
}

}