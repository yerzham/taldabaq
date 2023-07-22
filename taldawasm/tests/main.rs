
use taldawasm::http::http_endpoint;

#[http_endpoint::handler]
fn handler(_request: Request) -> Result<Response, Error> {
    Ok(Response {
        status_code: 200,
        body: None,
        headers: None
    })
}

#[test]
fn test() {
    assert_eq!(1, 1);
}