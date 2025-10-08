use crate::bindings::{free, pocket_group_field_new_with_params, pocket_group_field_t};
use std::ffi::{c_void, CStr, CString};
use serde::{Deserialize, Serialize};

impl pocket_group_field_t {

    pub fn to_group_field(&self) -> GroupField {
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

        GroupField {
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
        }
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

    pub fn to_pocket_group_field_t(&self) -> *mut pocket_group_field_t {
        let ret : *mut pocket_group_field_t = unsafe {
            let title = match self.title.clone() {
                Some(title) => CString::new(title).unwrap().into_raw(),
                None => std::ptr::null_mut()
            };

            let ret = pocket_group_field_new_with_params(
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

pub type GroupFields = Vec<GroupField>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_field_creation() {
        let group_field = GroupField {
            id: 1,
            server_id: 100,
            user_id: 1,
            group_id: 5,
            server_group_id: 50,
            title: Some("Field Title".to_string()),
            is_hidden: false,
            synchronized: true,
            deleted: false,
            timestamp_creation: 1640995200,
        };

        assert_eq!(group_field.id, 1);
        assert_eq!(group_field.server_id, 100);
        assert_eq!(group_field.user_id, 1);
        assert_eq!(group_field.group_id, 5);
        assert_eq!(group_field.server_group_id, 50);
        assert_eq!(group_field.title, Some("Field Title".to_string()));
        assert!(!group_field.is_hidden);
        assert!(group_field.synchronized);
        assert!(!group_field.deleted);
        assert_eq!(group_field.timestamp_creation, 1640995200);
    }

    #[test]
    fn test_group_field_with_null_title() {
        let group_field = GroupField {
            id: 2,
            server_id: 200,
            user_id: 2,
            group_id: 6,
            server_group_id: 60,
            title: None,
            is_hidden: true,
            synchronized: false,
            deleted: true,
            timestamp_creation: 1609459200,
        };

        assert_eq!(group_field.id, 2);
        assert!(group_field.title.is_none());
        assert!(group_field.is_hidden);
        assert!(!group_field.synchronized);
        assert!(group_field.deleted);
    }

    #[test]
    fn test_group_field_serialization() {
        let group_field = GroupField {
            id: 123,
            server_id: 456,
            user_id: 1,
            group_id: 2,
            server_group_id: 20,
            title: Some("Username Field".to_string()),
            is_hidden: false,
            synchronized: true,
            deleted: false,
            timestamp_creation: 1609459200,
        };

        let json = serde_json::to_string(&group_field).expect("Failed to serialize group_field");
        
        assert!(json.contains("\"id\":123"));
        assert!(json.contains("\"server_id\":456"));
        assert!(json.contains("\"user_id\":1"));
        assert!(json.contains("\"group_id\":2"));
        assert!(json.contains("\"server_group_id\":20"));
        assert!(json.contains("\"title\":\"Username Field\""));
        assert!(json.contains("\"is_hidden\":false"));
        assert!(json.contains("\"synchronized\":true"));
        assert!(json.contains("\"deleted\":false"));
        assert!(json.contains("\"timestamp_creation\":1609459200"));
    }

    #[test]
    fn test_group_field_deserialization() {
        let json = r#"{
            "id": 789,
            "server_id": 999,
            "user_id": 3,
            "group_id": 4,
            "server_group_id": 40,
            "title": "Password Field",
            "is_hidden": true,
            "synchronized": false,
            "deleted": false,
            "timestamp_creation": 1640995200
        }"#;

        let group_field: GroupField = serde_json::from_str(json).expect("Failed to deserialize group_field");
        
        assert_eq!(group_field.id, 789);
        assert_eq!(group_field.server_id, 999);
        assert_eq!(group_field.user_id, 3);
        assert_eq!(group_field.group_id, 4);
        assert_eq!(group_field.server_group_id, 40);
        assert_eq!(group_field.title, Some("Password Field".to_string()));
        assert!(group_field.is_hidden);
        assert!(!group_field.synchronized);
        assert!(!group_field.deleted);
        assert_eq!(group_field.timestamp_creation, 1640995200);
    }

    #[test]
    fn test_group_field_deserialization_null_title() {
        let json = r#"{
            "id": 101,
            "server_id": 202,
            "user_id": 1,
            "group_id": 1,
            "server_group_id": 10,
            "title": null,
            "is_hidden": false,
            "synchronized": true,
            "deleted": true,
            "timestamp_creation": 1234567890
        }"#;

        let group_field: GroupField = serde_json::from_str(json).expect("Failed to deserialize group_field");
        
        assert_eq!(group_field.id, 101);
        assert!(group_field.title.is_none());
        assert!(!group_field.is_hidden);
        assert!(group_field.synchronized);
        assert!(group_field.deleted);
        assert_eq!(group_field.timestamp_creation, 1234567890);
    }

    #[test]
    fn test_group_fields_type_alias() {
        let group_fields: GroupFields = vec![
            GroupField {
                id: 1,
                server_id: 10,
                user_id: 1,
                group_id: 1,
                server_group_id: 10,
                title: Some("Field 1".to_string()),
                is_hidden: false,
                synchronized: true,
                deleted: false,
                timestamp_creation: 1640995200,
            },
            GroupField {
                id: 2,
                server_id: 20,
                user_id: 1,
                group_id: 1,
                server_group_id: 10,
                title: Some("Field 2".to_string()),
                is_hidden: true,
                synchronized: false,
                deleted: false,
                timestamp_creation: 1640995300,
            }
        ];

        assert_eq!(group_fields.len(), 2);
        assert_eq!(group_fields[0].id, 1);
        assert_eq!(group_fields[0].title, Some("Field 1".to_string()));
        assert!(!group_fields[0].is_hidden);
        
        assert_eq!(group_fields[1].id, 2);
        assert_eq!(group_fields[1].title, Some("Field 2".to_string()));
        assert!(group_fields[1].is_hidden);
    }

    #[test]
    fn test_group_field_boolean_flags() {
        let mut group_field = GroupField {
            id: 1,
            server_id: 1,
            user_id: 1,
            group_id: 1,
            server_group_id: 1,
            title: Some("Test".to_string()),
            is_hidden: false,
            synchronized: false,
            deleted: false,
            timestamp_creation: 0,
        };

        // Test initial state
        assert!(!group_field.is_hidden);
        assert!(!group_field.synchronized);
        assert!(!group_field.deleted);

        // Modify flags
        group_field.is_hidden = true;
        group_field.synchronized = true;
        group_field.deleted = true;

        // Verify changes
        assert!(group_field.is_hidden);
        assert!(group_field.synchronized);
        assert!(group_field.deleted);
    }
}