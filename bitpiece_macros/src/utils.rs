use quote::{quote, quote_spanned};
use syn::Generics;

use crate::newtypes::{BitLenExpr, StorageTypeExpr, TypeExpr};

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
    pub mut_type_ident: syn::Ident,

    /// the bit length of the type.
    pub bit_len: BitLenExpr,

    /// the bits storage type of this type.
    pub storage_type: StorageTypeExpr,

    /// the fields type of this type.
    /// this is the type which represents the expanded view of this bitpiece.
    pub fields_type: TypeExpr,

    /// code for converting this type to its field values.
    /// this will be used as the body of the `to_fields` method.
    pub to_fields_code: proc_macro2::TokenStream,

    /// code for constructing this type from its field values.
    /// this will be used as the body of the `from_fields` method.
    pub from_fields_code: proc_macro2::TokenStream,

    /// code converting this type to its raw bits.
    /// this will be used as the body of the `to_bits` method.
    pub to_bits_code: proc_macro2::TokenStream,

    /// code for constructing this type from its raw bits.
    /// this will be used as the body of the `try_from_bits` method.
    pub try_from_bits_code: proc_macro2::TokenStream,

    /// an instantiation of this type with all bits sets to zero (if possible).
    pub zeroes: proc_macro2::TokenStream,

    /// an instantiation of this type with all bits sets to one (if possible).
    pub ones: proc_macro2::TokenStream,

    /// an instantiation of this type with the min possible value.
    pub min: proc_macro2::TokenStream,

    /// an instantiation of this type with the max possible value.
    pub max: proc_macro2::TokenStream,
}

/// generates the final implementation of the `BitPiece` trait given the implementation details.
pub fn bitpiece_gen_impl(params: BitPieceGenImplParams) -> proc_macro2::TokenStream {
    let BitPieceGenImplParams {
        type_ident,
        mut_type_ident,
        bit_len,
        fields_type,
        storage_type,
        to_fields_code,
        from_fields_code,
        to_bits_code,
        try_from_bits_code,
        zeroes,
        ones,
        min,
        max,
    } = params;
    quote! {
        #[automatically_derived]
        impl ::bitpiece::BitPiece for #type_ident {
            const BITS: usize = (#bit_len);
            const ZEROES: Self = #zeroes;
            const ONES: Self = #ones;
            const MIN: Self = #min;
            const MAX: Self = #max;
            type Bits = #storage_type;
            type Converter = Self;
            fn try_from_bits(bits: Self::Bits) -> Option<Self> {
                Self::try_from_bits(bits)
            }
            fn from_bits(bits: Self::Bits) -> Self {
                Self::from_bits(bits)
            }
            fn to_bits(self) -> Self::Bits {
                self.to_bits()
            }
        }

        #[automatically_derived]
        impl ::bitpiece::BitPieceHasMutRef for #type_ident {
            type MutRef<'s> = #mut_type_ident<'s>;
        }

        #[automatically_derived]
        impl ::bitpiece::BitPieceHasFields for #type_ident {
            type Fields = #fields_type;
            fn from_fields(fields: Self::Fields) -> Self {
                Self::from_fields(fields)
            }
            fn to_fields(self) -> Self::Fields {
                self.to_fields()
            }
        }

        impl #type_ident {
            pub const fn from_fields(fields: #fields_type) -> Self {
                #from_fields_code
            }
            pub const fn to_fields(self) -> #fields_type {
                #to_fields_code
            }
            pub const fn try_from_bits(bits: #storage_type) -> Option<Self> {
                #try_from_bits_code
            }
            pub const fn from_bits(bits: #storage_type) -> Self {
                Self::try_from_bits(bits).unwrap()
            }
            pub const fn to_bits(self) -> #storage_type {
                #to_bits_code
            }
            pub const fn const_eq(a: Self, b: Self) -> bool {
                a.to_bits() == b.to_bits()
            }
        }
    }
}
