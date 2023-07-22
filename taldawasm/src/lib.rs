pub mod http {
    pub mod http_endpoint {
        #[doc(hidden)]
        pub use ::http_endpoint::wit as __wit;
        #[doc(hidden)]
        pub use ::http_endpoint::wit::exports::taldawasm::http::http_endpoint::HttpEndpoint as __HttpEndpoint;
        #[doc(hidden)]
        pub use ::http_endpoint::export_endpoint as __export_endpoint;
        #[doc(hidden)]
        pub use ::http_endpoint::wit::taldawasm::http::http_endpoint_types as __types;

        pub use ::http_endpoint_macro::http_endpoint as handler;
    }

    pub type Request = ::http::Request<Option<::bytes::Bytes>>;
    pub type Response = ::http::Response<Option<::bytes::Bytes>>;
    pub use self::http_endpoint::__types::Error;
    pub use bytes::Bytes;

    pub use http::*;
}
