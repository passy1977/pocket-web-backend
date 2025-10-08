use crate::bindings::{free, pocket_field_new_with_params, pocket_field_t};
use std::ffi::{c_void, CStr, CString};
use serde::{Deserialize, Serialize};

impl pocket_field_t {

    pub fn to_field(&self) -> Field {
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

        Field {
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
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_new() {
        let field = Field::new();
        
        assert_eq!(field.id, 0);
        assert_eq!(field.server_id, 0);
        assert_eq!(field.user_id, 0);
        assert_eq!(field.group_id, 0);
        assert_eq!(field.server_group_id, 0);
        assert_eq!(field.group_field_id, 0);
        assert_eq!(field.server_group_field_id, 0);
        assert!(field.title.is_none());
        assert!(field.value.is_none());
        assert!(!field.is_hidden);
        assert!(!field.synchronized);
        assert!(!field.deleted);
        assert_eq!(field.timestamp_creation, 0);
    }

    #[test]
    fn test_field_creation_with_values() {
        let field = Field {
            id: 1,
            server_id: 100,
            user_id: 1,
            group_id: 5,
            server_group_id: 50,
            group_field_id: 10,
            server_group_field_id: 110,
            title: Some("Test Field".to_string()),
            value: Some("Test Value".to_string()),
            is_hidden: true,
            synchronized: true,
            deleted: false,
            timestamp_creation: 1640995200,
        };

        assert_eq!(field.id, 1);
        assert_eq!(field.server_id, 100);
        assert_eq!(field.user_id, 1);
        assert_eq!(field.group_id, 5);
        assert_eq!(field.server_group_id, 50);
        assert_eq!(field.group_field_id, 10);
        assert_eq!(field.server_group_field_id, 110);
        assert_eq!(field.title, Some("Test Field".to_string()));
        assert_eq!(field.value, Some("Test Value".to_string()));
        assert!(field.is_hidden);
        assert!(field.synchronized);
        assert!(!field.deleted);
        assert_eq!(field.timestamp_creation, 1640995200);
    }

    #[test]
    fn test_field_serialization() {
        let field = Field {
            id: 123,
            server_id: 456,
            user_id: 1,
            group_id: 2,
            server_group_id: 20,
            group_field_id: 3,
            server_group_field_id: 30,
            title: Some("Username".to_string()),
            value: Some("john_doe".to_string()),
            is_hidden: false,
            synchronized: true,
            deleted: false,
            timestamp_creation: 1609459200,
        };

        let json = serde_json::to_string(&field).expect("Failed to serialize field");
        
        assert!(json.contains("\"id\":123"));
        assert!(json.contains("\"title\":\"Username\""));
        assert!(json.contains("\"value\":\"john_doe\""));
        assert!(json.contains("\"is_hidden\":false"));
        assert!(json.contains("\"synchronized\":true"));
    }

    #[test]
    fn test_field_deserialization() {
        let json = r#"{
            "id": 789,
            "server_id": 999,
            "user_id": 2,
            "group_id": 3,
            "server_group_id": 30,
            "group_field_id": 4,
            "server_group_field_id": 40,
            "title": "Password",
            "value": "secret123",
            "is_hidden": true,
            "synchronized": false,
            "deleted": false,
            "timestamp_creation": 1640995200
        }"#;

        let field: Field = serde_json::from_str(json).expect("Failed to deserialize field");
        
        assert_eq!(field.id, 789);
        assert_eq!(field.server_id, 999);
        assert_eq!(field.user_id, 2);
        assert_eq!(field.group_id, 3);
        assert_eq!(field.server_group_id, 30);
        assert_eq!(field.group_field_id, 4);
        assert_eq!(field.server_group_field_id, 40);
        assert_eq!(field.title, Some("Password".to_string()));
        assert_eq!(field.value, Some("secret123".to_string()));
        assert!(field.is_hidden);
        assert!(!field.synchronized);
        assert!(!field.deleted);
        assert_eq!(field.timestamp_creation, 1640995200);
    }

    #[test]
    fn test_fields_type_alias() {
        let fields: Fields = vec![
            Field::new(),
            Field {
                id: 1,
                title: Some("Test".to_string()),
                ..Field::new()
            }
        ];

        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].id, 0);
        assert_eq!(fields[1].id, 1);
        assert_eq!(fields[1].title, Some("Test".to_string()));
    }

    #[test]
    fn test_field_with_null_values() {
        let field = Field {
            id: 1,
            server_id: 0,
            user_id: 1,
            group_id: 1,
            server_group_id: 0,
            group_field_id: 1,
            server_group_field_id: 0,
            title: None,
            value: None,
            is_hidden: false,
            synchronized: false,
            deleted: true,
            timestamp_creation: 0,
        };

        assert!(field.title.is_none());
        assert!(field.value.is_none());
        assert!(field.deleted);
    }
}