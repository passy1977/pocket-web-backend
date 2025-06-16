use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::models::field::Field;
use crate::models::group::Group;
use crate::models::group_field::GroupField;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
    pub iss: String,
    pub aud: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataTransport {
    pub path: String,
    pub title: String,
    pub jwt: Option<String>,
    pub groups: Option<Vec<Group>>,
    pub group_fields: Option<Vec<GroupField>>,
    pub fields: Option<Vec<Field>>,
    pub data: Option<String>,
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
            path: "".to_string(),
            title: "".to_string(),
            jwt: None,
            groups: None,
            group_fields: None,
            fields: None,
            data: None
        }
    }
    
}
