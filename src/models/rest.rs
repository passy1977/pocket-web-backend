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
    pub jwt_iss: String,
    pub jwt_aud: String,
}


impl Claims {

    pub fn new() -> Self {
        Self {
            sub: "".to_string(),
            company: "".to_string(),
            exp: 0,
            jwt_iss: "".to_string(),
            jwt_aud: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataTransport {
    pub path: String,
    pub jwt: Option<String>,
    pub groups: Vec<Group>,
    pub group_fields: Vec<GroupField>,
    pub fields: Vec<Field>,
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
    pub fn new() -> Self {
        Self {
            path: "".to_string(),
            jwt: None,
            groups: vec![],
            group_fields: vec![],
            fields: vec![],
        }
    }
    
}
