pub mod http {
    pub mod http_endpoint {
        #[doc(hidden)]
        pub use ::http_endpoint::wit as __wit;
        #[doc(hidden)]
        pub use ::http_endpoint::wit::exports::taldawasm::http::http_endpoint::HttpEndpoint as __HttpEndpoint;
        #[doc(hidden)]
        pub use ::http_endpoint::export_endpoint as __export_endpoint;

        pub use ::http_endpoint_macro::http_endpoint as handler;
        pub use ::http_endpoint::wit::taldawasm::http::http_endpoint_types::*;
    }
    
}
