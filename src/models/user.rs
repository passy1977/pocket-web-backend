use std::ffi::{c_uint, CStr};
use UserStat::{UserStatActive, UserStatDeleted, UserStatInvalidated, UserStatNotActive};
use crate::bindings::{pocket_user_t};

impl pocket_user_t {

    pub fn to_user(&self) -> User {
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
        
        User {
            id: deref_user.id,
            email,
            name,
            passwd,
            status: UserStat::from_uint(deref_user.status).unwrap(),
            timestamp_last_update: deref_user.timestamp_last_update
        }
    }
}


#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_uint;

    #[test]
    fn test_user_stat_from_uint_valid_values() {
        assert!(matches!(UserStat::from_uint(0), Some(UserStat::UserStatNotActive)));
        assert!(matches!(UserStat::from_uint(1), Some(UserStat::UserStatActive)));
        assert!(matches!(UserStat::from_uint(2), Some(UserStat::UserStatDeleted)));
        assert!(matches!(UserStat::from_uint(3), Some(UserStat::UserStatInvalidated)));
    }

    #[test]
    fn test_user_stat_from_uint_invalid_values() {
        assert_eq!(UserStat::from_uint(4), None);
        assert_eq!(UserStat::from_uint(999), None);
        assert_eq!(UserStat::from_uint(c_uint::MAX), None);
    }

    #[test]
    fn test_user_stat_enum_values() {
        // Verifica che i valori dell'enum corrispondano a quelli attesi
        assert_eq!(UserStat::UserStatNotActive as c_uint, 1);
        assert_eq!(UserStat::UserStatActive as c_uint, 0);
        assert_eq!(UserStat::UserStatDeleted as c_uint, 2);
        assert_eq!(UserStat::UserStatInvalidated as c_uint, 3);
    }

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 123,
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            passwd: "hashed_password".to_string(),
            status: UserStat::UserStatActive,
            timestamp_last_update: 1609459200, // 2021-01-01 00:00:00 UTC
        };

        assert_eq!(user.id, 123);
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.passwd, "hashed_password");
        assert!(matches!(user.status, UserStat::UserStatActive));
        assert_eq!(user.timestamp_last_update, 1609459200);
    }
}

