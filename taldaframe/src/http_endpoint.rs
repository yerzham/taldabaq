pub mod __wit {
    use std::str::FromStr;

    use axum::{
        async_trait,
        extract::{rejection::BytesRejection, FromRequest},
        response::{IntoResponse},
        BoxError,
    };
    use http::header::ToStrError;
    use http_body::Body;
    use wasmtime::component::*;

    bindgen!({
        path: "../taldawasm/wit",
        world: "taldawasm:main/http-endpoint-component",
        with: {
            "http-endpoint": crate::interfaces::taldawasm::main::http_endpoint,
            "http-types": crate::interfaces::taldawasm::main::http_types,
        },
    });

    use self::taldawasm::main::http_types as taldawasm_http_endpoint;

    impl From<&http::Method> for taldawasm_http_endpoint::Method {
        fn from(value: &http::Method) -> Self {
            use http::Method;
            use taldawasm_http_endpoint::Method as TWMethod;
            match value {
                &Method::GET => TWMethod::Get,
                &Method::POST => TWMethod::Post,
                &Method::PUT => TWMethod::Put,
                &Method::DELETE => TWMethod::Delete,
                &Method::HEAD => TWMethod::Head,
                &Method::OPTIONS => TWMethod::Options,
                &Method::CONNECT => TWMethod::Connect,
                &Method::PATCH => TWMethod::Patch,
                &Method::TRACE => TWMethod::Trace,
                _ => taldawasm_http_endpoint::Method::Other(value.to_string()),
            }
        }
    }

    impl TryFrom<taldawasm_http_endpoint::Method> for http::Method {
        type Error = http::method::InvalidMethod;
        fn try_from(value: taldawasm_http_endpoint::Method) -> Result<http::Method, http::method::InvalidMethod> {
            use http::Method;
            use taldawasm_http_endpoint::Method as TWMethod;
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

    fn create_taldawasm_http_endpoint_request(
        path: Option<&http::uri::PathAndQuery>,
        method: &http::Method,
        headers: &http::HeaderMap,
        body: Option<bytes::Bytes>,
    ) -> Result<taldawasm_http_endpoint::Request, ToStrError> {
        let tw_body = body.map(|b| b.into());
        let mut tw_headers: taldawasm_http_endpoint::Headers = vec![];
        headers.iter().try_for_each(|header| {
            let name = header.0.to_string();
            let value = header.1.to_str();
            value.map(|value| tw_headers.push((name, value.to_string())))
        })?;
        let tw_path = path.map_or("".to_string(), |path| path.to_string());

        Ok(taldawasm_http_endpoint::Request {
            path: tw_path,
            method: method.into(),
            headers: tw_headers,
            body: tw_body,
        })
    }

    #[async_trait]
    impl<S, B> FromRequest<S, B> for taldawasm_http_endpoint::Request
    where
        B: Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<BoxError>,
        S: Send + Sync,
    {
        type Rejection = (http::StatusCode, String);

        async fn from_request(req: http::Request<B>, state: &S) -> Result<Self, Self::Rejection> {
            let headers = req.headers();
            let path = req.uri().path_and_query();
            let method = req.method();
            let mut res = create_taldawasm_http_endpoint_request(path, method, headers, None)
                .map_err(|_| {
                    (
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read headers".to_string(),
                    )
                })?;
            let bytes = bytes::Bytes::from_request(req, state)
                .await
                .map_err(|err| match err {
                    BytesRejection::FailedToBufferBody(inner) => {
                        (http::StatusCode::INTERNAL_SERVER_ERROR, inner.body_text())
                    }
                    _ => (
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Unexpected error while reading body buffer.".to_string(),
                    ),
                })?;
            res.body = Some(bytes.into());
            Ok(res)
        }
    }

    fn create_http_status_code(status_code: u16) -> http::StatusCode {
        status_code
            .try_into()
            .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn create_http_headers(headers: taldawasm_http_endpoint::Headers) -> http::HeaderMap {
        let mut http_headers = http::HeaderMap::new();
        headers.iter().for_each(|header| {
            let name = header.0.to_string().to_lowercase();
            let value = header.1.to_string();
            http_headers.insert(
                http::header::HeaderName::from_lowercase(name.as_bytes()).unwrap(),
                value.parse().unwrap(),
            );
        });
        http_headers
    }

    fn create_taldawasm_http_endpoint_headers(headers: http::HeaderMap) -> taldawasm_http_endpoint::Headers {
        let mut tw_headers: taldawasm_http_endpoint::Headers = vec![];
        headers.iter().for_each(|header| {
            let name = header.0.to_string();
            let value = header.1.to_str().unwrap();
            tw_headers.push((name, value.to_string()))
        });
        tw_headers
    }

    impl IntoResponse for taldawasm_http_endpoint::Response {
        fn into_response(self) -> axum::response::Response {
            // its often easiest to implement `IntoResponse` by calling other implementations
            // let body = self.body.map_or(Ok("".to_string()), |v| {
            //     String::from_utf8(v).map_err(|e| {
            //         println!("wasm_app_execute: {:?}", e);
            //         StatusCode::INTERNAL_SERVER_ERROR
            //     })
            // })?;
            (
                create_http_status_code(self.status_code),
                create_http_headers(self.headers),
                self.body.or(Some(vec![])).unwrap(),
            )
                .into_response()
        }
    }

    use taldawasm_http_endpoint::*;

    impl taldawasm::main::http_outgoing::Host for crate::http_endpoint::HttpEndpointHost {
        fn fetch(&mut self, req: Request) -> wasmtime::Result<Result<Response, Error>> {
            Ok(Ok(Response {
                status_code: 200,
                headers: vec![
                    ("Content-Type".to_string(), "text/plain".to_string()),
                    ("X-Test".to_string(), "test".to_string()),
                ],
                body: Some("Outbound request attempt!".to_string().into()),
            }))
            // TODO: Support outbound requests from WASM on an async runtime
            // let method = TryInto::<http::Method>::try_into(req.method).map_err(|e| {
            //     Error::UnexpectedError(format!(
            //         "[taldawasm:http-endpoint-host]: Failed to convert method. Caused by: {}",
            //         e
            //     ))
            // });
            // let path = req.path;
            // let headers = create_http_headers(req.headers);
            // let body = req.body.map_or(vec![], |b| b.into());

            // match method {
            //     Ok(method) => {
            //         let client = reqwest::blocking::Client::new();
            //         let req = client.request(method, path).headers(headers).body(body);
            //         let res = req.send().map_err(|e| {
            //             Error::UnexpectedError(format!(
            //                 "[taldawasm:http-endpoint-host]: Failed to send a request. Caused by: {}",
            //                 e
            //             ))
            //         })?;
            //         let status_code = res.status().as_u16();
            //         let headers = res.headers().clone();
            //         let body = res.bytes().map_err(|e| {
            //             Error::UnexpectedError(format!(
            //                 "[taldawasm:http-endpoint-host]: Failed to read a response body. Caused by: {}",
            //                 e
            //             ))
            //         })?;
            //         Ok(Ok(Response {
            //             status_code,
            //             headers: create_taldawasm_http_endpoint_headers(headers),
            //             body: Some(body.into()),
            //         }))
            //     }
            //     Err(e) => Ok(Err(e)),
            // }
        }
    }
    impl taldawasm::main::http_types::Host for crate::http_endpoint::HttpEndpointHost {}
}

pub struct HttpEndpointHost;

pub mod interfaces {
    wasmtime::component::bindgen!({
        path: "../taldawasm/wit",
        interfaces: "
            import taldawasm:main/http-outgoing
        ",
    });
}

pub use __wit::taldawasm::main::http_types::*;
pub use __wit::HttpEndpointComponent;
