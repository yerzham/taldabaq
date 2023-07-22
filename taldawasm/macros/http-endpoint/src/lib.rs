use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote;

#[proc_macro_attribute]
pub fn http_endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    use_for_endpoint_impl(func).into()
}

fn use_for_endpoint_impl(mut func: syn::ItemFn) -> TokenStream2 {
    func.sig.ident = syn::Ident::new("handle_request", func.sig.ident.span());

    let contents = quote::quote! {
        use taldawasm::http::http_endpoint::__HttpEndpoint;
        pub struct Endpoint;
        impl __HttpEndpoint for Endpoint {
            #func
        }
        taldawasm::http::http_endpoint::__export_endpoint!(Endpoint);
    };

    TokenStream2::from(contents)
}
