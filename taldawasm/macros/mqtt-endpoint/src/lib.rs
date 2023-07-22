mod parse_source;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote;
use syn::parse::Error;

#[proc_macro_attribute]
pub fn http_endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    match http_endpoint2(attr.into(), item.into()) {
        Ok(output) => output,
        Err(error) => error.into_compile_error(),
    }
    .into()
}

fn http_endpoint2(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream2> {
    const HTTP_COMPONENT_WIT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../wit");
    
    let call_site = Span::call_site();
    let mut item: syn::ItemFn = syn::parse(item).unwrap();
    item.sig.ident = syn::Ident::new("handle_request", item.sig.ident.span());
    
    let (resolve, pkg, files) = 
        parse_source::parse_source(HTTP_COMPONENT_WIT).map_err(|err| 
            Error::new(call_site, format!("{err:?}")
        ))?;
    let world = resolve
        .select_world(pkg, Some("taldawit:http/endpoint"))
        .map_err(|e| Error::new(call_site, format!("{e:?}")))?;
    
    let mut opts = wit_bindgen_rust::Opts::default();
    // opts.std_feature = true;
    // opts.raw_strings = true;
    opts.macro_export = true;
    // opts.runtime_path = Some(String::from("wit_bindgen::rt"));
    // opts.macro_call_prefix = Some(String::from("::taldawasm::"));

    let mut resolved_files = Default::default();

    opts.build().generate(&resolve, world, &mut resolved_files);
    let (_, src) = resolved_files.iter().next().unwrap();
    let src = std::str::from_utf8(src).unwrap();
    let mut contents_wit = src.parse::<TokenStream2>().unwrap();

    for file in files.iter() {
        contents_wit.extend(
            format!("const _: &str = include_str!(r#\"{}\"#);\n", file.display())
                .parse::<TokenStream2>()
                .unwrap(),
        );
    }

    let contents = quote::quote! {
        
        use exports::taldawit::http::http_endpoint::{HttpEndpoint};

        #contents_wit

        pub struct Endpoint;

        impl HttpEndpoint for Endpoint {
            #item
        }

        export_endpoint!(Endpoint);
    };
    
    Ok(TokenStream2::from(contents))
}
