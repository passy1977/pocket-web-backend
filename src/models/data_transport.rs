use crate::models::field::Fields;
use crate::models::group::Groups;
use crate::models::group_field::GroupFields;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub aud: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataTransport {
    pub path: String,
    pub title: String,
    pub session_id: String,
    pub groups: Option<Groups>,
    pub group_fields: Option<GroupFields>,
    pub fields: Option<Fields>,
    pub data: Option<String>,
    pub error: Option<String>,
}

impl FromStr for DataTransport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match from_str::<DataTransport>(&s) {
            Ok(dt) => Ok(dt),
            Err(err) => Err(err.to_string())
        }
    }
}

impl DataTransport {
    pub fn default() -> Self {
        Self {
            path: "/login".to_string(),
            title: "Login".to_string(),
            session_id: "".to_string(),
            groups: None,
            group_fields: None,
            fields: None,
            data: None,
            error: None
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_data_transport_default() {
        let dt = DataTransport::default();
        
        assert_eq!(dt.path, "/login");
        assert_eq!(dt.title, "Login");
        assert_eq!(dt.session_id, "");
        assert!(dt.groups.is_none());
        assert!(dt.group_fields.is_none());
        assert!(dt.fields.is_none());
        assert!(dt.data.is_none());
        assert!(dt.error.is_none());
    }

    #[test]
    fn test_data_transport_serialization() {
        let dt = DataTransport {
            path: "/test".to_string(),
            title: "Test".to_string(),
            session_id: "session123".to_string(),
            groups: None,
            group_fields: None,
            fields: None,
            data: Some("test_data".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&dt).expect("Failed to serialize");
        assert!(json.contains("\"path\":\"/test\""));
        assert!(json.contains("\"title\":\"Test\""));
        assert!(json.contains("\"session_id\":\"session123\""));
        assert!(json.contains("\"data\":\"test_data\""));
    }

    #[test]
    fn test_data_transport_deserialization() {
        let json = r#"{
            "path": "/api/data",
            "title": "API Data",
            "session_id": "abc123",
            "groups": null,
            "group_fields": null,
            "fields": null,
            "data": "some data",
            "error": null
        }"#;

        let dt: DataTransport = serde_json::from_str(json).expect("Failed to deserialize");
        
        assert_eq!(dt.path, "/api/data");
        assert_eq!(dt.title, "API Data");
        assert_eq!(dt.session_id, "abc123");
        assert_eq!(dt.data, Some("some data".to_string()));
        assert!(dt.error.is_none());
    }

    #[test]
    fn test_data_transport_from_str() {
        let json = r#"{
            "path": "/test",
            "title": "Test",
            "session_id": "test123",
            "groups": null,
            "group_fields": null,
            "fields": null,
            "data": null,
            "error": "Test error"
        }"#;

        let dt = DataTransport::from_str(json).expect("Failed to parse from string");
        
        assert_eq!(dt.path, "/test");
        assert_eq!(dt.title, "Test");
        assert_eq!(dt.session_id, "test123");
        assert_eq!(dt.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_data_transport_from_str_invalid_json() {
        let invalid_json = "invalid json";
        let result = DataTransport::from_str(invalid_json);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_creation() {
        let claims = Claims {
            sub: "user123".to_string(),
            exp: 1640995200, // 2022-01-01 00:00:00 UTC
            iss: "pocket-app".to_string(),
            aud: "pocket-users".to_string(),
        };

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.exp, 1640995200);
        assert_eq!(claims.iss, "pocket-app");
        assert_eq!(claims.aud, "pocket-users");
    }

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "test_user".to_string(),
            exp: 1234567890,
            iss: "test_issuer".to_string(),
            aud: "test_audience".to_string(),
        };

        let json = serde_json::to_string(&claims).expect("Failed to serialize claims");
        assert!(json.contains("\"sub\":\"test_user\""));
        assert!(json.contains("\"exp\":1234567890"));
        assert!(json.contains("\"iss\":\"test_issuer\""));
        assert!(json.contains("\"aud\":\"test_audience\""));
    }
}
