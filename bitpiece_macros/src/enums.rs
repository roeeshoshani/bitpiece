use convert_case::Casing;
use quote::{format_ident, quote, ToTokens};
use syn::{DataEnum, DeriveInput, Fields};

use crate::{
    newtypes::{BitLenExpr, TypeExpr},
    utils::{bitpiece_gen_impl, not_supported_err, BitPieceGenImplParams},
    MacroArgs,
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

fn gen_try_from_bits_code(
    enum_ident: &syn::Ident,
    data_enum: &DataEnum,
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
                #const_ident => Some(Self::#ident),
            }
        });
    quote! {
        {
            #(#consts)*
            match bits {
                #(#arms)*
                _ => None,
            }
        }
    }
}

pub fn bitpiece_enum(
    input: &DeriveInput,
    data_enum: &DataEnum,
    macro_args: MacroArgs,
) -> proc_macro::TokenStream {
    if data_enum
        .variants
        .iter()
        .any(|variant| !matches!(variant.fields, Fields::Unit))
    {
        return not_supported_err("enum variants with data");
    }

    let bit_len_calc = match macro_args.explicit_bit_length {
        Some(explicit_bit_len) => BitLenExpr(quote! {#explicit_bit_len}),
        None => BitLenExpr(enum_bit_len(&input.ident, data_enum)),
    };
    let bit_len_ident = proc_macro2::Ident::new(
        &format!(
            "{}_BIT_LEN",
            input
                .ident
                .to_string()
                .to_case(convert_case::Case::Constant)
        ),
        input.ident.span(),
    );
    let bit_len = BitLenExpr(bit_len_ident.to_token_stream());

    let storage_type_calc = bit_len.storage_type();
    let storage_type_ident = format_ident!("{}StorageTy", input.ident);
    let storage_type = TypeExpr(storage_type_ident.to_token_stream());

    let implementation = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: input.ident.clone(),
        mut_type_ident: todo!(),
        to_bits_code: quote! { self as #storage_type },
        try_from_bits_code: gen_try_from_bits_code(&input.ident, data_enum, &storage_type),
        fields_type: TypeExpr(quote! { Self }),
        to_fields_code: quote! { self },
        from_fields_code: quote! { fields },
        storage_type,
        bit_len,
        zeroes: todo!(),
        ones: todo!(),
        min: todo!(),
        max: todo!(),
    });

    let vis = &input.vis;

    quote! {
        #vis const #bit_len_ident: usize = #bit_len_calc;
        #vis type #storage_type_ident = #storage_type_calc;

        #input

        #implementation
    }
    .into()
}
