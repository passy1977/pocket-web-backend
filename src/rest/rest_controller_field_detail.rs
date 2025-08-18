use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use actix_web::web::Json;
use actix_web::HttpResponse;


impl RestController {

    pub fn field_detail(&self, _data_transport: Json<DataTransport>) -> HttpResponse {



        HttpResponseHelper::ok()
            // .session_id(session.session_id)
            // .groups(get_list_group(group_controller, field_controller, group_id, &search))
            // .fields(get_list_field(field_controller, group_id, &search))
            .build()
    }

}