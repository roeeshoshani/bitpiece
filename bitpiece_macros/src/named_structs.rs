use convert_case::Casing;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, DeriveInput, FieldsNamed};

use crate::{
    newtypes::{BitLenExpr, BitOffsetExpr, TypeExpr},
    utils::{
        bitpiece_gen_impl, gen_deserialization_code, gen_explicit_bit_length_assertion,
        not_supported_err, BitPieceGenImplParams,
    },
};

pub fn bitpiece_named_struct(
    input: &DeriveInput,
    fields: &FieldsNamed,
    explicit_bit_length: Option<usize>,
) -> proc_macro::TokenStream {
    if fields.named.is_empty() {
        return not_supported_err("empty structs");
    }

    let ident = &input.ident;

    let field_types = fields
        .named
        .iter()
        .map(|field| TypeExpr::from_type(&field.ty));
    let bit_len_calc: BitLenExpr = field_types.clone().map(|field_ty| field_ty.bit_len()).sum();
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

    let fields_struct_ident = format_ident!("{}Fields", input.ident);
    let fields_struct_modified_fields = gen_fields_struct_modified_fields(fields);

    let ident_mut = format_ident!("{}Mut", input.ident);
    let bitpiece_impl = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: input.ident.clone(),
        bit_len: bit_len.clone(),
        storage_type: storage_type.clone(),
        serialization_code: quote! { self.storage },
        deserialization_code: gen_deserialization_code(&input.ident),
        try_deserialization_code: Some(gen_try_deserialization_code(fields, &storage_type)),
        mut_type: quote! { #ident_mut<'s, S> },
        fields_type: TypeExpr(quote! { #fields_struct_ident }),
        to_fields_code: gen_to_fields(fields, &fields_struct_ident),
        from_fields_code: gen_from_fields(fields, input),
    });
    let bitpiece_mut_impl = bitpiece_mut_gen_impl(&ident_mut, &input.ident);

    let field_access_fns = gen_field_access_fns(fields, &storage_type);
    let field_access_noshift_fns = gen_field_access_noshift_fns(fields, &storage_type);
    let field_set_fns = gen_field_set_fns(fields, &storage_type);
    let field_mut_fns = gen_field_mut_fns(fields, &storage_type);

    let mut_struct_field_access_fns = gen_mut_struct_field_access_fns(fields);
    let mut_struct_field_set_fns = gen_mut_struct_field_set_fns(fields);
    let mut_struct_field_mut_fns = gen_mut_struct_field_mut_fns(fields);

    let explicit_bit_len_assertion =
        gen_explicit_bit_length_assertion(explicit_bit_length, &bit_len);

    let vis = &input.vis;
    let attrs = &input.attrs;
    quote! {
        #vis const #bit_len_ident: usize = #bit_len_calc;
        #vis type #storage_type_ident = #storage_type_calc;

        #explicit_bit_len_assertion

        #[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        #vis struct #ident {
            storage: #storage_type,
        }

        #bitpiece_impl

        impl #ident {
            #(#field_access_fns)*
            #(#field_access_noshift_fns)*
            #(#field_set_fns)*
            #(#field_mut_fns)*
        }

        #vis struct #ident_mut<'s, S: ::bitpiece::BitStorage> {
            bits: ::bitpiece::BitsMut<'s, S>,
        }

        #bitpiece_mut_impl

        impl<'s, S: ::bitpiece::BitStorage> #ident_mut<'s, S> {
            #(#mut_struct_field_access_fns)*
            #(#mut_struct_field_set_fns)*
            #(#mut_struct_field_mut_fns)*
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

fn gen_fields_struct_modified_fields<'a>(fields: &'a FieldsNamed) -> FieldsNamed {
    let mut modified_fields = fields.clone();
    for field in &mut modified_fields.named {
        let ty = &field.ty;
        let inner_fields_ty = TypeExpr(quote! { #ty }).fields_ty().0;
        field.ty = parse_quote! { #inner_fields_ty };
    }
    modified_fields
}

/// returns an iterator over the extracted bits of each field.
fn fields_extracted_bits<'a, I: Iterator<Item = &'a syn::Field> + 'a>(
    fields: I,
    storage_type: &'a TypeExpr,
    storage_bits_expr: proc_macro2::TokenStream,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields).map(move |offset_and_len| {
        let FieldOffsetAndLen { len, offset, signed } = offset_and_len;
        extract_bits(ExtractBitsParams {
            value: storage_bits_expr.clone(),
            value_type: storage_type.clone(),
            extract_offset: offset,
            extract_len: len,
            signed
        })
    })
}

/// returns an iterator over the extracted bits (mask only, no shift) of each field.
fn fields_extracted_bits_noshift<'a, I: Iterator<Item = &'a syn::Field> + 'a>(
    fields: I,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields).map(|offset_and_len| {
        let FieldOffsetAndLen { len, offset, signed } = offset_and_len;
        extract_bits_noshift(ExtractBitsParams {
            value: quote! { self.storage },
            value_type: storage_type.clone(),
            extract_offset: offset,
            extract_len: len,
            signed
        })
    })
}

/// returns an iterator over the bit offset and bit length of each field.
fn fields_offsets_and_lens<'a, I: Iterator<Item = &'a syn::Field> + 'a>(
    fields: I,
) -> impl Iterator<Item = FieldOffsetAndLen> + 'a {
    fields.scan(BitLenExpr::zero(), |prev_fields_bit_len, cur_field| {
        let cur_field_bit_len = TypeExpr::from_type(&cur_field.ty).bit_len();
        let signed = TypeExpr::from_type(&cur_field.ty).signed();
        let new_bit_len = &*prev_fields_bit_len + &cur_field_bit_len;

        // the offset of this field is the len of all previous fields, and update the prev len to the new len.
        let offset = core::mem::replace(prev_fields_bit_len, new_bit_len);

        Some(FieldOffsetAndLen {
            len: cur_field_bit_len,
            offset: BitOffsetExpr(offset.0),
            signed,
        })
    })
}

/// parameters for extracting some range of bits from a value
struct ExtractBitsParams {
    /// the value to extract the bits from
    value: proc_macro2::TokenStream,
    /// the type of the value to extract the bits from
    value_type: TypeExpr,
    /// the offset at which to start extracting
    extract_offset: BitOffsetExpr,
    /// the amount of bits to extract
    extract_len: BitLenExpr,
    /// Signed-ness of field
    signed: bool,
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
        signed,
    } = &params;
        quote! {
            (
                ::bitpiece::extract_bits::<#signed>(#value as u64, #extract_offset, #extract_len) as #value_type
                // ::bitpiece::extract_bits(#value as u64, #extract_offset, #extract_len, #signed) as #value_type
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
        ..
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
                ..
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

fn gen_try_deserialization_code(
    fields: &FieldsNamed,
    storage_type: &TypeExpr,
) -> proc_macro2::TokenStream {
    // before constructing the type, make sure that the values of all fields are valid
    let per_field_call = fields_extracted_bits(fields.named.iter(), storage_type, quote! { bits })
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

fn gen_field_access_fns<'a>(
    fields: &'a FieldsNamed,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_extracted_bits(fields.named.iter(), storage_type, quote!{ self.storage })
        .zip(fields.named.iter())
        .map(|(bits, field)| {
            let vis = &field.vis;
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                #vis fn #ident (self) -> #ty {
                    <#ty as ::bitpiece::BitPiece>::from_bits(#bits as <#ty as ::bitpiece::BitPiece>::Bits)
                }
            }
        })
}

fn gen_field_access_noshift_fns<'a>(
    fields: &'a FieldsNamed,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_extracted_bits_noshift(fields.named.iter(), storage_type)
        .zip(fields.named.iter())
        .map(move |(bits, field)| {
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ident_noshift = format_ident!("{}_noshift", ident);
            quote! {
                #vis fn #ident_noshift (self) -> #storage_type {
                    #bits
                }
            }
        })
}

fn gen_mut_struct_field_access_fns<'a>(
    fields: &'a FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { len, offset, .. } = offset_and_len;
            let vis = &field.vis;
            let ident = &field.ident;
            let ty = &field.ty;
            quote! {
                #vis fn #ident(&self) -> #ty {
                    <#ty as ::bitpiece::BitPiece>::from_bits(
                        self.bits.get_bits(#offset, #len) as <#ty as ::bitpiece::BitPiece>::Bits
                    )
                }
            }
        })
}

fn gen_mut_struct_field_set_fns<'a>(
    fields: &'a FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { len, offset, .. } = offset_and_len;
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let set_ident = format_ident!("set_{}", ident);
            quote! {
                #vis fn #set_ident(&mut self, new_value: #ty) {
                    let new_value_bits = <#ty as ::bitpiece::BitPiece>::to_bits(new_value);
                    self.bits.set_bits(#offset, #len, new_value_bits as u64)
                }
            }
        })
}

fn gen_mut_struct_field_mut_fns<'a>(
    fields: &'a FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { offset, .. } = offset_and_len;
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let ident_mut = format_ident!("{}_mut", ident);
            let mut_ty = quote! {
                <#ty as ::bitpiece::BitPiece>::Mut<'s, S>
            };
            quote! {
                #vis fn #ident_mut<'a: 's>(&'a mut self) -> #mut_ty {
                    <
                        #mut_ty as ::bitpiece::BitPieceMut<'s, S, #ty>
                    >::new(self.bits.storage, self.bits.start_bit_index + #offset)
                }
            }
        })
}

fn gen_field_set_fns<'a>(
    fields: &'a FieldsNamed,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { len, offset, signed } = offset_and_len;
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
                    signed,
                },
                new_value: quote! { <#ty as ::bitpiece::BitPiece>::to_bits(new_value) },
            });
            quote! {
                #vis fn #set_ident (&mut self, new_value: #ty) {
                    self.storage = #modified_value_expr;
                }
            }
        })
}

fn gen_field_mut_fns<'a>(
    fields: &'a FieldsNamed,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { offset, .. } = offset_and_len;
            let vis = &field.vis;
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let ident_mut = format_ident!("{}_mut", ident);
            let storage_type = storage_type.clone();
            let mut_ty = quote! {
                <#ty as ::bitpiece::BitPiece>::Mut<'s, #storage_type>
            };
            quote! {
                #vis fn #ident_mut<'s>(&'s mut self) -> #mut_ty {
                    <
                        #mut_ty as ::bitpiece::BitPieceMut<'s, #storage_type, #ty>
                    >::new(&mut self.storage, #offset)
                }
            }
        })
}
/// information about the offset and len of a field.
struct FieldOffsetAndLen {
    len: BitLenExpr,
    offset: BitOffsetExpr,
    signed: bool,
}

/// generates the final implementation of the `BitPieceMut` trait given the implementation details.
fn bitpiece_mut_gen_impl(ident_mut: &syn::Ident, ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        #[automatically_derived]
        impl<'s, S: ::bitpiece::BitStorage> ::bitpiece::BitPieceMut<'s, S, #ident> for #ident_mut<'s, S> {
            fn new(storage: &'s mut S, start_bit_index: usize) -> Self {
                Self {
                    bits: ::bitpiece::BitsMut::new(storage, start_bit_index),
                }
            }
            fn get(&self) -> #ident {
                let bits_u64 = self.bits.get_bits(0, <#ident as ::bitpiece::BitPiece>::BITS);
                let bits = <<#ident as ::bitpiece::BitPiece>::Bits as ::bitpiece::BitStorage>::from_u64(bits_u64).unwrap();
                <#ident as ::bitpiece::BitPiece>::from_bits(bits)
            }
            fn set(&mut self, new_value: #ident) {
                let bits = <#ident as ::bitpiece::BitPiece>::to_bits(new_value);
                let bits_u64 = <<#ident as ::bitpiece::BitPiece>::Bits as ::bitpiece::BitStorage>::to_u64(bits);
                self.bits
                    .set_bits(0, <#ident as ::bitpiece::BitPiece>::BITS, bits_u64)
            }
        }
    }
}
