use proc_macro::TokenStream;
use proc_macro2;
use quote;
use syn;

#[proc_macro_attribute]
pub fn http_endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: syn::ItemFn = syn::parse(item).unwrap();
    let name = &item.sig.ident;
    let expanded = quote::quote! {
        #item
        #[link(wasm_import_module = "http")]
        extern "C" {
            fn body() -> *const u8;
            fn body_len() -> usize;
            fn method() -> *const u8;
            fn method_len() -> usize;
            fn path() -> *const u8;
            fn path_len() -> usize;
        }
        #[no_mangle]
        pub fn load(ptr: *const u32) -> u32 {
            unsafe { *ptr }
        }
        #[no_mangle]
        pub fn store(ptr: *mut u32, value: u32) {
            unsafe { *ptr = value }
        }
        #[no_mangle]
        #[cfg(target_family = "wasm")]
        pub fn size() -> usize {
            use core::arch::wasm32::memory_size;
            unsafe { memory_size(0) }
        }
        #[no_mangle]
        pub fn run() -> u16 {
            let request = Request {
                method: unsafe {
                    let method = std::slice::from_raw_parts(method(), method_len());
                    String::from_utf8_lossy(method).to_string()
                },
                path: unsafe {
                    let path = std::slice::from_raw_parts(path(), path_len());
                    String::from_utf8_lossy(path).to_string()
                },
                body: unsafe {
                    let body = std::slice::from_raw_parts(body(), body_len());
                    String::from_utf8_lossy(body).to_string()
                },
            };
            let response = #name(request);
            let response = match response {
                Ok(response) => response,
                Err(_) => Response {
                    status: 500,
                    body: "".to_string(),
                },
            };
            let body = response.body.as_bytes();
            let body_len = body.len();
            let body = body.as_ptr();
            unsafe {
                let body = std::slice::from_raw_parts(body, body_len);
                let body = std::str::from_utf8_unchecked(body);
                println!("{}", body);
            }
            response.status
        }
    };
    TokenStream::from(expanded)
}
