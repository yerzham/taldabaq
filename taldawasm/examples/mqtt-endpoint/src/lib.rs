use taldawasm::http::http_endpoint::{handler, Error, Request, Response};

#[handler]
fn handler(request: Request) -> Result<Response, Error> {
    Ok(Response {
        status_code: 200,
        body: request.body,
        headers: None,
    })
}
