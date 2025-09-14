use std::str::from_utf8;

use crate::{get_field_controller, get_group_controller, get_session};
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::{get_list_field, get_list_group, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::{HttpResponse};
use crate::services::session::Sessions;
use futures_util::stream::StreamExt as _;
use crate::bindings::{pocket_field_controller_new, pocket_field_controller_init, pocket_group_controller_init, pocket_group_controller_new};

impl RestController {
    pub async fn upload(&self, mut payload: Multipart) -> HttpResponse {
        let mut session_id = String::new();
        let mut file: String = String::new();

        while let Some(item) = payload.next().await {
            let mut field = match item {
                Ok(item) => item,
                Err(err) => {
                    return HttpResponseHelper::internal_server_error()
                        .error(err.to_string())
                        .build();
                }
            };

            if field.name().unwrap_or("") == "session_id" {
                if let Ok(tmp) = field.bytes(30).await {
                    match tmp {
                        Ok(bytes) => {
                            if let Ok(data) = from_utf8(bytes.to_vec().as_slice()) {
                                session_id = data.to_string();
                            }
                        }
                        Err(err) => {
                            return HttpResponseHelper::internal_server_error()
                                .error(err.to_string())
                                .build();
                        }
                    }
                }
            }

            if field.name().unwrap_or("") == "file" {
                while let Some(chunk) = field.next().await {
                    let chunk = match chunk {
                        Ok(chunk) => chunk,
                        Err(err) => {
                            return HttpResponseHelper::internal_server_error()
                                .error(err.to_string())
                                .build();
                        }
                    };

                    if let Ok(chunk_string) = String::from_utf8(chunk.to_vec()) {
                        file.push_str(&chunk_string);
                    }
                }
            }
        }

        let mut session = get_session!(session_id, "Session not found");

        unsafe {
            pocket_group_controller_data_import(self_, full_path_file_import);
        }

        let group_controller = get_group_controller!(session);

        let field_controller = get_field_controller!(session);

        let search = String::new();
        session.update_timestamp_last_update();
        HttpResponseHelper::ok()
            .path("/home")
            .title("")
            .session_id(session_id)
            .groups(get_list_group(
                group_controller,
                field_controller,
                0,
                &search,
            ))
            .fields(get_list_field(field_controller, 0, &search))
            .build()
    }

    pub fn import_data(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        if let Some(data) = &data_transport.data {
            if data.is_empty() {
                session.update_timestamp_last_update();
                HttpResponseHelper::ok()
                    .path("/import-data")
                    .title("Import data")
                    .session_id(session.session_id)
                    .build()
            } else {
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
