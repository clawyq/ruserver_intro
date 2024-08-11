mod mirror_json;
mod hello_world;

use axum::{routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_json::mirror_json;

pub fn create_routes() -> Router<()> {
    Router::new()
        .route("/", get(hello_world))
        .route("/json", post(mirror_json))
}
