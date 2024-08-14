use axum::{
    async_trait, body::Body, extract::{FromRequest, Request}, response::{IntoResponse, Response}, Json
};
use hyper::StatusCode;
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "Must provide a valid email"))]
    username: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters long"), custom(function=password_strength_check))] 
    password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user): Json<RequestUser> = Json::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{:?}", err)))?;
        if let Err(err) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", err)))
        }
        Ok(user)
    }
}

fn password_strength_check(password: &str) -> Result<(), ValidationError> {
    if password.len() < 12 {
        return Err(ValidationError::new("length").with_message(std::borrow::Cow::Borrowed("sike not long enough")));
    }

    let special_char = Regex::new(r##"[!@#$%^&*(),.?:{}|<>]"##).unwrap();
    if !special_char.is_match(password) {
        return Err(ValidationError::new("special").with_message(std::borrow::Cow::Borrowed("Password must contain at least one special character")));
    };

    let numbers = Regex::new(r"\d").unwrap();
    if !numbers.is_match(password) {
        return Err(ValidationError::new("number").with_message(std::borrow::Cow::Borrowed("Password must contain at least one number")));
    };

    let uppercase = Regex::new(r"[A-Z]").unwrap();
    if !uppercase.is_match(password) {
        return Err(ValidationError::new("case").with_message(std::borrow::Cow::Borrowed("Password must contain at least one uppercase character")));
    };

    Ok(())
}

pub async fn login(user: RequestUser) {
    dbg!(user);
}

pub async fn validate(Json(user): Json<RequestUser>) -> Response {
    (StatusCode::OK, format!("{} is ok", user.password)).into_response()
}
