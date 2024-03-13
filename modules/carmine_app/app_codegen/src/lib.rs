use proc_macro::TokenStream;

mod carmine_entry;

#[proc_macro_attribute]
pub fn carmine_entry(attr: TokenStream, item: TokenStream) -> TokenStream {
    carmine_entry::carmine_entry(attr, item)
}
