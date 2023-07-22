use taldawasm::http::{http_endpoint, Response, Error, Request};

#[http_endpoint::handler]
fn handler(request: Request) -> Result<Response, Error> {
    Ok(Response::new(Some("Hello, World!".into())))
}

#[test]
fn test() {
    assert_eq!(1, 1);
}