use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, DeriveInput, Expr, Fields, FieldsNamed,
    Generics,
};

/// an attribute for defining bitfield structs.
#[proc_macro_attribute]
pub fn bitpiece(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    impl_bitpiece(args, input)
}

fn not_supported_err_span(what: &str, span: proc_macro2::Span) -> proc_macro::TokenStream {
    quote_spanned! {
        span => compile_error!(concat!(#what, " are not supported"));
    }
    .into()
}

fn not_supported_err(what: &str) -> proc_macro::TokenStream {
    not_supported_err_span(what, proc_macro2::Span::call_site())
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
            syn::Fields::Named(fields) => bitpiece_named_struct(
                &input,
                &fields,
                BitOrderExpr(quote! { ::bitpiece::BitOrder::LsbFirst }),
            ),
            syn::Fields::Unnamed(_) => not_supported_err("unnamed structs"),
            syn::Fields::Unit => not_supported_err("empty structs"),
        },
        syn::Data::Enum(_) => not_supported_err("enums"),
        syn::Data::Union(_) => not_supported_err("unions"),
    }
}

fn are_generics_empty(generics: &Generics) -> bool {
    generics.lt_token.is_none()
        && generics.params.is_empty()
        && generics.gt_token.is_none()
        && generics.where_clause.is_none()
}

/// returns an iterator over the extracted bits of each field.
fn named_struct_fields_extracted_bits<'a, I: Iterator<Item = &'a syn::Field> + 'a>(
    fields: I,
    bit_order: &'a BitOrderExpr,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    fields_offsets_and_lens(fields).map(|offset_and_len| {
        let FieldOffsetAndLen { len, offset } = offset_and_len;
        extract_bits(ExtractBitsParams {
            value: quote! { self.storage },
            value_len: TypeExpr::self_type().bit_len(),
            value_type: storage_type.clone(),
            extract_offset: offset,
            extract_len: len,
            bit_order: bit_order.clone(),
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
    /// the bit length of the value to extract the bits from
    value_len: BitLenExpr,
    /// the type of the value to extract the bits from
    value_type: TypeExpr,
    /// the offset at which to start extracting
    extract_offset: BitOffsetExpr,
    /// the amount of bits to extract
    extract_len: BitLenExpr,
    /// the bit order to use when extracting the bits
    bit_order: BitOrderExpr,
}
impl ExtractBitsParams {
    pub fn mask(&self) -> proc_macro2::TokenStream {
        let Self {
            value_type,
            extract_len,
            ..
        } = self;
        quote! {
            ((1 as #value_type) << (#extract_len)).saturating_sub(1)
        }
    }
    pub fn shifted_mask(&self) -> proc_macro2::TokenStream {
        let mask = self.mask();
        let shift_amount = self.lowest_bit_index();
        quote! {
            (#mask) << (#shift_amount)
        }
    }

    /// the lowest bit index of the extracted bit range.
    /// this takes into account the bit order.
    pub fn lowest_bit_index(&self) -> proc_macro2::TokenStream {
        let Self {
            value_len,
            extract_offset,
            extract_len,
            bit_order,
            ..
        } = self;
        quote! {
            {
                let bit_order: ::bitpiece::BitOrder = (#bit_order);
                match bit_order {
                    ::bitpiece::BitOrder::LsbFirst => {
                        #extract_offset
                    },
                    ::bitpiece::BitOrder::MsbFirst => {
                        (#value_len) - (#extract_offset) - (#extract_len)
                    },
                }
            }
        }
    }
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
    let mask = params.mask();
    let shift_right_amount = params.lowest_bit_index();
    let value = &params.value;
    quote! {
        {
            ((#value) >> (#shift_right_amount)) & (#mask)
        }
    }
}

/// returns an expression for the provided value with the specified bit range modified to its new value.
fn modify_bits(params: ModifyBitsParams) -> proc_macro2::TokenStream {
    let ModifyBitsParams {
        extract_params,
        new_value,
    } = params;
    let shifted_mask = extract_params.shifted_mask();
    let shift_amount = extract_params.lowest_bit_index();
    let value = &extract_params.value;
    quote! {
        {
            let without_original_bits = (#value) & (!(#shifted_mask));
            let shifted_new_value = (#new_value) << (#shift_amount);
            without_original_bits | shifted_new_value
        }
    }
}

fn bitpiece_named_struct_field_access_fns<'a>(
    fields: &'a FieldsNamed,
    bit_order: &'a BitOrderExpr,
    storage_type: &'a TypeExpr,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    named_struct_fields_extracted_bits(fields.named.iter(), bit_order, storage_type)
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

fn bitpiece_named_struct_field_set_fns<'a>(
    fields: &'a FieldsNamed,
    bit_order: &'a BitOrderExpr,
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
                    value_len: TypeExpr::self_type().bit_len(),
                    value_type: storage_type.clone(),
                    extract_offset: offset,
                    extract_len: len,
                    bit_order: bit_order.clone(),
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

/// information about the offset and len of a field.
struct FieldOffsetAndLen {
    len: BitLenExpr,
    offset: BitOffsetExpr,
}

fn bitpiece_named_struct(
    input: &DeriveInput,
    fields: &FieldsNamed,
    bit_order: BitOrderExpr,
) -> proc_macro::TokenStream {
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

    let implementation = bitpiece_gen_impl(BitPieceGenImplParams {
        type_ident: input.ident.clone(),
        bit_len: total_bit_length,
        storage_type: storage_type.clone(),
        serialization_code: quote! { self.storage },
        deserialization_code: quote! { Self { storage: bits } },
    });

    let field_access_fns =
        bitpiece_named_struct_field_access_fns(fields, &bit_order, &storage_type);
    let field_set_fns = bitpiece_named_struct_field_set_fns(fields, &bit_order, &storage_type);

    let vis = &input.vis;
    let ident = &input.ident;
    let attrs = &input.attrs;
    quote! {
        #(#attrs)*
        #vis struct #ident {
            storage: #storage_type,
        }
        #implementation
        impl #ident {
            #(#field_access_fns)*
            #(#field_set_fns)*
        }
    }
    .into()
}

/// parameters for generating an implementation of the `BitPiece` trait.
struct BitPieceGenImplParams {
    /// the identifier of the type for which the trait is to be implemented.
    type_ident: syn::Ident,

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
            fn from_bits(bits: Self::Bits) -> Self {
                #deserialization_code
            }
            fn to_bits(self) -> Self::Bits {
                #serialization_code
            }
        }
    }
}
/// implements the `ToTokens` trait for a newtype which is just a wrapper something else which implements `ToTokens`.
macro_rules! impl_to_tokens_for_newtype {
    {$t: ty} => {
        impl quote::ToTokens for $t {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                self.0.to_tokens(tokens)
            }
        }
    };
}

/// an expression representing a type.
#[derive(Clone)]
struct TypeExpr(proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {TypeExpr}
impl TypeExpr {
    /// returns a type expression for the `Self` type.
    fn self_type() -> Self {
        Self(quote! {
            Self
        })
    }

    /// creates a new type expression from the given type value.
    fn from_type(ty: &syn::Type) -> Self {
        Self(quote! {
            #ty
        })
    }

    /// returns the bit length of this type.
    /// this is only valid if the type implements the `BitPiece` trait.
    fn bit_len(&self) -> BitLenExpr {
        BitLenExpr(quote! {
            <#self as ::bitpiece::BitPiece>::BITS
        })
    }
}

/// an expression for the serialized size of some type.
struct BitLenExpr(proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {BitLenExpr}
impl BitLenExpr {
    /// returns a serialized size expression for a size of zero
    fn zero() -> Self {
        Self(quote! {0})
    }

    /// returns the smallest storage type needed to store a value with this bit length.
    fn storage_type(&self) -> TypeExpr {
        TypeExpr(quote! {
            <::bitpiece::BitLength<{ #self }> as ::bitpiece::AssociatedStorage>::Storage
        })
    }
}
impl core::ops::Add for BitLenExpr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(quote! {
            (#self) + (#rhs)
        })
    }
}
impl<'a> core::ops::Add for &'a BitLenExpr {
    type Output = BitLenExpr;

    fn add(self, rhs: Self) -> Self::Output {
        BitLenExpr(quote! {
            (#self) + (#rhs)
        })
    }
}
impl std::iter::Sum for BitLenExpr {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_else(Self::zero)
    }
}

/// an expression for a bit offset inside a bitfield.
struct BitOffsetExpr(proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {BitOffsetExpr}

/// an expression for the bit order of a bitfield.
#[derive(Clone)]
struct BitOrderExpr(proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {BitOrderExpr}
