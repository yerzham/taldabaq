mod http_endpoint;
mod mqtt_endpoint;

use std::{ops::Deref, str::FromStr};

#[doc(hidden)]
pub mod __wit {
    wit_bindgen::generate!({
        path: "wit",
        world: "taldawasm:main/endpoint",
        macro_call_prefix: "::taldawasm::__wit::",
        macro_export
    });
}

pub mod types {
    pub use crate::__wit::taldawasm::main::http_types::*;
}

pub mod http {
    use std::result::Result;
    use crate::__wit::taldawasm::main::http_outgoing::fetch as talda_fetch;
    pub mod endpoint {
        pub use crate::__wit::exports::taldawasm::main::http_endpoint::HttpEndpoint as Endpoint;
        pub use crate::http_endpoint::*;
    }

    pub type Request = ::http::Request<Option<::bytes::Bytes>>;
    pub type Response = ::http::Response<Option<::bytes::Bytes>>;
    pub type Error = crate::types::Error;
    pub use bytes::Bytes;

    pub use http::*;

    pub fn fetch(request: Request) -> Result<Response, Error> {
        let req: crate::types::Request = request.try_into()?;
        let res: crate::types::Response = talda_fetch(&req)?;
        res.try_into()
    }
}

pub mod mqtt {
    pub mod endpoint {
        pub use crate::__wit::exports::taldawasm::main::mqtt_endpoint::MqttEndpoint as Endpoint;
        pub use crate::mqtt_endpoint::*;
    }
}

impl TryFrom<types::Method> for http::Method {
    type Error = http::method::InvalidMethod;
    fn try_from(value: types::Method) -> Result<Self, Self::Error> {
        use crate::http::Method;
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

impl From<&http::Method> for types::Method {
    fn from(value: &http::Method) -> Self {
        use crate::http::Method;
        use types::Method as TWMethod;
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
            _ => TWMethod::Other(value.to_string()),
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

impl TryFrom<types::Request> for http::Request {
    type Error = types::Error;
    fn try_from(req: types::Request) -> Result<Self, Self::Error> {
        let headers = req.headers;
        let method: http::Method = req.method.try_into().map_err(|error| {
            types::Error::UnexpectedError(format!(
                "[taldawasm:http-endpoint-macro]: Failed to convert method. Caused by: {}",
                error
            ))
        })?;

        let mut http_res: http::Request = http::request::Builder::new()
            .uri(req.path)
            .method(method)
            .body(req.body.map(|b| b.into()))
            .map_err(|error| {
                types::Error::UnexpectedError(format!(
                    "[taldawasm:http-endpoint-macro]: Failed to build a request. Caused by: {}",
                    error
                ))
            })?;

        let http_headers = create_http_headers(headers);
        *http_res.headers_mut() = http_headers;

        Ok(http_res)
    }
}

impl TryFrom<http::Request> for types::Request {
    type Error = types::Error;
    fn try_from(req: http::Request) -> Result<Self, Self::Error> {
        let path = req.uri();
        let method = req.method();
        let headers = req.headers();
        let body = req.body().as_ref().map(|b| b.deref());

        let mut tw_headers: types::Headers = vec![];

        headers
            .iter()
            .try_for_each(|header| {
                let name = header.0.to_string();
                let value = header.1.to_str();
                match value {
                    Ok(value) => Ok(tw_headers.push((name, value.to_string()))),
                    Err(e) => Err(e),
                }
            })
            .map_err(|error| {
                types::Error::UnexpectedError(format!(
                    "[taldawasm:http-endpoint-macro]: Failed to convert headers. Caused by: {}",
                    error
                ))
            })?;

        let tw_req = types::Request {
            method: method.into(),
            path: path.to_string(),
            headers: tw_headers,
            body: body.map(|b| b.into()),
        };

        Ok(tw_req)
    }
}

impl TryFrom<types::Response> for http::Response {
    type Error = types::Error;
    fn try_from(value: types::Response) -> Result<Self, Self::Error> {
        let status = http::StatusCode::from_u16(value.status_code).map_err(|error| {
            types::Error::UnexpectedError(format!(
                "[taldawasm:http-endpoint-macro]: Failed to convert status code. Caused by: {}",
                error
            ))
        })?;
        let headers = create_http_headers(value.headers);
        let body = value.body.map(|b| b.into());

        let mut http_res = http::Response::new(body);
        *http_res.status_mut() = status;
        *http_res.headers_mut() = headers;

        Ok(http_res)
    }
}

impl TryFrom<http::Response> for types::Response {
    type Error = types::Error;
    fn try_from(value: http::Response) -> Result<Self, Self::Error> {
        let headers = value.headers();
        let status = value.status();
        let body = value.body().as_ref().map(|b| b.deref().into());

        let mut tw_headers: types::Headers = vec![];

        headers
            .iter()
            .try_for_each(|header| {
                let name = header.0.to_string();
                let value = header.1.to_str();
                match value {
                    Ok(value) => Ok(tw_headers.push((name, value.to_string()))),
                    Err(e) => Err(e),
                }
            })
            .map_err(|error| {
                types::Error::UnexpectedError(format!(
                    "[taldawasm:http-endpoint-macro]: Failed to convert headers. Caused by: {}",
                    error
                ))
            })?;

        let tw_res = types::Response {
            status_code: status.as_u16(),
            headers: tw_headers,
            body,
        };

        Ok(tw_res)
    }
}
