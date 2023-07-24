use taldawasm::http::{Response, Error, Request};

#[taldawasm::http::endpoint::handler]
fn handler(_request: Request) -> Result<Response, Error> {
    Ok(Response::new(Some("Hello, World!".into())))
}

#[test]
fn test() {
    assert_eq!(1, 1);
}