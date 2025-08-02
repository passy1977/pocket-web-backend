use crate::bindings::{pocket_field_controller_del_field, pocket_field_controller_init, pocket_field_controller_new, pocket_field_controller_persist_field, pocket_field_controller_t, pocket_group_controller_del_group, pocket_group_controller_init, pocket_group_controller_new, pocket_group_controller_persist_group, pocket_group_controller_t, pocket_stat_t_OK};
use crate::models::group::{Group, Groups};
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::{get_list_group, split_id_group_and_search, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_field_controller, get_group_controller, get_session};
use actix_web::web::Json;
use actix_web::HttpResponse;
use crate::models::field::Field;

fn group_handler(group_controller: *mut pocket_group_controller_t, field_controller: *mut pocket_field_controller_t, data_transport: &Json<DataTransport>, kind: &String, err : &mut Option<&str>) -> bool {
    if data_transport.groups.is_none() {
        return false;
    }

    if err.is_some() {
        return false;
    }

    for group in data_transport.groups.clone().unwrap() {
        let Group { id, server_id, deleted, .. } = group;
        return match (id, server_id, deleted) {
            (id, _server_id @ 0, _deleted @ false) if id > 0 => {
                //new
                unsafe {
                    pocket_group_controller_persist_group(group_controller, group.to_pocket_group_t()) == pocket_stat_t_OK
                }
            }
            (id, server_id, _deleted @ false) if id > 0 && server_id > 0 => {
                //modify
                unsafe {
                    pocket_group_controller_persist_group(group_controller, group.to_pocket_group_t()) == pocket_stat_t_OK
                }
            }
            (id, _, _deleted @ true) if id > 0 => {
                //delete
                unsafe {
                    pocket_group_controller_del_group(group_controller, field_controller, group.to_pocket_group_t()) == pocket_stat_t_OK
                }
            }
            (_, _, _) => false
        }
    }

    true
}

fn group_field_handler(group_controller: *mut pocket_group_controller_t, field_controller: *mut pocket_field_controller_t,  data_transport: &Json<DataTransport>, kind: &String, err : &mut Option<&str>) -> bool {

    true
}

fn field_handler(field_controller: *mut pocket_field_controller_t, data_transport: &Json<DataTransport>, kind: &String, err : &mut Option<&str>) -> bool {
    if data_transport.fields.is_none() {
        return false;
    }

    if err.is_some() {
        return false;
    }

    for field in data_transport.fields.clone().unwrap() {
        let Field { id, server_id, deleted, .. } = field;
        return match (id, server_id, deleted) {
            (id, _server_id @ 0, _deleted @ false) if id > 0 => {
                //new
                unsafe {
                    pocket_field_controller_persist_field(field_controller, field.to_pocket_field_t()) == pocket_stat_t_OK
                }
            }
            (id, server_id, _deleted @ false) if id > 0 && server_id > 0 => {
                //modify
                unsafe {
                    pocket_field_controller_persist_field(field_controller, field.to_pocket_field_t()) == pocket_stat_t_OK
                }
            }
            (id, _, _deleted @ true) if id > 0 => {
                //delete
                unsafe {
                    pocket_field_controller_del_field(field_controller, field.to_pocket_field_t()) == pocket_stat_t_OK
                }
            }
            (_, _, _) => false
        }
    }
    true
}

impl RestController {

    pub fn data(&self, data_transport: Json<DataTransport>) -> HttpResponse {

        let mut session = get_session!(data_transport.session_id, "Session not found");

        let split: Vec<&str> = data_transport.path.split("/").collect();

        if split.len() < 4 {
            return HttpResponseHelper::internal_server_error()
                .error("/from/kind/action are mandatory")
                .build()
        }

        let ref from = split[1].to_string();
        let ref kind = split[2].to_string();
        let ref action = split[3].to_string();

        let mut err : Option<&str> = None;

        let group_controller = get_group_controller!(session);

        let field_controller = get_field_controller!(session);

        let status_op = match kind.as_str() {
            "group" => group_handler(group_controller, field_controller, &data_transport, &action, &mut err),
            "groupField" => group_field_handler(group_controller, field_controller, &data_transport, &action, &mut err),
            "field" => field_handler(field_controller, &data_transport, &action, &mut err),
            _ => return HttpResponseHelper::forbidden()
                .error("kind not valid")
                .build()
        };

        if err.is_some() {
            return HttpResponseHelper::forbidden()
                .error(err.unwrap())
                .build()
        }

        match from.as_str() {
            "home" => self.home(data_transport),
            _ => HttpResponseHelper::forbidden()
                .error("fron not valid")
                .build()
        }

    }

}