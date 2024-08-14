use hyper::HeaderMap;

pub async fn headers(headers: HeaderMap) -> String {
    let val = headers.get("User-Agent").unwrap();
    match val.to_str() {
        Ok(dog) => dog.to_owned(),
        Err(e) => format!("what the dog doin: {e}"),
    }
}
