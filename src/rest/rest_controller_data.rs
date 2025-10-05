use crate::bindings::{pocket_field_controller_del, pocket_field_controller_init, pocket_field_controller_new, pocket_field_controller_persist, pocket_field_controller_t, pocket_group_controller_del, pocket_group_controller_init, pocket_group_controller_new, pocket_group_controller_persist, pocket_group_controller_t, pocket_group_field_controller_del, pocket_group_field_controller_init, pocket_group_field_controller_new, pocket_group_field_controller_persist, pocket_group_field_controller_t, pocket_is_no_network, pocket_send_data, pocket_stat_t_OK, pocket_stat_t_READY};
use crate::models::field::Field;
use crate::models::group::Group;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::{split_group_id_and_search, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_field_controller, get_group_controller, get_group_field_controller, get_session, perform_timestamp_last_update};
use actix_web::web::Json;
use actix_web::HttpResponse;
use crate::constants::Stats;
use crate::models::group_field::GroupField;
use crate::utils::are_sets_equal;

fn group_handler(group_controller: *mut pocket_group_controller_t, group_field_controller: *mut pocket_group_field_controller_t, field_controller: *mut pocket_field_controller_t, data_transport: &mut Json<DataTransport>, _from: &String, _kind: &String, _action: &String, err : &mut Option<&str>) -> bool {
    if data_transport.groups.is_none() {
        return false;
    }

    if  data_transport.groups.clone().unwrap().is_empty() {
        return true;
    }

    if err.is_some() {
        return false;
    }

    let mut new_groups = data_transport.groups.clone().unwrap();
    for group in &mut new_groups {

        let group_c = group.to_pocket_group_t();
        if group_c.is_null() {
            return false;
        }

        let Group { id, server_id: _server_id, deleted, .. } = group;
        match (id, _server_id, deleted) {
            (id, _server_id @ 0, _deleted @ false) if *id <= 0 => {
                //new
                unsafe {
                    let rc = pocket_group_controller_persist(group_controller, group_c);
                    if rc == pocket_stat_t_OK || rc == pocket_stat_t_READY {
                        group.id = (*group_c).id;
                    } else {
                        return false
                    }
                }
            }
            (id, _, _deleted @ false) if *id > 0 => {
                //modify
                unsafe {
                    let rc = pocket_group_controller_persist(group_controller, group_c);
                    if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                        return false
                    }
                }
            }
            (_, _, _deleted @ true) => {
                //delete
                unsafe {
                    let rc = pocket_group_controller_del(group_controller, group_field_controller, field_controller, group_c);
                    if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                        return false
                    }
                }
            }
            (_, _, _) => return false
        }
    }

    data_transport.groups = Some(new_groups);

    true
}

fn group_field_handler(group_field_controller: *mut pocket_group_field_controller_t, data_transport: &mut Json<DataTransport>, _from: &String, _kind: &String, _action: &String, err : &mut Option<&str>) -> bool {
    if data_transport.group_fields.is_none() {
        return false;
    }

    if  data_transport.group_fields.clone().unwrap().is_empty() {
        return true;
    }

    if err.is_some() {
        return false;
    }

    // let mut tuple: (i64, i64) = (0, 0);
    // if from == "group_detail" {
    //     tuple = if let Some(group) = data_transport.groups.clone().unwrap().get(0) {
    //         (group.id, group.server_id)
    //     } else {
    //         return false;
    //     };
    // }
    let tuple = if let Some(group) = data_transport.groups.clone().unwrap().get(0) {
        (group.id, group.server_id)
    } else {
        return false;
    };

    let mut new_group_fields = data_transport.group_fields.clone().unwrap();
    for ref mut group_field in &mut new_group_fields {

        // if from == "group_detail" {
        group_field.group_id = tuple.0;
        group_field.server_group_id = tuple.1;
        // }

        let group_field_c = group_field.to_pocket_group_field_t();
        if group_field_c.is_null() {
            return false;
        }

        let GroupField { id, server_id: _server_id, deleted, .. } = group_field;
        match (id, _server_id, deleted) {
            (id, _server_id @ 0, _deleted @ false) if *id <= 0 => {
                //new
                unsafe {
                    let rc = pocket_group_field_controller_persist(group_field_controller, group_field_c);
                    if rc == pocket_stat_t_OK || rc == pocket_stat_t_READY {
                        group_field.id = (*group_field_c).id;
                    } else {
                        return false
                    }
                }
            }
            (id, _, _deleted @ false) if *id > 0 => {
                //modify
                unsafe {
                    let rc = pocket_group_field_controller_persist(group_field_controller, group_field_c);
                    if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                        return false
                    }
                }
            }
            (_, _, _deleted @ true) => {
                //delete
                unsafe {
                    let rc = pocket_group_field_controller_del(group_field_controller, group_field_c);
                    if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                        return false
                    }
                }
            }
            (_, _, _) => return false
        }
    }

    data_transport.group_fields = Some(new_group_fields);

    true
}

fn field_handler(field_controller: *mut pocket_field_controller_t, data_transport: &mut Json<DataTransport>, _from: &String, _kind: &String, _action: &String, err : &mut Option<&str>) -> bool {
    if data_transport.fields.is_none() {
        return false;
    }

    if  data_transport.fields.clone().unwrap().is_empty() {
        return true;
    }

    if err.is_some() {
        return false;
    }
    
    let tuple= if data_transport.groups.is_some() {
        if let Some(group) = data_transport.groups.clone().unwrap().get(0) {
            Some((group.id, group.server_id))
        } else {
            None
        }
    } else {
        None
    };

    let mut idx = 0;
    let mut new_fields = data_transport.fields.clone().unwrap();
    for ref mut field in &mut new_fields {

        if let Some((group_id, server_group_id)) = tuple {
            field.group_id = group_id;
            field.server_group_id = server_group_id;
        }
        
        if data_transport.group_fields.is_some() {
            if let Some(group_field) = data_transport.group_fields.clone().unwrap().get(idx) {
                field.group_field_id = group_field.id;
                field.server_group_field_id= group_field.server_id;
            } else {
                return false;
            }
        }


        let field_c = field.to_pocket_field_t();
        if field_c.is_null() {
            return false;
        }

        let Field { id, server_id: _server_id, deleted, .. } = field;
        match (id, _server_id, deleted) {
            (id, _server_id @ 0, _deleted @ false) if *id <= 0 => {
                //new
                unsafe {
                    let rc = pocket_field_controller_persist(field_controller, field_c);
                    if rc == pocket_stat_t_OK || rc == pocket_stat_t_READY {
                        field.id = (*field_c).id;
                    } else {
                        return false
                    }
                }
            }
            (id, _, _deleted @ false) if *id > 0 => {
                //modify
                unsafe {
                    let rc = pocket_field_controller_persist(field_controller, field_c);
                    if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                        return false
                    }
                }
            }
            (_, _, _deleted @ true) => {
                //delete
                unsafe {
                    let rc = pocket_field_controller_del(field_controller, field_c);
                    if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                        return false
                    }
                }
            }
            (_, _, _) => return false
        }
        idx += 1;
    }

    data_transport.fields = Some(new_fields);

    true
}


impl RestController {

    pub fn data(&self, mut data_transport: Json<DataTransport>) -> HttpResponse {

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

        let group_controller = get_group_controller!(session);

        let group_field_controller = get_group_field_controller!(session);

        let field_controller = get_field_controller!(session);

        let mut err : Option<&str> = None;

        if data_transport.groups.is_some() {
            let original_groups = data_transport.groups.clone().unwrap();

            group_handler(group_controller, group_field_controller, field_controller, &mut data_transport, &from, &kind, &action, &mut err);

            if !are_sets_equal(&original_groups, &data_transport.groups.clone().unwrap()) {
                let tmp = data_transport.groups.clone().unwrap();
                let Group{id, group_id, .. } = tmp.get(0).unwrap();

                let mut _id = "".to_string();
                let (_, search) = match split_group_id_and_search(&data_transport, &mut _id) {
                    Ok((id_group, search)) => (id_group, search),
                    Err(e) => return HttpResponseHelper::internal_server_error()
                        .error(e)
                        .build()
                };

                data_transport.data = Some(format!("{group_id}|{search}|{id}").to_string());
            }
        }

        if data_transport.group_fields.is_some() {
            group_field_handler(group_field_controller, &mut data_transport, &from, &kind, &action, &mut err);
        }

        if data_transport.fields.is_some() {
            field_handler(field_controller, &mut data_transport, &from, &kind, &action, &mut err);
        }

        unsafe  {
            let rc = pocket_send_data(session.pocket);
            if rc != pocket_stat_t_OK && rc != pocket_stat_t_READY {
                eprintln!("Impossible to send data rc:{}({rc})", Stats::to_string(rc));
            }
        }

        if err.is_some() {
            return HttpResponseHelper::forbidden()
                .error(err.unwrap())
                .build()
        }

        unsafe {
            session.remote_session_handling = !pocket_is_no_network(session.pocket);
        }
        perform_timestamp_last_update!(session);
        self.home(data_transport)

    }

}