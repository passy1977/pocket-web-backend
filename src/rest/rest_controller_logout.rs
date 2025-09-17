use crate::bindings::{pocket_logout, pocket_stat_t_OK};
use crate::get_session;
use crate::models::data_transport::DataTransport;
use crate::rest::rest_controller::{split_group_id_and_search, RestController};
use crate::services::http_response_helper::HttpResponseHelper;
use crate::services::session::Sessions;
use actix_web::web::Json;
use actix_web::HttpResponse;


impl RestController {

    pub fn logout(&self, mut data_transport: Json<DataTransport>) -> HttpResponse {
        let  session = get_session!(data_transport.session_id, "Session not found");


        let mut maintain_config = "".to_string();
        let (_, _) = match split_group_id_and_search(&data_transport, &mut maintain_config) {
            Ok((id_group, search)) => (id_group, search),
            Err(e) => return HttpResponseHelper::internal_server_error()
                .error(e)
                .build()
        };


        let maintain_config = if maintain_config == "true" {
            true
        } else if maintain_config == "false" {
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
                data_transport.error = Some("Unable perform logout".to_string());
                return self.home(data_transport)
            }
        }


    }
    
}