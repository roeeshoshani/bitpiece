mod named_struct;
mod newtypes;
mod utils;

use named_struct::bitpiece_named_struct;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};
use utils::not_supported_err;

/// an attribute for defining bitfield structs.
#[proc_macro_attribute]
pub fn bitpiece(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    impl_bitpiece(args, input)
}

fn impl_bitpiece(
    args_tokens: proc_macro::TokenStream,
    input_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if !args_tokens.is_empty() {
        return quote_spanned! {
            proc_macro2::TokenStream::from(args_tokens).span() => compile_error!("no args expected");
        }
        .into();
    }
    let input = parse_macro_input!(input_tokens as DeriveInput);

    match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => bitpiece_named_struct(&input, &fields),
            syn::Fields::Unnamed(_) => not_supported_err("unnamed structs"),
            syn::Fields::Unit => not_supported_err("empty structs"),
        },
        syn::Data::Enum(_) => not_supported_err("enums"),
        syn::Data::Union(_) => not_supported_err("unions"),
    }
}
