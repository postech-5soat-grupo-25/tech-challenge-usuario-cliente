use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
    OpenApiError,
};

use crate::base::domain_error::DomainError;

use std::sync::Arc;
use crate::traits::authentication_adapter::AuthenticationAdapter;

pub struct AuthenticatedUser {
    user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = DomainError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => {
                let token = token.replace("Bearer ", "");
                let auth_adapter = req.rocket().state::<Arc<dyn AuthenticationAdapter + Sync + Send>>().unwrap();
                match auth_adapter.validate_token(token.to_string(), None).await {
                    Ok(user_id) => Outcome::Success(AuthenticatedUser { user_id }),
                    Err(_) => {
                        return Outcome::Error((Status::Unauthorized, DomainError::Unauthorized))
                    }
                }
            }
            None => Outcome::Error((Status::BadRequest, DomainError::Invalid("Missing Token".to_string()))),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for AuthenticatedUser {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        let security_scheme = SecurityScheme {
            description: Some("Authorization: 'Bearer: `token`'".to_owned()),

            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
