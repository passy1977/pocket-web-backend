use crate::bindings::{free, pocket_field_new_with_params, pocket_field_t};
use std::ffi::{c_void, CStr, CString};
use serde::{Deserialize, Serialize};

impl pocket_field_t {

    pub fn to_field(&self) -> Option<Field> {
        let title;
        let value;
        let deref_field : pocket_field_t;
        unsafe {
            deref_field = *self;

            title = if !deref_field.title.is_null() {
                CStr::from_ptr(deref_field.title).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

            value = if !deref_field.value.is_null() {
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
            title,
            value,
            is_hidden: deref_field.is_hidden,
            synchronized: deref_field.synchronized,
            deleted: deref_field.deleted,
            timestamp_creation: deref_field.timestamp_creation,
        })
    }
}

#[warn(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Field {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub server_group_id: i64,
    pub group_field_id: i64,
    pub server_group_field_id: i64,
    pub title: Option<String>,
    pub value: Option<String>,
    pub is_hidden: bool,
    pub synchronized: bool,
    pub deleted: bool,
    pub timestamp_creation: u64,
}

impl Field {

    pub fn new()  -> Self {
        Self {
            id: 0,
            server_id: 0,
            user_id: 0,
            group_id: 0,
            server_group_id: 0,
            group_field_id: 0,
            server_group_field_id: 0,
            title: None,
            value: None,
            is_hidden: false,
            synchronized: false,
            deleted: false,
            timestamp_creation: 0,
        }
    }

    pub fn to_pocket_field_t(&self) -> *mut pocket_field_t {
        let ret : *mut pocket_field_t = unsafe {
            let title = match self.title.clone() {
                Some(title) => CString::new(title).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };

            let value = match self.value.clone() {
                Some(value) => CString::new(value).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };
            
            let ret = pocket_field_new_with_params(
                self.id,
                self.server_id,
                self.user_id,
                self.group_id,
                self.server_group_id,
                self.group_field_id,
                self.server_group_field_id,
                title,
                value,
                self.is_hidden,
                self.synchronized,
                self.deleted,
                self.timestamp_creation
            );

            if self.title.as_ref().is_some() {
                free(title as *mut c_void);
            }

            if self.value.as_ref().is_some() {
                free(value as *mut c_void);
            }

            ret
        };
        
        ret
    }
}

pub type Fields = Vec<Field>;