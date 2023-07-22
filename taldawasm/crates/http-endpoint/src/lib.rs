#[doc(hidden)]
pub mod wit {
    use std::{str::FromStr, ops::Deref};

    wit_bindgen::generate!({
        path: "../../wit",
        world: "taldawasm:http/endpoint",
        macro_call_prefix: "::taldawasm::http::http_endpoint::__wit::",
        macro_export
    });

    use taldawasm::http::http_endpoint_types as types;

    impl TryFrom<types::Method> for http::Method {
        type Error = http::method::InvalidMethod;
        fn try_from(value: types::Method) -> Result<Self, Self::Error> {
            use http::Method;
            use types::Method as TWMethod;
            match value {
                TWMethod::Get => Ok(Method::GET),
                TWMethod::Post => Ok(Method::POST),
                TWMethod::Put => Ok(Method::PUT),
                TWMethod::Delete => Ok(Method::DELETE),
                TWMethod::Head => Ok(Method::HEAD),
                TWMethod::Options => Ok(Method::OPTIONS),
                TWMethod::Connect => Ok(Method::CONNECT),
                TWMethod::Patch => Ok(Method::PATCH),
                TWMethod::Trace => Ok(Method::TRACE),
                TWMethod::Other(value) => Method::from_str(value.as_str()),
            }
        }
    }

    fn create_http_headers(headers: Vec<(String, String)>) -> http::header::HeaderMap {
        let mut http_headers = http::header::HeaderMap::new();
        headers.iter().for_each(|(name, value)| {
            http_headers.insert(
                http::header::HeaderName::from_str(name).unwrap(),
                http::header::HeaderValue::from_str(value).unwrap(),
            );
        });
        http_headers
    }

    impl TryFrom<types::Request> for http::Request<Option<bytes::Bytes>> {
        type Error = types::Error;
        fn try_from(req: types::Request) -> Result<Self, Self::Error> {
            let headers = req.headers;
            let method: http::Method = req.method.try_into().map_err(|e| {
                types::Error::UnexpectedError(format!("[taldawasm:http-endpoint-macro]: Failed to convert method. Caused by: {}", e))
            })?;

            let mut http_res: http::Request<Option<bytes::Bytes>> = http::Request::builder()
                .uri(req.path)
                .method(method)
                .body(req.body.map(|b| b.into())).map_err(|e| {
                    types::Error::UnexpectedError(format!("[taldawasm:http-endpoint-macro]: Failed to build a request. Caused by: {}", e))
                })?;

            let http_headers = create_http_headers(headers);
            *http_res.headers_mut() = http_headers;

            Ok(http_res)
        }
    }

    impl TryFrom<http::Response<Option<bytes::Bytes>>> for types::Response {
        type Error = types::Error;
        fn try_from(value: http::Response<Option<bytes::Bytes>>) -> Result<Self, Self::Error> {
            let headers = value.headers();
            let status = value.status();
            let body = value.body().as_ref().map(|b| b.deref().into());

            let mut tw_headers: types::Headers = vec![];

            headers.iter().try_for_each(|header| {
                let name = header.0.to_string();
                let value = header.1.to_str();
                match value {
                    Ok(value) => Ok(tw_headers.push((name, value.to_string()))),
                    Err(e) => Err(e),
                }
            }).map_err(|e| {
                types::Error::UnexpectedError(format!("[taldawasm:http-endpoint-macro]: Failed to convert headers. Caused by: {}", e))
            })?;

            let tw_res = types::Response {
                status_code: status.as_u16(),
                headers: tw_headers,
                body,
            };

            Ok(tw_res)
        }
    }
}
