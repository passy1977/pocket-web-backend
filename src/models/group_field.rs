use crate::bindings::{free, pocket_group_field_init_with_id, pocket_group_field_t};
use std::ffi::{c_void, CStr, CString};
use serde::{Deserialize, Serialize};

impl pocket_group_field_t {

    pub fn to_field(&self) -> Option<GroupField> {
        let title;
        let deref_group_field: pocket_group_field_t;
        unsafe {
            deref_group_field = *self;

            title = if !deref_group_field.title.is_null() {
                CStr::from_ptr(deref_group_field.title).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

        }

        Some(GroupField {
            id: deref_group_field.id,
            server_id: deref_group_field.server_id,
            user_id: deref_group_field.user_id,
            group_id: deref_group_field.group_id,
            server_group_id: deref_group_field.server_group_id,
            title,
            is_hidden: deref_group_field.is_hidden,
            synchronized: deref_group_field.synchronized,
            deleted: deref_group_field.deleted,
            timestamp_creation: deref_group_field.timestamp_creation,
        })
    }
}

#[warn(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GroupField {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub server_group_id: i64,
    pub title: Option<String>,
    pub is_hidden: bool,
    pub synchronized: bool,
    pub deleted: bool,
    pub timestamp_creation: u64,
}

impl GroupField {

    pub fn new()  -> Self {
        Self {
            id: 0,
            server_id: 0,
            user_id: 0,
            group_id: 0,
            server_group_id: 0,
            title: None,
            is_hidden: false,
            synchronized: false,
            deleted: false,
            timestamp_creation: 0,
        }
    }

    pub fn to_pocket_field_t(&self) -> *mut pocket_group_field_t {
        let ret : *mut pocket_group_field_t = unsafe {
            let title = match self.title.clone() {
                Some(title) => CString::new(title).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };

            let ret = pocket_group_field_init_with_id(
                self.id,
                self.server_id,
                self.user_id,
                self.group_id,
                self.server_group_id,
                title,
                self.is_hidden,
                self.synchronized,
                self.deleted,
                self.timestamp_creation
            );

            if self.title.as_ref().is_some() {
                free(title as *mut c_void);
            }
            
            ret
        };


        ret
    }
}

