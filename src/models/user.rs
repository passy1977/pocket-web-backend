use std::ffi::{c_uint, CStr};
use UserStat::{UserStatActive, UserStatDeleted, UserStatInvalidated, UserStatNotActive};
use crate::bindings::{pocket_user_t};

impl pocket_user_t {

    pub fn to_field(&self) -> Option<User> {
        let email;
        let name;
        let passwd;
        let deref_user: pocket_user_t;
        unsafe {
            deref_user = *self;

            email = if !deref_user.email.is_null() {
                CStr::from_ptr(deref_user.email).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

            name = if !deref_user.name.is_null() {
                CStr::from_ptr(deref_user.name).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };

            passwd = if !deref_user.passwd.is_null() {
                CStr::from_ptr(deref_user.passwd).to_string_lossy().into_owned()
                    .into()
            } else {
                None
            };
        }

        let email = email.unwrap();
        let name=  name.unwrap();
        let passwd= passwd.unwrap();
        
        Some(User {
            id: deref_user.id,
            email,
            name,
            passwd,
            status: UserStat::from_uint(deref_user.status)?,
            timestamp_last_update: deref_user.timestamp_last_update
        })
    }
}


pub enum UserStat {
    UserStatNotActive = 1,
    UserStatActive = 0,
    UserStatDeleted = 2,
    UserStatInvalidated = 3
}

impl  UserStat {
    
    pub fn from_uint(status: c_uint) -> Option<UserStat> {
    
        match status {
            0 => Some(UserStatNotActive),
            1 => Some(UserStatActive),
            2 => Some(UserStatDeleted),
            3 => Some(UserStatInvalidated),
            _ => None,
        }
    }
}

pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub passwd: String,
    pub status: UserStat,
    pub timestamp_last_update: i64,
}

