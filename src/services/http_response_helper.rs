use crate::models::field::Fields;
use crate::models::group::Groups;
use crate::models::group_field::GroupFields;
use crate::models::data_transport::DataTransport;
use crate::utils::Result;
use actix_web::HttpResponse;

enum Status {
    Ok,
    NotAcceptable,
    Forbidden,
    InternalServerError
}

pub struct HttpResponseHelper(Status, DataTransport);

impl HttpResponseHelper {

    pub fn ok() -> Self {
        Self(Status::Ok, DataTransport::default())
    }

    pub fn not_acceptable() -> Self {
        Self(Status::NotAcceptable, DataTransport::default())
    }

    pub fn forbidden() -> Self {
        Self(Status::Forbidden, DataTransport::default())
    }

    pub fn internal_server_error() -> Self {
        Self(Status::InternalServerError, DataTransport::default())
    }


    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.1.path = path.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.1.title = title.into();
        self
    }

    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.1.session_id = session_id.into();
        self
    }

    pub fn groups(mut self, groups: Result<Groups>) -> Self {
        self.1.groups = match groups {
            Ok(groups) => Some(groups),
            Err(_) => None,
        };
        self
    }

    pub fn group_fields(mut self, group_fields: Result<GroupFields>) -> Self {
        self.1.group_fields = match group_fields {
            Ok(group_fields) => Some(group_fields),
            Err(_) => None,
        };
        self
    }

    pub fn fields(mut self, fields: Result<Fields>) -> Self {
        self.1.fields = match fields {
            Ok(fields) => Some(fields),
            Err(_) => None,
        };
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.1.data = Some(data.into());
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.1.error = Some(error.into());
        self
    }

    pub fn build(self) -> HttpResponse {
        match self.0 {
            Status::Ok => HttpResponse::Ok(),
            Status::NotAcceptable => HttpResponse::NotAcceptable(),
            Status::Forbidden => HttpResponse::Forbidden(),
            Status::InternalServerError => HttpResponse::InternalServerError()
        }.json(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::field::Fields;
    use crate::models::group::Groups;
    use crate::models::group_field::GroupFields;

    #[test]
    fn test_http_response_helper_ok() {
        let helper = HttpResponseHelper::ok();
        
        match helper.0 {
            Status::Ok => (),
            _ => panic!("Expected Status::Ok"),
        }
        
        assert_eq!(helper.1.path, "/login");
        assert_eq!(helper.1.title, "Login");
    }

    #[test]
    fn test_http_response_helper_not_acceptable() {
        let helper = HttpResponseHelper::not_acceptable();
        
        match helper.0 {
            Status::NotAcceptable => (),
            _ => panic!("Expected Status::NotAcceptable"),
        }
    }

    #[test]
    fn test_http_response_helper_forbidden() {
        let helper = HttpResponseHelper::forbidden();
        
        match helper.0 {
            Status::Forbidden => (),
            _ => panic!("Expected Status::Forbidden"),
        }
    }

    #[test]
    fn test_http_response_helper_internal_server_error() {
        let helper = HttpResponseHelper::internal_server_error();
        
        match helper.0 {
            Status::InternalServerError => (),
            _ => panic!("Expected Status::InternalServerError"),
        }
    }

    #[test]
    fn test_http_response_helper_path() {
        let helper = HttpResponseHelper::ok()
            .path("/test/path");
        
        assert_eq!(helper.1.path, "/test/path");
    }

    #[test]
    fn test_http_response_helper_title() {
        let helper = HttpResponseHelper::ok()
            .title("Test Title");
        
        assert_eq!(helper.1.title, "Test Title");
    }

    #[test]
    fn test_http_response_helper_session_id() {
        let helper = HttpResponseHelper::ok()
            .session_id("test_session_123");
        
        assert_eq!(helper.1.session_id, "test_session_123");
    }

    #[test]
    fn test_http_response_helper_data() {
        let helper = HttpResponseHelper::ok()
            .data("test data content");
        
        assert_eq!(helper.1.data, Some("test data content".to_string()));
    }

    #[test]
    fn test_http_response_helper_error() {
        let helper = HttpResponseHelper::ok()
            .error("test error message");
        
        assert_eq!(helper.1.error, Some("test error message".to_string()));
    }

    #[test]
    fn test_http_response_helper_groups_success() {
        let groups: Groups = vec![];
        let helper = HttpResponseHelper::ok()
            .groups(Ok(groups));
        
        assert!(helper.1.groups.is_some());
    }

    #[test]
    fn test_http_response_helper_groups_error() {
        let helper = HttpResponseHelper::ok()
            .groups(Err("test error"));
        
        assert!(helper.1.groups.is_none());
    }

    #[test]
    fn test_http_response_helper_group_fields_success() {
        let group_fields: GroupFields = vec![];
        let helper = HttpResponseHelper::ok()
            .group_fields(Ok(group_fields));
        
        assert!(helper.1.group_fields.is_some());
    }

    #[test]
    fn test_http_response_helper_group_fields_error() {
        let helper = HttpResponseHelper::ok()
            .group_fields(Err("test error"));
        
        assert!(helper.1.group_fields.is_none());
    }

    #[test]
    fn test_http_response_helper_fields_success() {
        let fields: Fields = vec![];
        let helper = HttpResponseHelper::ok()
            .fields(Ok(fields));
        
        assert!(helper.1.fields.is_some());
    }

    #[test]
    fn test_http_response_helper_fields_error() {
        let helper = HttpResponseHelper::ok()
            .fields(Err("test error"));
        
        assert!(helper.1.fields.is_none());
    }

    #[test]
    fn test_http_response_helper_chaining() {
        let helper = HttpResponseHelper::ok()
            .path("/api/test")
            .title("API Test")
            .session_id("session_abc")
            .data("test payload")
            .error("test error");
        
        assert_eq!(helper.1.path, "/api/test");
        assert_eq!(helper.1.title, "API Test");
        assert_eq!(helper.1.session_id, "session_abc");
        assert_eq!(helper.1.data, Some("test payload".to_string()));
        assert_eq!(helper.1.error, Some("test error".to_string()));
    }

    #[test]
    fn test_http_response_helper_build() {
        let response = HttpResponseHelper::ok()
            .path("/test")
            .title("Test")
            .build();
        
        // Verifica che la response sia stata creata
        // Non possiamo facilmente testare il contenuto della response HTTP
        // ma possiamo verificare che non vada in panic
        assert_eq!(response.status(), 200);
    }
}