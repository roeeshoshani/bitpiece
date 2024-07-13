use quote::{format_ident, quote};
use syn::{DataEnum, DeriveInput, Fields};

use crate::{
    newtypes::{BitLenExpr, TypeExpr},
    utils::{
        bitpiece_gen_impl, gen_explicit_bit_length_assertion, not_supported_err,
        BitPieceGenImplParams,
    },
};

fn enum_variant_values<'a>(
    enum_ident: &'a syn::Ident,
    data_enum: &'a DataEnum,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let enum_ident = enum_ident.clone();
        quote! {
            #enum_ident::#ident
        }
    })
}

fn enum_variant_u64_values<'a>(
    enum_ident: &'a syn::Ident,
    data_enum: &'a DataEnum,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    enum_variant_values(enum_ident, data_enum).map(|value| {
        quote! {
            (#value as u64)
        }
    })
}

fn enum_bit_len(enum_ident: &syn::Ident, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let u64_values = enum_variant_u64_values(enum_ident, data_enum);
    let bits_required_for_each_value =
        u64_values.map(|u64_value| quote! { (64 - (#u64_value).leading_zeros()) });
    let max_bits_required = bits_required_for_each_value.reduce(|accumulator, cur_item| {
        quote! {
            max(#accumulator, #cur_item)
        }
    });
    quote! {
        {
            const fn max(a: u32, b: u32) -> u32 {
                if a >= b {
                    a
                } else {
                    b
                }
            }
            (#max_bits_required) as usize
        }
    }
}

fn gen_deserialization_code(
    enum_ident: &syn::Ident,
    data_enum: &DataEnum,
    bit_len: &BitLenExpr,
    storage_type: &TypeExpr,
) -> proc_macro2::TokenStream {
    let const_idents = (0..data_enum.variants.len()).map(|i| format_ident!("V{}", i));
    let consts =
        data_enum
            .variants
            .iter()
            .zip(const_idents.clone())
            .map(|(variant, const_ident)| {
                let ident = &variant.ident;
                quote! {
                    const #const_ident: #storage_type = #enum_ident::#ident as #storage_type;
                }
            });
    let arms = data_enum
        .variants
        .iter()
        .zip(const_idents)
        .map(|(variant, const_ident)| {
            let ident = &variant.ident;
            quote! {
                #const_ident => Self::#ident,
            }
        });
    quote! {
        {
            #(#consts)*
            const END: #storage_type = 1 << (#bit_len);
            match bits {
                #(#arms)*
                END.. => todo!(),
            }
        }
    }
}

pub fn bitpiece_enum(
    input: &DeriveInput,
    data_enum: &DataEnum,
    explicit_bit_length: Option<usize>,
) -> proc_macro::TokenStream {
    if data_enum
        .variants
        .iter()
        .any(|variant| !matches!(variant.fields, Fields::Unit))
    {
        return not_supported_err("enum variants with data");
    }
    let bit_len = BitLenExpr(enum_bit_len(&input.ident, data_enum));
    let storage_type = bit_len.storage_type();

    let explicit_bit_len_assertion =
        gen_explicit_bit_length_assertion(explicit_bit_length, &bit_len);

    let implementation = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: input.ident.clone(),
        mut_type: quote! { ::bitpiece::GenericBitPieceMut<'s, S, Self> },
        serialization_code: quote! { self as #storage_type },
        deserialization_code: gen_deserialization_code(
            &input.ident,
            data_enum,
            &bit_len,
            &storage_type,
        ),
        storage_type,
        bit_len,
    });

    quote! {
        #explicit_bit_len_assertion
        #input
        #implementation
    }
    .into()
}
