use taldawasm::http::http_endpoint::{Response, Error, Request, handler};

#[handler]
fn handler(request: Request) -> Result<Response, Error> {
    Ok(Response {
        status_code: 200,
        body: request.body,
        headers: None
    })
}

#[test]
fn test() {
    assert_eq!(1, 1);
}