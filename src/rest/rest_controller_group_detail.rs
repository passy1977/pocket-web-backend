use crate::bindings::{pocket_group_free, pocket_group_controller_get, pocket_group_controller_init, pocket_group_controller_new, pocket_group_field_controller_init, pocket_group_field_controller_new};
use crate::models::group::{Group, Groups};
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::{get_list_group_field, split_group_id_and_search, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_group_controller, get_group_field_controller, get_session};
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    pub fn group_detail(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        let mut id = "".to_string();
        let (group_id, search) = match split_group_id_and_search(&data_transport, &mut id) {
            Ok((id_group, search)) => (id_group, search),
            Err(e) => return HttpResponseHelper::internal_server_error()
                .error(e)
                .build()
        };

        let id = match id.replace("|", "").parse::<i64>() {
            Ok(number) => number,
            Err(e) => return HttpResponseHelper::internal_server_error()
                .error(e.to_string())
                .build()
        };

        let group_controller = get_group_controller!(session);

        let group_field_controller = get_group_field_controller!(session);

        let group = unsafe {
            let group_ptr = pocket_group_controller_get(group_controller, id);
            if group_ptr.is_null() {
                if id == 0 {

                    let group_ptr = pocket_group_controller_get(group_controller, group_id);
                    if group_ptr.is_null() {
                        return HttpResponseHelper::internal_server_error()
                        .error("Group not found".to_string())
                        .build()
                    }
                    let group = (*group_ptr).to_group();
                    pocket_group_free(group_ptr);


                    let empty_search = "".to_string();

                    session.update_timestamp_last_update();
                    return HttpResponseHelper::ok()
                        .path("/group-detail")
                        .title("New group".to_string())
                        .session_id(session.session_id)
                        .groups(Ok(
                            vec![Group { 
                                group_id: group.group_id,
                                server_group_id: group.server_group_id,
                                ..Group::new()
                            }]
                        ))
                        .group_fields(get_list_group_field(group_field_controller, group_id, &empty_search))
                        .data(empty_search)
                        .build()
                } else {
                    return HttpResponseHelper::internal_server_error()
                    .error("Group not found".to_string())
                    .build()
                }
            } else {
                let ret = (*group_ptr).to_group();
                pocket_group_free(group_ptr);
                ret
            }
        };


        let mut title = "New group".to_string();
        if id > 0 && group.title.is_some() {
            title = group.title.clone().unwrap();
        }

        let mut groups = Groups::new();
        groups.push(group);
        let groups = Ok(groups);

        session.update_timestamp_last_update();
        HttpResponseHelper::ok()
            .path("/group-detail")
            .title(title)
            .session_id(session.session_id)
            .groups(groups)
            .group_fields(get_list_group_field(group_field_controller, if id > 0 { id } else { group_id }, &search))
            .data(search)
            .build()
    }

}