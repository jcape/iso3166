//! Procedural Macros for generating ISO 3166 enumerations and structures

mod lukes;

use proc_macro::TokenStream;

/// Generate the relevant types from the provided source data.
#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    lukes::generate(input.into()).into()
}
