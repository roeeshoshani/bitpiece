use quote::quote;
use syn::{Type, TypePath};

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
pub struct TypeExpr(pub proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {TypeExpr}
impl TypeExpr {
    /// creates a new type expression from the given type value.
    pub fn from_type(ty: &syn::Type) -> Self {
        Self(quote! {
            #ty
        })
    }

    /// returns the bit length of this type.
    /// this is only valid if the type implements the `BitPiece` trait.
    pub fn bit_len(&self) -> BitLenExpr {
        BitLenExpr(quote! {
            <#self as ::bitpiece::BitPiece>::BITS
        })
    }

    /// returns if the field is of type SB##
    pub fn signed(&self) -> bool {
        let field_type = syn::parse2(self.0.clone());
        match field_type {
            Ok(Type::Path(TypePath {path, ..})) => {
                if let Some(segment) = path.segments.last() {
                    let type_name = segment.ident.to_string();
                    type_name.starts_with("SB") &&
                    type_name.chars().skip(2).all(|c| c.is_ascii_digit())
                } else {
                    false
                }
            }
            _ => false
        }
    }

    /// returns the fields type of this type.
    /// this is only valid if the type implements the `BitPiece` trait.
    pub fn fields_ty(&self) -> TypeExpr {
        TypeExpr(quote! {
            <#self as ::bitpiece::BitPiece>::Fields
        })
    }
}

/// an expression for the serialized size of some type.
#[derive(Clone)]
pub struct BitLenExpr(pub proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {BitLenExpr}
impl BitLenExpr {
    /// returns a serialized size expression for a size of zero
    pub fn zero() -> Self {
        Self(quote! {0})
    }

    /// returns the smallest storage type needed to store a value with this bit length.
    pub fn storage_type(&self) -> TypeExpr {
        TypeExpr(quote! {
            <::bitpiece::BitLength<{ #self }, false> as ::bitpiece::AssociatedStorage>::Storage
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
pub struct BitOffsetExpr(pub proc_macro2::TokenStream);
impl_to_tokens_for_newtype! {BitOffsetExpr}
