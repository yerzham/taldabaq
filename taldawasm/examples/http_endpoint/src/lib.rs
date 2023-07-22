// use exports::taldawit::http::http_endpoint::{HttpEndpoint, Request, Response, Error};

// wit_bindgen::generate!({
//     path: "../../wit",
//     world: "taldawit:http/endpoint"
// });

// pub struct Endpoint;

// impl HttpEndpoint for Endpoint {
//     fn handle_request(request: Request) -> Result<Response, Error> {
//         Ok(Response {
//             status_code: 200,
//             body: request.body,
//             headers: None
//         })
//     }
// }

// export_endpoint!(Endpoint);

// Objective: less is more

use taldawasm::http::http_endpoint::handler;
use taldawit::http::http_endpoint_types::{Request, Response, Error}; // FIXME: ghost module

#[handler]
fn handler(request: Request) -> Result<Response, Error> {
    Ok(Response {
        status_code: 200,
        body: request.body,
        headers: None
    })
}