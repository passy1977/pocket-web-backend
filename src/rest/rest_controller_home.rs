use crate::bindings::{pocket_field_controller_init, pocket_field_controller_new, pocket_group_controller_init, pocket_group_controller_new};
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::*;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_field_controller, get_group_controller, get_session};
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {
    

pub fn home(&self, data_transport: Json<DataTransport>) -> HttpResponse {
    let mut session = get_session!(data_transport.session_id, "Session not found");

    let mut id = "".to_string();
    let (group_id, search) = match split_group_id_and_search(&data_transport, &mut id) {
        Ok((id_group, search)) => (id_group, search),
        Err(e) => return HttpResponseHelper::internal_server_error()
            .error(e)
            .build()
    };

    let group_controller = get_group_controller!(session);

    let field_controller = get_field_controller!(session);


    Sessions::share().remove(&session.session_id, false);

    Sessions::share().add(session.clone());

    session.update_timestamp_last_update();
    HttpResponseHelper::ok()
        .path("/home")
        .title("")
        .session_id(session.session_id)
        .groups(get_list_group(group_controller, field_controller, group_id, &search))
        .fields(get_list_field(field_controller, group_id, &search))
        .build()
}

}