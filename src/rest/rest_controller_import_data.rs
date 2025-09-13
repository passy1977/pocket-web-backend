use crate::get_session;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::{HttpResponse, Error};
use futures_util::stream::StreamExt as _;


impl RestController {


    pub async fn upload(&self, mut payload: Multipart) -> Result<HttpResponse, Error> {
        // iterate over multipart stream
        while let Some(item) = payload.next().await {
            let mut field = item?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
            }
        }

        Ok(HttpResponse::Ok().into())
    }

    pub fn import_data(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        if let Some(data) = &data_transport.data {
            if data.is_empty() {
                HttpResponseHelper::internal_server_error()
                    .error("Data it's mandatory")
                    .build()  
            } else {

            session.update_timestamp_last_update();
            HttpResponseHelper::internal_server_error()
                .error("Not Implemented")
                .build()
        } 
        
        } else {
            HttpResponseHelper::internal_server_error()
            .error("Data it's mandatory")
            .build()  
        }
    }

    
}