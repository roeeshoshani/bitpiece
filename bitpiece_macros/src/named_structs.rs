use quote::{format_ident, quote};
use syn::{DeriveInput, FieldsNamed};

use crate::{
    newtypes::{BitLenExpr, BitOffsetExpr, TypeExpr},
    utils::{are_generics_empty, not_supported_err},
};

pub fn bitpiece_named_struct(input: &DeriveInput, fields: &FieldsNamed) -> proc_macro::TokenStream {
    if !are_generics_empty(&input.generics) {
        return not_supported_err("generics");
    }
    if fields.named.is_empty() {
        return not_supported_err("empty structs");
    }
    let field_types = fields
        .named
        .iter()
        .map(|field| TypeExpr::from_type(&field.ty));
    let total_bit_length: BitLenExpr = field_types.clone().map(|field_ty| field_ty.bit_len()).sum();
    let storage_type = total_bit_length.storage_type();

    let ident_mut = format_ident!("{}Mut", input.ident);
    let bitpiece_impl = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: input.ident.clone(),
        bit_len: total_bit_length,
        storage_type: storage_type.clone(),
        serialization_code: quote! { self.storage },
        deserialization_code: quote! { Self { storage: bits } },
        ident_mut: ident_mut.clone(),
    });
    let bitpiece_mut_impl = bitpiece_mut_gen_impl(&ident_mut, &input.ident);

    let field_access_fns = field_access_fns(fields, &storage_type);
    let field_set_fns = field_set_fns(fields, &storage_type);
    let field_mut_fns = field_mut_fns(fields, &storage_type);

    let mut_struct_field_access_fns = mut_struct_field_access_fns(fields);
    let mut_struct_field_set_fns = mut_struct_field_set_fns(fields);
    let mut_struct_field_mut_fns = mut_struct_field_mut_fns(fields);

    let vis = &input.vis;
    let ident = &input.ident;
    let attrs = &input.attrs;
    quote! {
        #(#attrs)*
        #vis struct #ident {
            storage: #storage_type,
        }

        #bitpiece_impl

        impl #ident {
            #(#field_access_fns)*
            #(#field_set_fns)*
            #(#field_mut_fns)*
        }

        #vis struct #ident_mut<'s, S: ::bitpiece::BitStorage> {
            bits: ::bitpiece::BitsMut<'s, S, #ident>,
        }

        #bitpiece_mut_impl

        impl<'s, S: ::bitpiece::BitStorage> #ident_mut<'s, S> {
            #(#mut_struct_field_access_fns)*
            #(#mut_struct_field_set_fns)*
            #(#mut_struct_field_mut_fns)*
        }
    }
    .into()
}

/// returns an iterator over the extracted bits of each field.
fn fields_extracted_bits<'a, I: Iterator<Item = &'a syn::Field> + 'a>(
    fields: I,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields).map(|offset_and_len| {
        let FieldOffsetAndLen { len, offset } = offset_and_len;
        extract_bits(ExtractBitsParams {
            value: quote! { self.storage },
            value_type: storage_type.clone(),
            extract_offset: offset,
            extract_len: len,
        })
    })
}
/// returns an iterator over the bit offset and bit length of each field.
fn fields_offsets_and_lens<'a, I: Iterator<Item = &'a syn::Field> + 'a>(
    fields: I,
) -> impl Iterator<Item = FieldOffsetAndLen> + 'a {
    fields.scan(BitLenExpr::zero(), |prev_fields_bit_len, cur_field| {
        let cur_field_bit_len = TypeExpr::from_type(&cur_field.ty).bit_len();
        let new_bit_len = &*prev_fields_bit_len + &cur_field_bit_len;

        // the offset of this field is the len of all previous fields, and update the prev len to the new len.
        let offset = core::mem::replace(prev_fields_bit_len, new_bit_len);

        Some(FieldOffsetAndLen {
            len: cur_field_bit_len,
            offset: BitOffsetExpr(offset.0),
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

fn field_access_fns<'a>(
    fields: &'a FieldsNamed,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_extracted_bits(fields.named.iter(), storage_type)
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

fn mut_struct_field_access_fns<'a>(
    fields: &'a FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { len, offset } = offset_and_len;
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

fn mut_struct_field_set_fns<'a>(
    fields: &'a FieldsNamed,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { len, offset } = offset_and_len;
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

fn mut_struct_field_mut_fns<'a>(
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

fn field_set_fns<'a>(
    fields: &'a FieldsNamed,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields.named.iter())
        .zip(fields.named.iter())
        .map(|(offset_and_len, field)| {
            let FieldOffsetAndLen { len, offset } = offset_and_len;
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
                new_value: quote! { <#ty as ::bitpiece::BitPiece>::to_bits(new_value) },
            });
            quote! {
                #vis fn #set_ident (&mut self, new_value: #ty) {
                    self.storage = #modified_value_expr;
                }
            }
        })
}

fn field_mut_fns<'a>(
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
}

/// parameters for generating an implementation of the `BitPiece` trait.
struct BitPieceGenImplParams {
    /// the identifier of the type for which the trait is to be implemented.
    type_ident: syn::Ident,

    /// the identifier of the type's mutable bit access type.
    ident_mut: syn::Ident,

    /// the bit length of the type.
    bit_len: BitLenExpr,

    /// the bits storage type of this type.
    storage_type: TypeExpr,

    /// code for serializing this type.
    /// this will be used as the body of the `to_bits` method.
    serialization_code: proc_macro2::TokenStream,

    /// code for deserializing this type.
    /// this will be used as the body of the `from_bits` method.
    deserialization_code: proc_macro2::TokenStream,
}

/// generates the final implementation of the `BitPiece` trait given the implementation details.
fn bitpiece_gen_impl(params: BitPieceGenImplParams) -> proc_macro2::TokenStream {
    let BitPieceGenImplParams {
        type_ident,
        ident_mut,
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
            type Mut<'s, S: BitStorage + 's> = #ident_mut<'s, S>;
            fn from_bits(bits: Self::Bits) -> Self {
                #deserialization_code
            }
            fn to_bits(self) -> Self::Bits {
                #serialization_code
            }
        }
    }
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