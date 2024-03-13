use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn carmine_entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let input_name = &input.sig.ident;

    TokenStream::from(quote!(
        fn main() -> Result<(), CarmineError<String>> {
            println!("Hello from carmine");
            #input_name();
            Ok(())
        }

        #[allow(unused)]
        #input
    ))
}
