use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    server_msg: String,
}

pub async fn mirror_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        server_msg: "whats up".to_owned(),
    })
}
