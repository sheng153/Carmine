use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn carmine_entry(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let input_name = &input.sig.ident;

    TokenStream::from(quote!(
        fn main() -> Result<(), carmine_engine::errors::CarmineError> {
            carmine_engine::logging::init();
            carmine_engine::logging::debug!("Hello from carmine");
            #input_name();
            Ok(())
        }
        #input
    ))
}
