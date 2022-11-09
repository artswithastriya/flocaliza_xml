use proc_macro::TokenStream;
use syn::parse_macro_input;
mod process;

#[proc_macro_attribute]
pub fn localizable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as syn::ItemStruct);
 
    process::input(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
 
   
}
