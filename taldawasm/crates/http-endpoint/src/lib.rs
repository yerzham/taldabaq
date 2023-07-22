#[doc(hidden)]
pub mod wit {
    wit_bindgen::generate!({
        path: "../../wit",
        world: "taldawasm:http/endpoint",
        macro_call_prefix: "::taldawasm::http::http_endpoint::__wit::",
        macro_export
    });
}
