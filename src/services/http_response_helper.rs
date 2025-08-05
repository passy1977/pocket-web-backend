use crate::models::field::Fields;
use crate::models::group::Groups;
use crate::models::group_field::GroupFields;
use crate::models::rests::DataTransport;
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