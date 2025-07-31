use std::ffi::CString;
use crate::models::rests::DataTransport;
use crate::services::data::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use std::sync::Arc;
use crate::bindings::{pocket_field_controller_get_list_field, pocket_field_controller_t, pocket_field_free, pocket_field_t, pocket_group_controller_get_list_group, pocket_group_controller_t, pocket_group_free, pocket_group_t};
use crate::models::field::Fields;
use crate::models::group::Groups;

pub struct RestController {
    pub(super) data: Data
}

pub fn split_id_group_and_search(data_transport: &Json<DataTransport>) -> crate::utils::Result<(i64, String)> {
    match &data_transport.data {
        None => Err("No data send"),
        Some(data) => {
            let split: Vec<&str> = data.split("|").collect();

            if split.len() != 2 {
                return Err("group_id is mandatory")
            }

            let id_group = match split[0].to_string().parse::<i64>() {
                Ok(group_id) => group_id,
                Err(_) => return Err("group_id parse error"),
            };

            Ok((id_group, split[1].to_string()))
        }
    }

}

pub fn get_list_group(group_controller: *const pocket_group_controller_t, field_controller: *const pocket_field_controller_t, group_id: i64, search: &String,) -> crate::utils::Result<Groups> {
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

pub fn get_list_field(field_controller: *const pocket_field_controller_t, group_id: i64, search: &String,) -> crate::utils::Result<Fields> {
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
    
    fn new() -> Self {
        Self { 
            data: Data::init().unwrap()
        }
    }

    pub fn share() -> Arc<Self> {
        static INSTANCE: once_cell::sync::Lazy<Arc<RestController>> = once_cell::sync::Lazy::new(|| {
            Arc::new(RestController::new())
        });

        INSTANCE.clone()
    }

    pub fn debug(&self, data_transport: Json<DataTransport>) -> HttpResponse {

        let data = match &data_transport.data {
            None => return HttpResponse::Forbidden().json(DataTransport{
                error: Some("No data send".to_string()),
                ..DataTransport::default()
            }),
            Some(data) => data
        };

        HttpResponse::Ok().json(DataTransport {
            path: data.clone(),
            title: "Debug".to_string(),
            data: None,
            ..data_transport.into_inner()
        })
    }
}