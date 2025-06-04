use crate::bindings::{pocket_field_new_with_args, pocket_field_t};
use std::ffi::{CStr, CString};

pub struct Field {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub server_group_id: i64,
    pub group_field_id: i64,
    pub server_group_field_id: i64,
    pub title: String,
    pub value: String,
    pub is_hidden: bool,
    pub synchronized: bool,
    pub deleted: bool,
    pub timestamp_creation: u64,
}

pub struct FieldController {
    
}

impl Field {

     fn to_field(field : *mut pocket_field_t) -> Option<Field> {
        if field.is_null() {
            return None 
        }
         
         let title_opt;
         let value_opt;
         let deref_field : pocket_field_t;
         unsafe {
            deref_field = *field;

            title_opt = if !deref_field.title.is_null() {
                CStr::from_ptr(deref_field.title).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

             value_opt = if !deref_field.value.is_null() {
                CStr::from_ptr(deref_field.value).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };
         }
         
        Some(Field {
            id: deref_field.id,
            server_id: deref_field.server_id,
            user_id: deref_field.user_id,
            group_id: deref_field.group_id,
            server_group_id: deref_field.server_group_id,
            group_field_id: deref_field.group_field_id,
            server_group_field_id: deref_field.server_group_field_id,
            title: match title_opt {
                None => return None,
                Some(str) => str
            },
            value: match value_opt {
                None => return None,
                Some(str) => str
            },
            is_hidden: deref_field.is_hidden,
            synchronized: deref_field.synchronized,
            deleted: deref_field.deleted,
            timestamp_creation: deref_field.timestamp_creation,
        })
    }


    fn to_pocket_field_t(field : Field) -> *mut pocket_field_t {
        let ret : *mut pocket_field_t = unsafe {
            let title = CString::new(field.title).unwrap().into_raw();
            let value = CString::new(field.value).unwrap().into_raw();
            
            let ret = pocket_field_new_with_args(
                field.id,
                field.server_id,
                field.user_id,
                field.group_id,
                field.server_group_id,
                field.group_field_id,
                field.server_group_field_id,
                title,
                value,
                field.is_hidden,
                field.synchronized,
                field.deleted,
                field.timestamp_creation
            );

            unsafe {
                CString::from_raw(title);
                CString::from_raw(value);
            }

            ret
        };


        return ret;
    }
}