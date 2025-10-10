use std::{self, io::Write, ffi::CString, path::MAIN_SEPARATOR, str::from_utf8, fs::File};
use crate::services::secure_session::generate_secure_session_id;
use crate::{get_field_controller, get_group_controller, get_session, perform_timestamp_last_update};
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::{delete_file, get_list_field, get_list_group, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::{HttpResponse};
use crate::services::session::Sessions;
use futures_util::stream::StreamExt as _;
use crate::bindings::{pocket_field_controller_init, pocket_field_controller_new, pocket_group_controller_data_import, pocket_group_controller_data_import_legacy, pocket_group_controller_init, pocket_group_controller_new, pocket_is_no_network};

impl RestController {
    pub async fn upload(&self, mut payload: Multipart) -> HttpResponse {
        let mut session_id = String::new();
        let mut file_legacy: String = String::new();
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
                if let Ok(tmp) = field.bytes(70).await {
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

            if field.name().unwrap_or("") == "file_legacy" {
                if let Ok(tmp) = field.bytes(3).await {
                    match tmp {
                        Ok(bytes) => {
                            if let Ok(data) = from_utf8(bytes.to_vec().as_slice()) {
                                file_legacy = data.to_string();
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

        let mut full_path_file = match self.data.dir_path.clone().as_path().to_str() {
            None => return HttpResponseHelper::not_acceptable()
                .session_id(session.session_id)
                .error("data_dir_path not found")
                .build(),
            Some(data_dir_path) => data_dir_path.to_string()
        };
        full_path_file.push(MAIN_SEPARATOR);
        full_path_file.push_str(&generate_secure_session_id());



        if writeln!(File::create(full_path_file.clone()).unwrap(), "{file}").is_err() {
            return HttpResponseHelper::not_acceptable()
                .session_id(session.session_id)
                .error("Impossible write on {full_path_file}")
                .build()
        }

        unsafe {
            if let Ok(c_full_path_file_import) = CString::new(full_path_file.clone()) {

                if file_legacy == "1" {
                    if !pocket_group_controller_data_import_legacy(session.pocket, c_full_path_file_import.as_ptr()) {
                        let _ = delete_file(&full_path_file);
                        return HttpResponseHelper::not_acceptable()
                            .session_id(session.session_id)
                            .error("Unable import data")
                            .build()
                    }
                } else {
                    if !pocket_group_controller_data_import(session.pocket, c_full_path_file_import.as_ptr()) {
                        let _ = delete_file(&full_path_file);
                        return HttpResponseHelper::not_acceptable()
                            .session_id(session.session_id)
                            .error("Unable import data")
                            .build()
                    }
                }
            }

            session.remote_session_handling = !pocket_is_no_network(session.pocket);
        }

        let _ = delete_file(&full_path_file);

        let group_controller = get_group_controller!(session);

        let field_controller = get_field_controller!(session);

        let search = String::new();
        perform_timestamp_last_update!(session);
        HttpResponseHelper::ok()
            .path("/home")
            .title("")
            .session_id(session.session_id)
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
                perform_timestamp_last_update!(session);
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
