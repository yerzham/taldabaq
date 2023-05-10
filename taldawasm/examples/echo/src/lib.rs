use http_wrapper::http_endpoint;
use structs::{Request, Response};

#[http_endpoint]
pub fn handler(_req: Request) -> Result<Response, ()> {
    Ok(Response {
        status: 200,
        body: "Hello, world!".to_string(),
    })
}