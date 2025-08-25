use crate::bindings::pocket_field_controller_new;
use crate::bindings::{pocket_field_controller_get, pocket_field_controller_init};
use crate::models::data_transport::DataTransport;
use crate::models::field::{Field, Fields};
use crate::rest::rest_controller::{split_group_id_and_search, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use crate::{get_field_controller, get_session};
use actix_web::web::Json;
use actix_web::HttpResponse;

impl RestController {

    pub fn field_detail(&self, data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        let mut id = "".to_string();
        let (_, _) = match split_group_id_and_search(&data_transport, &mut id) {
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
                Field::new()
            } else {
                (*field_ptr).to_field()
            }
        };

        let mut title = "New field".to_string();
        if id > 0 && field.title.is_some() {
            title = field.title.clone().unwrap();
        }

        let mut fields = Fields::new();
        fields.push(field);
        let fields: crate::utils::Result<Fields> = Ok(fields);


        HttpResponseHelper::ok()
            .path("/field-detail")
            .title(title)
            .session_id(session.session_id)
            .fields(fields)
            .build()
    }

}