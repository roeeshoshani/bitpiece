mod enums;
mod named_structs;
mod newtypes;
mod utils;

use enums::bitpiece_enum;
use named_structs::bitpiece_named_struct;
use syn::{parse_macro_input, DeriveInput, LitInt};
use utils::{are_generics_empty, not_supported_err};

/// an attribute for defining bitfields.
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
    let maybe_explicit_bit_length = parse_macro_input!(args_tokens as Option<LitInt>);
    let explicit_bit_length: Option<usize> = match maybe_explicit_bit_length {
        Some(bit_length) => match bit_length.base10_parse() {
            Ok(parsed_bit_length) => Some(parsed_bit_length),
            Err(err) => {
                return err.into_compile_error().into();
            }
        },
        None => None,
    };
    let input = parse_macro_input!(input_tokens as DeriveInput);

    if !are_generics_empty(&input.generics) {
        return not_supported_err("generics");
    }

    match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => {
                bitpiece_named_struct(&input, fields, explicit_bit_length)
            }
            syn::Fields::Unnamed(_) => not_supported_err("unnamed structs"),
            syn::Fields::Unit => not_supported_err("empty structs"),
        },
        syn::Data::Enum(data_enum) => bitpiece_enum(&input, data_enum, explicit_bit_length),
        syn::Data::Union(_) => not_supported_err("unions"),
    }
}
