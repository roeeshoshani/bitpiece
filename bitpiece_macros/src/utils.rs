use quote::{quote, quote_spanned};
use syn::Generics;

use crate::newtypes::{BitLenExpr, TypeExpr};

pub fn not_supported_err_span(what: &str, span: proc_macro2::Span) -> proc_macro::TokenStream {
    quote_spanned! {
        span => compile_error!(concat!(#what, " are not supported"));
    }
    .into()
}

pub fn not_supported_err(what: &str) -> proc_macro::TokenStream {
    not_supported_err_span(what, proc_macro2::Span::call_site())
}

pub fn are_generics_empty(generics: &Generics) -> bool {
    generics.lt_token.is_none()
        && generics.params.is_empty()
        && generics.gt_token.is_none()
        && generics.where_clause.is_none()
}

pub fn gen_explicit_bit_length_assertion(
    explicit_bit_length: Option<usize>,
    actual_bit_length: &BitLenExpr,
) -> proc_macro2::TokenStream {
    match explicit_bit_length {
        Some(explicit_bit_length) => quote! {
            const _: () = if (#explicit_bit_length) != (#actual_bit_length) {
                panic!("explicit bit length does not match actual bit length")
            } else {
                ()
            };
        },
        None => quote! {},
    }
}

/// parameters for generating an implementation of the `BitPiece` trait.
pub struct BitPieceGenImplParams {
    /// the identifier of the type for which the trait is to be implemented.
    pub type_ident: syn::Ident,

    /// the mutable bit access type.
    pub mut_type: proc_macro2::TokenStream,

    /// the bit length of the type.
    pub bit_len: BitLenExpr,

    /// the bits storage type of this type.
    pub storage_type: TypeExpr,

    /// code for serializing this type.
    /// this will be used as the body of the `to_bits` method.
    pub serialization_code: proc_macro2::TokenStream,

    /// code for deserializing this type.
    /// this will be used as the body of the `from_bits` method.
    pub deserialization_code: proc_macro2::TokenStream,
}

/// generates the final implementation of the `BitPiece` trait given the implementation details.
pub fn bitpiece_gen_impl(params: BitPieceGenImplParams) -> proc_macro2::TokenStream {
    let BitPieceGenImplParams {
        type_ident,
        mut_type,
        bit_len,
        storage_type,
        serialization_code,
        deserialization_code,
    } = params;
    quote! {
        #[automatically_derived]
        impl ::bitpiece::BitPiece for #type_ident {
            const BITS: usize = (#bit_len);
            type Bits = (#storage_type);
            type Mut<'s, S: BitStorage + 's> = #mut_type;
            fn from_bits(bits: Self::Bits) -> Self {
                #deserialization_code
            }
            fn to_bits(self) -> Self::Bits {
                #serialization_code
            }
        }
    }
}
