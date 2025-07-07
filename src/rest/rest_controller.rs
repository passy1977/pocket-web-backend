use crate::models::rests::DataTransport;
use crate::services::data::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use std::sync::Arc;

pub struct RestController {
    pub(super) data: Data
}



impl RestController {
    
    fn new() -> Self {
        Self { 
            data: Data::init().unwrap()
        }
    }

    pub fn share() -> Arc<Self> {
        static INSTANCE: once_cell::sync::Lazy<Arc<RestController>> = once_cell::sync::Lazy::new(|| {
            Arc::new(RestController::new())
        });

        INSTANCE.clone()
    }

    pub fn debug(&self, data_transport: Json<DataTransport>) -> HttpResponse {

        let data = match &data_transport.data {
            None => return HttpResponse::Forbidden().json(DataTransport{
                error: Some("No data send".to_string()),
                ..DataTransport::default()
            }),
            Some(data) => data
        };

        HttpResponse::Ok().json(DataTransport {
            path: data.clone(),
            title: "Debug".to_string(),
            data: None,
            ..data_transport.into_inner()
        })
    }
    
}