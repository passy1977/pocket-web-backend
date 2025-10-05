use crate::bindings::{pocket_field_controller_new, pocket_field_free};
use crate::bindings::{pocket_field_controller_get, pocket_field_controller_init};
use crate::models::data_transport::DataTransport;
use crate::models::field::{Field, Fields};
use crate::rest::rest_controller::{split_group_id_and_search, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_field_controller, get_session, perform_timestamp_last_update};
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    pub fn field_detail(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        let mut id = "".to_string();
        let (group_id, _) = match split_group_id_and_search(&data_transport, &mut id) {
            Ok((id_group, search)) => (id_group, search),
            Err(e) => return HttpResponseHelper::internal_server_error()
                .error(e)
                .build()
        };

        let id = match id.replace("|", "").parse::<i64>() {
            Ok(number) => number,
            Err(e) => return HttpResponseHelper::internal_server_error()
                .error(e.to_string())
                .build()
        };


        let field_controller = get_field_controller!(session);
        
        let field = unsafe {
            let field_ptr = pocket_field_controller_get(field_controller, id);
            if field_ptr.is_null() {
                if id == 0 {

                    let field_ptr = pocket_field_controller_get(field_controller, group_id);
                    if field_ptr.is_null() {
                        return HttpResponseHelper::internal_server_error()
                        .error("Field not found".to_string())
                        .build()
                    }
                    let field = (*field_ptr).to_field();
                    pocket_field_free(field_ptr);


                    let empty_search = "".to_string();

                    perform_timestamp_last_update!(session);
                    return HttpResponseHelper::ok()
                        .path("/field-detail")
                        .title("New field".to_string())
                        .session_id(session.session_id)
                        .fields(Ok(
                            vec![Field { 
                                group_id: field.group_id,
                                server_group_id: field.server_group_id,
                                ..Field::new()
                            }]
                        ))
                        .data(empty_search)
                        .build()

                } else {
                    return HttpResponseHelper::internal_server_error()
                    .error("Field not found".to_string())
                    .build()
                }
            } else {
                let ret = (*field_ptr).to_field();
                pocket_field_free(field_ptr);
                ret
            }
        };

        let mut title = "New field".to_string();
        if id > 0 && field.title.is_some() {
            title = field.title.clone().unwrap();
        }

        let mut fields = Fields::new();
        fields.push(field);
        let fields: crate::utils::Result<Fields> = Ok(fields);

        perform_timestamp_last_update!(session);
        HttpResponseHelper::ok()
            .path("/field-detail")
            .title(title)
            .session_id(session.session_id)
            .fields(fields)
            .build()
    }

}