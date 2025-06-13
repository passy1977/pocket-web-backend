use crate::bindings::{free, pocket_group_init_with_id, pocket_group_t};
use serde::{Deserialize, Serialize};
use std::ffi::{c_void, CStr, CString};

impl pocket_group_t {

    pub fn to_group(&self) -> Option<Group> {
        let title;
        let icon;
        let note;
        let deref_group: pocket_group_t;
        unsafe {
            deref_group = *self;

            title = if !deref_group.title.is_null() {
                CStr::from_ptr(deref_group.title).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };
            
            icon = if !deref_group.icon.is_null() {
                CStr::from_ptr(deref_group.icon).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

            note = if !deref_group.note.is_null() {
                CStr::from_ptr(deref_group.note).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };
        }

        Some(Group {
            id: deref_group.id,
            server_id: deref_group.server_id,
            user_id: deref_group.user_id,
            group_id: deref_group.group_id,
            server_group_id: deref_group.server_group_id,
            title,
            icon,
            note,
            is_hidden: deref_group.is_hidden,
            synchronized: deref_group.synchronized,
            deleted: deref_group.deleted,
            timestamp_creation: deref_group.timestamp_creation,
        })
    }
}

#[warn(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub server_group_id: i64,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub note: Option<String>,
    pub is_hidden: bool,
    pub synchronized: bool,
    pub deleted: bool,
    pub timestamp_creation: u64,
}


impl Group {

    pub fn new()  -> Self {
        Self {
            id: 0,
            server_id: 0,
            user_id: 0,
            group_id: 0,
            server_group_id: 0,
            title: None,
            icon: None,
            note: None,
            is_hidden: false,
            synchronized: false,
            deleted: false,
            timestamp_creation: 0,
        }
    }

    pub fn to_pocket_field_t(&self) -> *mut pocket_group_t {
        let ret : *mut pocket_group_t = unsafe {
            let title = match self.title.clone() {
                Some(title) => CString::new(title).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };

            let icon = match self.icon.clone() {
                Some(icon) => CString::new(icon).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };

            let note = match self.note.clone() {
                Some(note) => CString::new(note).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };

            let ret = pocket_group_init_with_id(
                self.id,
                self.server_id,
                self.user_id,
                self.group_id,
                self.server_group_id,
                title,
                icon,
                note,
                self.synchronized,
                self.deleted,
                self.timestamp_creation
            );

            if self.title.as_ref().is_some() {
                free(title as *mut c_void);
            }

            if self.icon.as_ref().is_some() {
                free(icon as *mut c_void);
            }

            if self.note.as_ref().is_some() {
                free(note as *mut c_void);
            }

            ret
        };


        ret
    }
}


