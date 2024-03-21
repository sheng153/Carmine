use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_str, Ident, ItemStruct, TypeReference};

#[proc_macro_attribute]
pub fn event(categories: TokenStream, block: TokenStream) -> TokenStream {
    let block = parse_macro_input!(block as ItemStruct);
    let name = &block.ident;
    let static_name: Ident = parse_str(&format!(
        "CATEGORY_FLAGS_{}",
        &name.to_string().to_ascii_uppercase()
    ))
    .unwrap();

    let categories = parse_macro_input!(categories as TypeReference);

    TokenStream::from(quote! {
        #block

        static #static_name: u8 = carmine_events::generate_mask(#categories);

        impl carmine_events::Event for #name {
            #[inline]
            fn get_event_type(&self) -> EventType {
                carmine_events::EventType::#name
            }
            #[inline]
            fn get_handled(&self) -> bool {
                self.handled
            }
            #[inline]
            fn set_handled(&mut self, handled: bool) {
                self.handled = handled;
            }
            #[inline]
            fn get_categories() -> &'static [EventCategory] {
                #categories
            }
            #[inline]
            fn get_category_flags() -> u8 {
                #static_name
            }
            #[inline]
            fn is_in_category(category: EventCategory) -> bool {
                #static_name & category.value() != 0
            }
        }
    })
}
