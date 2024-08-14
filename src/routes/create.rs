use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub async fn create() -> Response {
    (StatusCode::CREATED, "created bro rust me").into_response()
}
