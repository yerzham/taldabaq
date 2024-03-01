use std::ops::Deref;

use taldawasm::http::{response::Builder, Error, Request, Response};

#[taldawasm::http::endpoint::handler]
fn echo(request: Request) -> Result<Response, Error> {
    let body = request.body().as_ref()
        .map_or(Ok(""), |x| std::str::from_utf8(x.deref()));
    let path = request.uri().path_and_query()
        .map(|x| x.as_str())
        .unwrap_or("");
    let mut res = match body {
        Ok(x) => Response::new(Some(format!("Path: {}\nBody:\n{}", path, x).into())),
        Err(_) => Builder::new()
            .status(400)
            .body(Some("Failed to parse the request body".into()))
            .unwrap(),
    };
    res.headers_mut()
        .insert("Content-Type", "text/plain".parse().unwrap());
    Ok(res)
}
