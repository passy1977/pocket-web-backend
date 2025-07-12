use std::error::Error;
use std::ffi::CString;
use std::ops::Deref;
use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::bindings::{pocket_group_controller_get_list_group, pocket_field_controller_init, pocket_field_controller_new, pocket_group_controller_init, pocket_group_controller_new};
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

            (match split[0].to_string().parse::<u32>() {
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


    unsafe {

        let mut count = Box::new(1i32);
        let groups_ptr = pocket_group_controller_get_list_group(
            group_controller,
            group_id,
            CString::new(search).expect("search::new failed").as_ptr(),
            count.as_mut()
        );
        if groups_ptr.is_null() {
            return HttpResponseHelper::internal_server_error()
                .error("Groups it's null")
                .build()
        }

        let mut groups = Vec::with_capacity(*count as usize);
        for i in 0i32..*count {
            let group_ptr = *groups_ptr.offset(i as isize);
            if !group_ptr.is_null() {
                let group = std::ptr::read(group_ptr);
                groups.push(group);
            }
        }

    }


    HttpResponseHelper::ok().session_id("ssss").build()
}

}