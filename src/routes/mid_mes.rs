use super::SharedData;
use axum::{extract::Request, middleware::Next, response::Response, Extension};
use hyper::StatusCode;

pub async fn mid_mes(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.shared_msg
}

pub async fn mid_zhng(Extension(zhngd): Extension<String>) -> String {
    zhngd
}

pub async fn zhng_header(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let msg = headers
        .get("tc-message")
        .ok_or_else(|| {
            println!("cannot find la sia");
            StatusCode::NOT_FOUND
        })?
        .to_str()
        .unwrap()
        .to_owned();
    request.extensions_mut().insert(msg + " from middleware");
    Ok(next.run(request).await)
}
