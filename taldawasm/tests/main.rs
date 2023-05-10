
use http_wrapper::http_endpoint;
use structs::{Request, Response};

#[http_endpoint]
fn handler(_req: Request) -> Result<Response, ()> {
    Ok(Response {
        status: 200,
        body: "Hello, world!".to_string(),
    })
}

#[test]
fn test() {
    assert_eq!(1, 1);
}