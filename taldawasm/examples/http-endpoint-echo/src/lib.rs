use taldawasm::http::http_endpoint::{handler, Error, Request, Response};

#[handler]
fn handler(request: Request) -> Result<Response, Error> {
    Ok(Response {
        status_code: 200,
        body: request.body,
        headers: request
            .headers
            .iter()
            .find_map(|(name, value)| {
                if name == "Content-Type" {
                    Some(vec![(name.to_string(), value.to_string())])
                } else {
                    None
                }
            })
            .or(Some(vec![(
                "Content-Type".to_string(),
                "text/plain".to_string(),
            )])),
    })
}
