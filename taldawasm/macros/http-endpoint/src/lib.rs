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
        pub struct HttpEndpoint;
        #func
        impl taldawasm::http::endpoint::Endpoint for HttpEndpoint {
            fn handle_request(req: taldawasm::types::Request) -> Result<taldawasm::types::Response, taldawasm::types::Error> {
                let req: taldawasm::http::Request = req.try_into()?;
                let res = #name(req)?;
                res.try_into()
            }
        }
        impl taldawasm::mqtt::endpoint::Endpoint for HttpEndpoint {
            fn handle_message(message: String) -> Result<String, String> {
                unimplemented!("This component does not implement an mqtt endpoint interface")
            }
        }
        taldawasm::export_endpoint!(HttpEndpoint);
    };

    TokenStream2::from(contents)
}
