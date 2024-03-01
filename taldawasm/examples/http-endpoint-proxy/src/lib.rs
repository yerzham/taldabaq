use taldawasm::http::{request::Builder as RequestBuilder, Error, Request, Response, fetch};

#[taldawasm::http::endpoint::handler]
fn proxy_like(_request: Request) -> Result<Response, Error> {
    return fetch(RequestBuilder::new()
        .uri("https://httpbin.org/get")
        .method("GET")
        .body(None)
        .unwrap()
    )
}
