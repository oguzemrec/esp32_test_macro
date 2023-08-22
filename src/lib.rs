

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input};


#[proc_macro_attribute]
pub fn esp32_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syn data structure
    let input = parse_macro_input!(input as syn::ItemFn);

    // Extract the function identifier and its block
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    // Generate the modified function body using the quote crate
    let expanded = quote! {
        fn #fn_name() {
            #fn_block
            println!("Test Passed: {}", stringify!(#fn_name));
        }
    };

    // Convert the generated tokens back into a TokenStream
    TokenStream::from(expanded)
}