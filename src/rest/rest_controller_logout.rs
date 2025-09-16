use crate::bindings::{pocket_logout, pocket_stat_t_OK};
use crate::get_session;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::RestController;
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use actix_web::web::Json;
use actix_web::HttpResponse;


impl RestController {

    pub fn logout(&self, mut data_transport: Json<DataTransport>) -> HttpResponse {
        let mut session = get_session!(data_transport.session_id, "Session not found");

        if let Some(data) = &data_transport.data {

            let maintain_config = if data == "true" {
                true
            } else if data == "false" {
                false
            } else {
                data_transport.error = Some("Data has wrong value".to_string());
                return self.home(data_transport)
            };

            unsafe {
                if pocket_logout(session.pocket, maintain_config) == pocket_stat_t_OK {

                    // let user = (*((*session.pocket).user as *const pocket_user_t)).to_user();

                    // if !maintain_config {
                    //     if let Err(error) = self.data.remove_config_json(&user.email) {
                    //         session.update_timestamp_last_update();
                    //         data_transport.error = Some(error.to_string());
                    //         return self.home(data_transport)
                    //     }
                    // }
                    
                    Sessions::share().remove(&session.session_id, true);

                    return HttpResponseHelper::ok()
                    .path("/login")
                    .data("logout")
                    .session_id(session.session_id).build();

                } else {
                    session.update_timestamp_last_update();
                    data_transport.error = Some("Unable perform logout".to_string());
                    return self.home(data_transport)
                }
            }

        } else {
            data_transport.error = Some("Data is mandatory".to_string());
            return self.home(data_transport) 
        }
    }
    
}