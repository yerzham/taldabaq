use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote;

#[proc_macro_attribute]
pub fn http_endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    use_for_endpoint_impl(func).into()
}

fn use_for_endpoint_impl(func: syn::ItemFn) -> TokenStream2 {
    // let name = func.sig.ident;
    let name = func.sig.ident.clone();

    let contents = quote::quote! {
        use taldawasm::http::http_endpoint::__HttpEndpoint;
        pub struct HttpEndpointHandler;
        #func
        impl __HttpEndpoint for HttpEndpointHandler {
            fn handle_request(req: taldawasm::http::http_endpoint::__types::Request) -> Result<taldawasm::http::http_endpoint::__types::Response, taldawasm::http::http_endpoint::__types::Error> {
                let req: taldawasm::http::Request = req.try_into()?;
                let res = #name(req)?;
                res.try_into()
            }
        }
        taldawasm::http::http_endpoint::__export_endpoint!(HttpEndpointHandler);
    };

    TokenStream2::from(contents)
}
