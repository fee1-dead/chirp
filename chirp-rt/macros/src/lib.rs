use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn kernel(_inputs: TokenStream, _i: TokenStream) -> TokenStream {
    TokenStream::from_str(r##"
        ::core::compile_error!("You must use chirpc to compile programs using taichi");
    "##).unwrap()
}