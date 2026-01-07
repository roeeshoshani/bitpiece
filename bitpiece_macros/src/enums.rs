use convert_case::Casing;
use quote::{format_ident, quote, ToTokens};
use syn::{DataEnum, DeriveInput, Fields};

use crate::{
    newtypes::{BitLenExpr, StorageTypeExpr, TypeExpr},
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
        quote! { #value as u64 }
    })
}

fn gen_enum_variant_u64_values_array(
    enum_ident: &syn::Ident,
    data_enum: &DataEnum,
) -> proc_macro2::TokenStream {
    let values = enum_variant_u64_values(enum_ident, data_enum);
    quote! {
        [#(#values,)*]
    }
}

fn enum_bit_len(num_variants: usize, u64_values_array: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        {
            const BITS_REQUIRED_FOR_EACH_VALUE: [u64; #num_variants] = {
                let mut res = [0u64; #num_variants];
                use ::bitpiece::const_for;
                const_for!(i in 0..#num_variants => {
                    let variant_value = #u64_values_array[i];
                    let bits_required = (64 - variant_value.leading_zeros() as u64);
                    res[i] = bits_required;
                });
                res
            };
            ::bitpiece::const_array_max_u64(&BITS_REQUIRED_FOR_EACH_VALUE)
        }
    }
}

fn gen_try_from_bits_code(
    enum_ident: &syn::Ident,
    data_enum: &DataEnum,
    storage_type: &StorageTypeExpr,
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

    let ident = &input.ident;
    let num_variants = data_enum.variants.len();

    let u64_values_calc = gen_enum_variant_u64_values_array(ident, data_enum);
    let u64_values_ident = proc_macro2::Ident::new(
        &format!(
            "{}_VARIANT_VALUES_U64",
            input
                .ident
                .to_string()
                .to_case(convert_case::Case::Constant)
        ),
        ident.span(),
    );

    let bit_len_calc = match macro_args.explicit_bit_length {
        Some(explicit_bit_len) => BitLenExpr(quote! {#explicit_bit_len}),
        None => BitLenExpr(enum_bit_len(num_variants, &u64_values_ident)),
    };
    let bit_len_ident = proc_macro2::Ident::new(
        &format!(
            "{}_BIT_LEN",
            input
                .ident
                .to_string()
                .to_case(convert_case::Case::Constant)
        ),
        ident.span(),
    );
    let bit_len = BitLenExpr(bit_len_ident.to_token_stream());

    let storage_type_calc = bit_len.storage_type();
    let storage_type_ident = format_ident!("{}StorageTy", ident);
    let storage_type = StorageTypeExpr(storage_type_ident.to_token_stream());

    let mut_type_ident = format_ident!("{}MutRef", ident);

    let min_u64_val = quote! { ::bitpiece::const_array_min_u64(&#u64_values_ident) };
    let max_u64_val = quote! { ::bitpiece::const_array_max_u64(&#u64_values_ident) };
    let min_variant = quote! { #ident::from_bits(#min_u64_val as #storage_type) };
    let max_variant = quote! { #ident::from_bits(#max_u64_val as #storage_type) };

    let implementation = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: ident.clone(),
        mut_type_ident: mut_type_ident.clone(),
        to_bits_code: quote! { self as #storage_type },
        try_from_bits_code: gen_try_from_bits_code(ident, data_enum, &storage_type),
        fields_type: TypeExpr(quote! { Self }),
        to_fields_code: quote! { self },
        from_fields_code: quote! { fields },
        storage_type,
        bit_len,
        zeroes: min_variant.clone(),
        ones: max_variant.clone(),
        min: min_variant,
        max: max_variant,
    });

    let vis = &input.vis;

    quote! {
        #vis const #u64_values_ident: [u64; #num_variants] = #u64_values_calc;
        #vis const #bit_len_ident: usize = (#bit_len_calc) as usize;
        #vis type #storage_type_ident = #storage_type_calc;

        #input

        ::bitpiece::bitpiece_define_mut_ref_type! { #ident, #mut_type_ident, #vis }

        #implementation
    }
    .into()
}
