use convert_case::Casing;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, DeriveInput, Field, FieldsNamed};

use crate::{
    newtypes::{BitLenExpr, BitOffsetExpr, StorageTypeExpr, TypeExpr},
    utils::{
        bitpiece_gen_impl, gen_explicit_bit_length_assertion, not_supported_err,
        BitPieceGenImplParams,
    },
    MacroArgs,
};

pub fn bitpiece_named_struct(
    input: &DeriveInput,
    fields: &FieldsNamed,
    macro_args: MacroArgs,
) -> proc_macro::TokenStream {
    if fields.named.is_empty() {
        return not_supported_err("empty structs");
    }

    let ident = &input.ident;

    let bit_len_calc = calc_bit_len(fields);
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
    let storage_type = StorageTypeExpr(storage_type_ident.to_token_stream());

    let fields_struct_ident = format_ident!("{}Fields", input.ident);
    let fields_struct_modified_fields = gen_fields_struct_modified_fields(fields);

    let mut_type_ident = format_ident!("{}MutRef", input.ident);
    let fields_type = TypeExpr(quote! { #fields_struct_ident });

    let fields_offsets_and_lens_consts = gen_fields_offsets_and_lens_consts(ident, fields);

    let bitpiece_impl = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: input.ident.clone(),
        bit_len: bit_len.clone(),
        storage_type: storage_type.clone(),
        to_bits_code: quote! { self.storage },
        try_from_bits_code: gen_try_from_bits_code(ident, fields, &storage_type),
        mut_type_ident: mut_type_ident.clone(),
        zeroes: gen_const_instantiation(ident, fields, &fields_type, "ZEROES"),
        ones: gen_const_instantiation(ident, fields, &fields_type, "ONES"),
        min: gen_const_instantiation(ident, fields, &fields_type, "MIN"),
        max: gen_const_instantiation(ident, fields, &fields_type, "MAX"),
        to_fields_code: gen_to_fields(fields, &fields_struct_ident),
        from_fields_code: gen_from_fields(fields, input),
        fields_type,
    });

    let field_access_fns = gen_field_access_fns(ident, fields, &storage_type);
    let field_access_noshift_fns = gen_field_access_noshift_fns(ident, fields, &storage_type);
    let field_set_fns = gen_field_set_fns(ident, fields, &storage_type);
    let field_mut_fns = gen_field_mut_fns(ident, fields, &storage_type);

    let mut_struct_field_access_fns = gen_mut_struct_field_access_fns(ident, fields);
    let mut_struct_field_set_fns = gen_mut_struct_field_set_fns(ident, fields);
    let mut_struct_field_mut_fns = gen_mut_struct_field_mut_fns(ident, fields);

    let explicit_bit_len_assertion =
        gen_explicit_bit_length_assertion(macro_args.explicit_bit_length, &bit_len);

    let vis = &input.vis;
    let attrs = &input.attrs;
    quote! {
        #vis const #bit_len_ident: usize = #bit_len_calc;
        #vis type #storage_type_ident = #storage_type_calc;

        #explicit_bit_len_assertion

        #[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        #vis struct #ident {
            pub storage: #storage_type,
        }

        #bitpiece_impl

        impl #ident {
            #fields_offsets_and_lens_consts
            #field_access_fns
            #field_access_noshift_fns
            #field_set_fns
            #field_mut_fns
        }

        ::bitpiece::bitpiece_define_mut_ref_type! { #ident, #mut_type_ident }

        impl<'s> #mut_type_ident<'s> {
            #mut_struct_field_access_fns
            #mut_struct_field_set_fns
            #mut_struct_field_mut_fns
        }

        impl ::core::convert::From<#fields_struct_ident> for #ident {
            fn from(fields: #fields_struct_ident) -> Self {
                <Self as ::bitpiece::BitPiece>::from_fields(fields)
            }
        }
        impl ::core::convert::From<#ident> for #fields_struct_ident {
            fn from(value: #ident) -> Self {
                <#ident as ::bitpiece::BitPiece>::to_fields(value)
            }
        }

        #(#attrs)*
        #vis struct #fields_struct_ident #fields_struct_modified_fields
    }
    .into()
}

fn gen_const_instantiation(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
    fields_type: &TypeExpr,
    const_name: &str,
) -> proc_macro2::TokenStream {
    let const_name_ident = format_ident!("{}", const_name);
    let instantiate_each_field = fields.named.iter().map(|f| {
        let field_ident = &f.ident;
        let field_type = &f.ty;
        quote! {
            #field_ident: <#field_type as ::bitpiece::BitPiece>::#const_name_ident,
        }
    });
    quote! {
        #type_ident::from_fields(#fields_type {
            #(#instantiate_each_field)*
        })
    }
}

fn get_field_offset_const_ident(field: &Field) -> syn::Ident {
    let field_name_const_case = field
        .ident
        .as_ref()
        .unwrap()
        .to_string()
        .to_case(convert_case::Case::Constant);
    format_ident!("{}_OFFSET", field_name_const_case)
}

fn get_field_len_const_ident(field: &Field) -> syn::Ident {
    let field_name_const_case = field
        .ident
        .as_ref()
        .unwrap()
        .to_string()
        .to_case(convert_case::Case::Constant);
    format_ident!("{}_LEN", field_name_const_case)
}

fn get_field_offset(type_ident: &syn::Ident, field: &Field) -> BitOffsetExpr {
    let const_ident = get_field_offset_const_ident(field);
    BitOffsetExpr(quote! {
        #type_ident::#const_ident
    })
}

fn get_field_len(type_ident: &syn::Ident, field: &Field) -> BitLenExpr {
    let const_ident = get_field_len_const_ident(field);
    BitLenExpr(quote! {
        #type_ident::#const_ident
    })
}

fn gen_fields_offsets_and_lens_consts(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    // iterator over each fields and its previous field. in the first iteration, the previous fields is `None`.
    let fields_with_prev = fields.named.iter().enumerate().map(|(i, field)| {
        let prev = if i == 0 {
            None
        } else {
            Some(&fields.named[i - 1])
        };
        (prev, field)
    });

    fields_with_prev
        .map(|(prev, cur)| {
            let offset_const_ident = get_field_offset_const_ident(cur);
            let len_const_ident = get_field_len_const_ident(cur);
            let offset = match prev {
                Some(prev) => {
                    let prev_offset = get_field_offset(type_ident, prev);
                    let prev_len = get_field_len(type_ident, prev);
                    quote! {
                        (#prev_offset) + (#prev_len)
                    }
                }
                None => quote! { 0 },
            };
            let len = TypeExpr::from_type(&cur.ty).bit_len();
            quote! {
                const #len_const_ident: usize = #len;
                const #offset_const_ident: usize = #offset;
            }
        })
        .collect()
}

fn calc_bit_len(fields: &FieldsNamed) -> BitLenExpr {
    let field_types = fields
        .named
        .iter()
        .map(|field| TypeExpr::from_type(&field.ty));
    field_types.clone().map(|field_ty| field_ty.bit_len()).sum()
}

fn gen_fields_struct_modified_fields(fields: &FieldsNamed) -> FieldsNamed {
    let mut modified_fields = fields.clone();
    for field in &mut modified_fields.named {
        let ty = &field.ty;
        let inner_fields_ty = TypeExpr(quote! { #ty }).fields_ty().0;
        field.ty = parse_quote! { #inner_fields_ty };
    }
    modified_fields
}

/// returns an iterator over the extracted bits of each field.
fn fields_extracted_bits<'a>(
    type_ident: &'a syn::Ident,
    fields: &'a FieldsNamed,
    storage_type: &'a StorageTypeExpr,
    storage_bits_expr: proc_macro2::TokenStream,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields.named.iter().map(move |field| {
        let len = get_field_len(type_ident, field);
        let offset = get_field_offset(type_ident, field);
        extract_bits(ExtractBitsParams {
            value: storage_bits_expr.clone(),
            value_type: storage_type.clone(),
            extract_offset: offset,
            extract_len: len,
        })
    })
}

/// returns an iterator over the extracted bits (mask only, no shift) of each field.
fn fields_extracted_bits_noshift<'a>(
    type_ident: &'a syn::Ident,
    fields: &'a FieldsNamed,
    storage_type: &'a StorageTypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields.named.iter().map(move |field| {
        let len = get_field_len(type_ident, field);
        let offset = get_field_offset(type_ident, field);
        extract_bits_noshift(ExtractBitsParams {
            value: quote! { self.storage },
            value_type: storage_type.clone(),
            extract_offset: offset,
            extract_len: len,
        })
    })
}

/// parameters for extracting some range of bits from a value
struct ExtractBitsParams {
    /// the value to extract the bits from
    value: proc_macro2::TokenStream,
    /// the type of the value to extract the bits from
    value_type: StorageTypeExpr,
    /// the offset at which to start extracting
    extract_offset: BitOffsetExpr,
    /// the amount of bits to extract
    extract_len: BitLenExpr,
}

/// parameters for modifying some range of bits of a value
struct ModifyBitsParams {
    /// the parameters used for extracting the range of bits to be modified.
    extract_params: ExtractBitsParams,
    /// the new value of the specified bit range.
    new_value: proc_macro2::TokenStream,
}

/// extracts some bits from a value
fn extract_bits(params: ExtractBitsParams) -> proc_macro2::TokenStream {
    let ExtractBitsParams {
        value,
        value_type,
        extract_offset,
        extract_len,
    } = &params;
    quote! {
        (
            ::bitpiece::extract_bits(#value as u64, #extract_offset, #extract_len) as #value_type
        )
    }
}

/// extracts some bits (mask only, no shift) from a value
fn extract_bits_noshift(params: ExtractBitsParams) -> proc_macro2::TokenStream {
    let ExtractBitsParams {
        value,
        value_type,
        extract_offset,
        extract_len,
    } = &params;
    quote! {
        (
            ::bitpiece::extract_bits_noshift(#value as u64, #extract_offset, #extract_len) as #value_type
        )
    }
}

/// returns an expression for the provided value with the specified bit range modified to its new value.
fn modify_bits(params: ModifyBitsParams) -> proc_macro2::TokenStream {
    let ModifyBitsParams {
        extract_params:
            ExtractBitsParams {
                value,
                value_type,
                extract_offset,
                extract_len,
            },
        new_value,
    } = params;
    quote! {
        (
            ::bitpiece::modify_bits(#value as u64, #extract_offset, #extract_len, #new_value as u64) as #value_type
        )
    }
}

fn gen_from_fields<'a>(fields: &'a FieldsNamed, input: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let field_set_calls = fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_set_fn_ident = format_ident!("set_{}", field_ident);
        quote! {
            result.#field_set_fn_ident(::bitpiece::BitPiece::from_fields(fields.#field_ident));
        }
    });
    quote! {
        let mut result = #ident::zeroes();
        #(#field_set_calls)*
        result
    }
}

fn gen_try_from_bits_code(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
    storage_type: &StorageTypeExpr,
) -> proc_macro2::TokenStream {
    // before constructing the type, make sure that the values of all fields are valid
    let per_field_call = fields_extracted_bits(type_ident, fields, storage_type, quote! { bits })
        .zip(fields.named.iter())
        .map(|(bits, field)| {
            let ty = &field.ty;
            quote! {
                let _ = <#ty as ::bitpiece::BitPiece>::try_from_bits(#bits as <#ty as ::bitpiece::BitPiece>::Bits)?;
            }
        });
    quote! {
        let result = Self { storage: bits };
        #(#per_field_call)*
        Some(result)
    }
}

fn gen_to_fields<'a>(
    fields: &'a FieldsNamed,
    fields_struct_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let field_initializers = fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! {
            #field_ident: ::bitpiece::BitPiece::to_fields(self.#field_ident()),
        }
    });
    quote! {
        #fields_struct_ident {
            #(#field_initializers)*
        }
    }
}

fn gen_field_access_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
    storage_type: &StorageTypeExpr,
) -> proc_macro2::TokenStream {
    fields_extracted_bits(type_ident, fields, storage_type, quote!{ self.storage })
        .zip(fields.named.iter())
        .map(|(bits, field)| {
            let vis = &field.vis;
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                #vis const fn #ident (self) -> #ty {
                    <#ty as ::bitpiece::BitPiece>::from_bits(#bits as <#ty as ::bitpiece::BitPiece>::Bits)
                }
            }
        }).collect()
}

fn gen_field_access_noshift_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
    storage_type: &StorageTypeExpr,
) -> proc_macro2::TokenStream {
    fields_extracted_bits_noshift(type_ident, fields, storage_type)
        .zip(fields.named.iter())
        .map(move |(bits, field)| {
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ident_noshift = format_ident!("{}_noshift", ident);
            quote! {
                #vis const fn #ident_noshift (self) -> #storage_type {
                    #bits
                }
            }
        })
        .collect()
}

fn gen_mut_struct_field_access_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    fields
        .named
        .iter()
        .map(|field| {
            let len = get_field_len(type_ident, field);
            let offset = get_field_offset(type_ident, field);
            let vis = &field.vis;
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                #vis const fn #ident(&self) -> #ty {
                    <#ty as ::bitpiece::BitPiece>::from_bits(
                        self.bits.get_bits(#offset, #len) as <#ty as ::bitpiece::BitPiece>::Bits
                    )
                }
            }
        })
        .collect()
}

fn gen_mut_struct_field_set_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    fields
        .named
        .iter()
        .map(|field| {
            let len = get_field_len(type_ident, field);
            let offset = get_field_offset(type_ident, field);
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let set_ident = format_ident!("set_{}", ident);
            quote! {
                #vis const fn #set_ident(&mut self, new_value: #ty) {
                    let new_value_bits = <#ty as ::bitpiece::BitPiece>::to_bits(new_value);
                    self.bits.set_bits(#offset, #len, new_value_bits as u64)
                }
            }
        })
        .collect()
}

fn gen_mut_struct_field_mut_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    fields
        .named
        .iter()
        .map(|field| {
            let offset = get_field_offset(type_ident, field);
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let ident_mut = format_ident!("{}_mut", ident);
            let mut_ty = quote! {
                <#ty as ::bitpiece::BitPieceHasMutRef>::MutRef<'s>
            };
            quote! {
                #vis const fn #ident_mut(&'s mut self) -> #mut_ty {
                    #mut_ty::new(self.bits.storage, self.bits.start_bit_index + #offset)
                }
            }
        })
        .collect()
}

fn gen_field_set_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
    storage_type: &StorageTypeExpr,
) -> proc_macro2::TokenStream {
    fields
        .named
        .iter()
        .map(|field| {
            let len = get_field_len(type_ident, field);
            let offset = get_field_offset(type_ident, field);
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let set_ident = format_ident!("set_{}", ident);
            let modified_value_expr = modify_bits(ModifyBitsParams {
                extract_params: ExtractBitsParams {
                    value: quote! { self.storage },
                    value_type: storage_type.clone(),
                    extract_offset: offset,
                    extract_len: len,
                },
                new_value: quote! { <#ty as ::bitpiece::BitPiece>::Converter::to_bits(new_value) },
            });
            quote! {
                #vis const fn #set_ident (&mut self, new_value: #ty) {
                    self.storage = #modified_value_expr;
                }
            }
        })
        .collect()
}

fn gen_field_mut_fns(
    type_ident: &syn::Ident,
    fields: &FieldsNamed,
    storage_type: &StorageTypeExpr,
) -> proc_macro2::TokenStream {
    fields
        .named
        .iter()
        .map(move |field| {
            let offset = get_field_offset(type_ident, field);
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let ident_mut = format_ident!("{}_mut", ident);
            let storage_type = storage_type.clone();
            let mut_ty = quote! {
                <#ty as ::bitpiece::BitPieceHasMutRef>::MutRef<'s>
            };
            quote! {
                #vis const fn #ident_mut<'s>(&'s mut self) -> #mut_ty {
                    <
                        #mut_ty as ::bitpiece::BitPieceMut<'s, #storage_type, #ty>
                    >::new(&mut self.storage, #offset)
                }
            }
        })
        .collect()
}
