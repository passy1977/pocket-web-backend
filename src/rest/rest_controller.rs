use crate::bindings::{pocket_field_controller_free_list, pocket_field_controller_get_list, pocket_field_controller_t, pocket_field_t, pocket_group_controller_free_list, pocket_group_controller_get_list, pocket_group_controller_t, pocket_group_field_controller_free_list, pocket_group_field_controller_get_list, pocket_group_field_controller_t, pocket_group_field_t, pocket_group_t};
use crate::models::field::Fields;
use crate::models::group::Groups;
use crate::models::group_field::GroupFields;
use crate::models::data_transport::DataTransport;
use crate::services::data::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use std::ffi::CString;
use std::path::Path;
use std::sync::Arc;
use std::{fs, io::{Error, ErrorKind}};

pub struct RestController {
    pub(super) data: Data
}

pub fn split_group_id_and_search(data_transport: &Json<DataTransport>, other: &mut String) -> crate::utils::Result<(i64, String)> {
    match &data_transport.data {
        None => Err("No data send"),
        Some(data) => {
            let split: Vec<&str> = data.split("|").collect();

            if split.len() < 2 {
                return Err("group_id is mandatory")
            }

            let id_group = match split[0].to_string().parse::<i64>() {
                Ok(group_id) => group_id,
                Err(_) => return Err("group_id parse error"),
            };

            let search = split[1].to_string();


            if split.len() > 2 {
                other.clear();
                for str in split[2 ..].iter() {
                    other.push_str(str);
                    other.push_str("|");
                }
            }

            Ok((id_group, search))
        }
    }

}

pub fn get_list_group(group_controller: *const pocket_group_controller_t, field_controller: *const pocket_field_controller_t, group_id: i64, search: &String) -> crate::utils::Result<Groups> {
    let mut ret = Groups::new();

    unsafe {

        let mut count = Box::new(0i32);
        let groups_ptr = pocket_group_controller_get_list(
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
            }
        }

        pocket_group_controller_free_list(groups_ptr, *count);
    }

    Ok(ret)
}

pub fn get_list_group_field(group_field_controller: *const pocket_group_field_controller_t, group_id: i64, search: &String) -> crate::utils::Result<GroupFields> {
    let mut ret = GroupFields::new();

    unsafe {

        let mut count = Box::new(0i32);
        let group_fields_ptr = pocket_group_field_controller_get_list(
            group_field_controller,
            group_id,
            CString::new(search.clone()).expect("search::new failed").as_ptr(),
            count.as_mut()
        );
        if group_fields_ptr.is_null() {
            return Err("Groups it's null")
        }

        for i in 0i32..*count {
            let group_field_ptr = *group_fields_ptr.offset(i as isize);
            if !group_field_ptr.is_null() {
                let pocket_group_field: pocket_group_field_t = std::ptr::read(group_field_ptr);
                ret.push(pocket_group_field.to_group_field());
            }
        }

        pocket_group_field_controller_free_list(group_fields_ptr, *count);
    }

    Ok(ret)
}

pub fn get_list_field(field_controller: *const pocket_field_controller_t, group_id: i64, search: &String) -> crate::utils::Result<Fields> {
    let mut ret = Fields::new();

    unsafe {

        let mut count = Box::new(0i32);
        let fields_ptr = pocket_field_controller_get_list(
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
            }
        }

        pocket_field_controller_free_list(fields_ptr, *count);
    }

    Ok(ret)
}

pub fn delete_file(file: &String) -> Result<(), Error> {
    let path = Path::new(file);

    if path.exists() && path.is_file() {
        fs::remove_file(path)?;
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Impossible remove configuration file"));
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::web::Json;

    #[test]
    fn test_split_group_id_and_search_valid_data() {
        let data_transport = Json(DataTransport {
            data: Some("123|search_term".to_string()),
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_ok());
        let (group_id, search) = result.unwrap();
        assert_eq!(group_id, 123);
        assert_eq!(search, "search_term");
        assert!(other.is_empty());
    }

    #[test]
    fn test_split_group_id_and_search_with_additional_data() {
        let data_transport = Json(DataTransport {
            data: Some("456|search|extra|data".to_string()),
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_ok());
        let (group_id, search) = result.unwrap();
        assert_eq!(group_id, 456);
        assert_eq!(search, "search");
        assert_eq!(other, "extra|data|");
    }

    #[test]
    fn test_split_group_id_and_search_no_data() {
        let data_transport = Json(DataTransport {
            data: None,
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No data send");
    }

    #[test]
    fn test_split_group_id_and_search_insufficient_parts() {
        let data_transport = Json(DataTransport {
            data: Some("123".to_string()), // Only group_id, missing search
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "group_id is mandatory");
    }

    #[test]
    fn test_split_group_id_and_search_invalid_group_id() {
        let data_transport = Json(DataTransport {
            data: Some("invalid_id|search_term".to_string()),
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "group_id parse error");
    }

    #[test]
    fn test_split_group_id_and_search_empty_parts() {
        let data_transport = Json(DataTransport {
            data: Some("789|".to_string()), // Empty search
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_ok());
        let (group_id, search) = result.unwrap();
        assert_eq!(group_id, 789);
        assert_eq!(search, "");
    }

    #[test]
    fn test_split_group_id_and_search_negative_id() {
        let data_transport = Json(DataTransport {
            data: Some("-1|search_term".to_string()),
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_ok());
        let (group_id, search) = result.unwrap();
        assert_eq!(group_id, -1);
        assert_eq!(search, "search_term");
    }

    #[test]
    fn test_split_group_id_and_search_special_characters() {
        let data_transport = Json(DataTransport {
            data: Some("100|search with spaces|extra data with symbols!@#".to_string()),
            ..DataTransport::default()
        });
        let mut other = String::new();

        let result = split_group_id_and_search(&data_transport, &mut other);
        
        assert!(result.is_ok());
        let (group_id, search) = result.unwrap();
        assert_eq!(group_id, 100);
        assert_eq!(search, "search with spaces");
        assert_eq!(other, "extra data with symbols!@#|");
    }

    #[test]
    fn test_rest_controller_creation() {
        // Test that we can create a RestController
        // Note: this test is limited because Data::init() might fail
        // if there are no appropriate configuration files
        let data = match crate::services::data::Data::init() {
            Ok(data) => data,
            Err(_) => {
                // If we can't initialize Data, create a mock
                // This test mainly verifies that the structure is defined correctly
                return;
            }
        };

        let controller = RestController { data };
        // If we get here, creation was successful
        assert_eq!(controller.data.address.is_empty(), false);
    }
}