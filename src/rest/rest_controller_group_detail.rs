use crate::bindings::{pocket_group_controller_get, pocket_group_controller_init, pocket_group_controller_new, pocket_group_field_controller_init, pocket_group_field_controller_new};
use crate::models::group::Groups;
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::{get_list_group_field, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_group_controller, get_group_field_controller, get_session};
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    pub fn group_detail(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        let id = match &data_transport.data {
            None => return HttpResponseHelper::internal_server_error()
                .error("Data cannot be empty")
                .build(),
            Some(data) =>
                match data.to_string().parse::<i64>() {
                    Ok(id) => id,
                    Err(e) => return HttpResponseHelper::internal_server_error()
                        .error(e.to_string())
                        .build()
                }
        };

        let group_controller = get_group_controller!(session);

        let group_field_controller = get_group_field_controller!(session);

        let groups = unsafe {
            let group_ptr = pocket_group_controller_get(group_controller, id);
            if group_ptr.is_null() {
                Err("Group not found")
            } else {
                let mut tmp_groups = Groups::new();
                tmp_groups.push((*group_ptr).to_group());
                Ok(tmp_groups)
            }
        };

        session.update_timestamp_last_update();
        let search = "".to_string();
        HttpResponseHelper::ok()
            .path("/group-detail")
            .title("Group detail")
            .session_id(session.session_id)
            .groups(groups)
            .group_fields(get_list_group_field(group_field_controller, id, &search))
            .build()
    }

}