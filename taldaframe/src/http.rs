use wasmtime::component::*;
pub use exports::taldawasm::http::http_endpoint::HttpEndpoint;
pub use taldawasm::http::http_endpoint_types::{Method, Error};

bindgen!({
  path: "../taldawasm/wit",
  world: "taldawasm:http/endpoint"
});

impl From<hyper::Method> for Method {
  fn from(value: hyper::Method) -> Self {
      match value {
          hyper::Method::GET => Method::Get,
          hyper::Method::POST => Method::Post,
          hyper::Method::PUT => Method::Put,
          hyper::Method::DELETE => Method::Delete,
          hyper::Method::HEAD => Method::Head,
          hyper::Method::OPTIONS => Method::Options,
          hyper::Method::CONNECT => Method::Connect,
          hyper::Method::PATCH => Method::Patch,
          hyper::Method::TRACE => Method::Trace,
          _ => Method::Other(value.to_string())
      }
  }
}
