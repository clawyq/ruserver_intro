use axum::{
    async_trait, body::Body, extract::{FromRequest, Request}, response::{IntoResponse, Response}, Json
};
use hyper::StatusCode;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "Must provide a valid email"))]
    username: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters long"))] //custom(function="<name_of_func>")
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

pub async fn login(user: RequestUser) {
    dbg!(user);
}

pub async fn validate(Json(user): Json<RequestUser>) -> Response {
    (StatusCode::OK, format!("{} is ok", user.password)).into_response()
}
