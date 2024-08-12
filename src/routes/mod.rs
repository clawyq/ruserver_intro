mod mirror_json;
mod hello_world;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod headers;
mod mid_mes;

use axum::{middleware, routing::{get, post}, Extension, Router};
use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

use hello_world::hello_world;
use mirror_json::mirror_json;
use path_variables::{path_variables, hardcoded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use headers::headers;
use mid_mes::{mid_mes, mid_zhng, zhng_header};

#[derive(Clone)]
pub struct SharedData {
    pub shared_msg: String
}

pub fn create_routes() -> Router<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData { shared_msg: "shared_data".to_owned() };
    Router::new()
        .route("/json", post(mirror_json))
        .route("/", get(hello_world))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hardcoded_path))
        .route("/query_params", get(query_params))
        .route("/user_agent", get(mirror_user_agent))
        .route("/headers", get(headers))
        .route("/mid_mes", get(mid_mes))
        .route("/read_mid", get(mid_zhng))
        .layer(cors)
        .layer(middleware::from_fn(zhng_header))
        .layer(Extension(shared_data))
}
