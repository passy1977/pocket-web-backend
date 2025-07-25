use crate::bindings::{pocket_field_controller_get_list_field, pocket_field_controller_init, pocket_field_controller_new, pocket_field_controller_t, pocket_field_free, pocket_field_t, pocket_group_controller_get_list_group, pocket_group_controller_init, pocket_group_controller_new, pocket_group_controller_t, pocket_group_free, pocket_group_t};
use crate::models::field::Fields;
use crate::models::group::Groups;
use crate::models::rests::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::utils::Result;
use actix_web::web::Json;
use actix_web::HttpResponse;
use std::ffi::CString;

fn get_list_group(group_controller: *const pocket_group_controller_t, field_controller: *const pocket_field_controller_t, group_id: i64, search: &String,) -> Result<Groups> {
    let mut ret = Groups::new();

    unsafe {

        let mut count = Box::new(0i32);
        let groups_ptr = pocket_group_controller_get_list_group(
            group_controller,
            field_controller,
            group_id,
            CString::new(search.clone()).expect("search::new failed").as_ptr(),
            count.as_mut()
        );
        if groups_ptr.is_null() {
            return Err("Groups it's null")
        }

        for i in 0i32..*count {
            let group_ptr = *groups_ptr.offset(i as isize);
            if !group_ptr.is_null() {
                let pocket_group: pocket_group_t = std::ptr::read(group_ptr);
                ret.push(pocket_group.to_group());
                pocket_group_free(group_ptr);
            }
        }
    }

    Ok(ret)
}

fn get_list_field(field_controller: *const pocket_field_controller_t, group_id: i64, search: &String,) -> Result<Fields> {
    let mut ret = Fields::new();

    unsafe {

        let mut count = Box::new(0i32);
        let fields_ptr = pocket_field_controller_get_list_field(
            field_controller,
            group_id,
            CString::new(search.clone()).expect("search::new failed").as_ptr(),
            count.as_mut()
        );
        if fields_ptr.is_null() {
            return Err("Groups it's null")
        }

        for i in 0i32..*count {
            let field_ptr = *fields_ptr.offset(i as isize);
            if !field_ptr.is_null() {
                let pocket_field: pocket_field_t = std::ptr::read(field_ptr);
                ret.push(pocket_field.to_field());
                pocket_field_free(field_ptr);
            }
        }
    }

    Ok(ret)
}

impl RestController {
    

pub fn home(&self, data_transport: Json<DataTransport>) -> HttpResponse {
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

            (match split[0].to_string().parse::<i64>() {
                Ok(group_id) => group_id,
                Err(e) => return HttpResponseHelper::internal_server_error()
                    .error(e.to_string())
                    .build()
            }, split[1].to_string())
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


            session.field_controller
        } else {
            session.field_controller
        };

        field_controller
    };
    

    Sessions::share().remove(&session.session_id, false);

    Sessions::share().add(session.clone());


    HttpResponseHelper::ok()
        .session_id(session.session_id)
        .groups(get_list_group(group_controller, field_controller, group_id, &search))
        .fields(get_list_field(field_controller, group_id, &search))
        .build()
}

}