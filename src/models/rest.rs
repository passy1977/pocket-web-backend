use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::models::field::Field;
use crate::models::group::Group;
use crate::models::group_field::GroupField;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    pub path: String,
    pub jwt: Claims,
    pub groups: Vec<Group>,
    pub group_fields: Vec<GroupField>,
    pub fields: Vec<Field>,
}

impl Request {
    fn new() -> Self {
        Self {
            path: "".to_string(),
            jwt: Claims {
                sub: "".to_string(),
                company: "".to_string(),
                exp: 0,
            },
            groups: vec![],
            group_fields: vec![],
            fields: vec![],
        }
    }
    
}

pub struct Response {

}
