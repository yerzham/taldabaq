use taldawasm::http::http_endpoint::{handler, Error, Method, Request, Response};

#[handler]
fn handler(request: Request) -> Result<Response, Error> {
    match request.method {
        Method::Get => Ok(Response {
            status_code: 200,
            body: Some("Hello World".into()),
            headers: Some(vec![("Content-Type".to_string(), "text/plain".to_string())]),
        }),
        _ => Ok(Response {
            status_code: 405,
            body: Some("".into()),
            headers: Some(vec![
                ("Content-Type".to_string(), "text/plain".to_string()),
                ("Allow".to_string(), "GET".to_string()),
            ]),
        }),
    }
}
