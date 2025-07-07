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
    pub jwt: Option<String>,
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
            jwt: None,
            groups: None,
            group_fields: None,
            fields: None,
            data: None,
            error: None
        }
    }
    
}
