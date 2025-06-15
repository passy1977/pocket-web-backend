use jsonwebtoken::{encode, EncodingKey, Header};
use crate::constants::{DATA, jwt::SECRET};
use crate::models::rest::{Claims, DataTransport};
use crate::services::data::Data;
use crate::utils::Result;

pub fn login(request: DataTransport) -> Result<DataTransport> {
    
    let data = unsafe {
        match (&raw const DATA).read() {
            None => return Err("DATA not ready".into()),
            Some(data) => data
        }
    };
    
    let claims = Claims {
        sub: "".to_string(),
        company: "".to_string(),
        exp: 0,
        iss: data.jwt_iss.clone(),
        aud: data.jwt_aud.clone(),
    };

    let jwt = match encode(&Header::default(), &claims, &EncodingKey::from_secret(data.jwt_secret.as_bytes())) {
        Ok(token) => Some(token),
        Err(err) => return Err("Impossible generate d token".into())
    };
    
    
    Ok(DataTransport {
        path: "/registration".to_string(),
        title: "Registration".to_string(),
        jwt,
        ..DataTransport::default()
    })
}