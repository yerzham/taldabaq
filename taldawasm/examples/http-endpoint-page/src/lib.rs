use rust_embed_for_web::{EmbedableFile, RustEmbed};
use taldawasm::http::{response::Builder, Error, Method, Request, Response};

#[derive(RustEmbed)]
#[folder = "public/"]
#[gzip = true]
struct Asset;

#[taldawasm::http::endpoint::handler]
fn handler(request: Request) -> Result<Response, Error> {
    match request.method() {
        &Method::GET => {
            let index = Asset::get("index.html").map_or(
                Err(Error::UnexpectedError(
                    "index.html could not be read".to_string(),
                )),
                |x| Ok(x),
            )?;

            let gzip = index.data_gzip();
            let is_gzip = gzip.is_some();
            let contents = gzip.or(Some(index.data())).unwrap();
            let mut res = Response::new(Some(contents.into()));
            let headers = res.headers_mut();
            headers.insert("ETag", index.etag().parse().unwrap());
            headers.insert("Content-Type", "text/html".parse().unwrap());
            if is_gzip {
                headers.insert("Content-Encoding", "gzip".parse().unwrap());
            }
            Ok(res)
        }
        _ => {
            let mut res = Builder::new()
                .status(405)
                .body(Some("Method Not Allowed".into()))
                .unwrap();
            let headers = res.headers_mut();
            headers.insert("Content-Type", "text/plain".parse().unwrap());
            headers.insert("Allow", "GET".parse().unwrap());
            Ok(res)
        }
    }
}
