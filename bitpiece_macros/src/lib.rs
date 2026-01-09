mod enums;
mod named_structs;
mod newtypes;
mod utils;

use std::{collections::HashSet, str::FromStr};

use enum_all_values_const::AllValues;
use enums::bitpiece_enum;
use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use named_structs::bitpiece_named_struct;
use strum::{EnumString, VariantNames};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    DeriveInput, LitInt,
};
use utils::{are_generics_empty, not_supported_err};

/// an attribute for defining bitfields.
#[proc_macro_attribute]
pub fn bitpiece(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    impl_bitpiece(args, input)
}

#[derive(EnumString, VariantNames, AllValues, Hash, Clone, Copy, Debug, PartialEq, Eq)]
enum OptIn {
    Get,
    Set,
    GetNoshift,
    GetMut,
    ConstEq,
    FieldsStruct,
    MutStruct,
    MutStructFieldGet,
    MutStructFieldSet,
    MutStructFieldGetNoshift,
    MutStructFieldMut,
}

#[derive(EnumString, VariantNames, Hash, Clone, Copy, Debug, PartialEq, Eq)]
enum OptInPreset {
    Basic,
    All,
    MutStructAll,
}
impl OptInPreset {
    fn opt_ins(&self) -> &'static [OptIn] {
        match self {
            OptInPreset::Basic => &[OptIn::Get, OptIn::Set],
            OptInPreset::All => OptIn::ALL_VALUES.as_slice(),
            OptInPreset::MutStructAll => &[
                OptIn::MutStruct,
                OptIn::MutStructFieldGet,
                OptIn::MutStructFieldSet,
                OptIn::MutStructFieldGetNoshift,
                OptIn::MutStructFieldMut,
            ],
        }
    }
}

struct ExplicitBitLengthArg {
    bit_length: usize,
    lit: LitInt,
}

struct OptInArg {
    opt_in: OptIn,
    ident: syn::Ident,
}
struct OptInPresetArg {
    opt_in_preset: OptInPreset,
    ident: syn::Ident,
}

enum MacroArg {
    ExplicitBitLength(ExplicitBitLengthArg),
    OptIn(OptInArg),
    OptInPreset(OptInPresetArg),
}
impl Parse for MacroArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let opt_in_names: String = OptIn::VARIANTS
            .iter()
            .map(|v| format!("`{}`", v.to_snake_case()))
            .join(", ");
        let preset_names: String = OptInPreset::VARIANTS
            .iter()
            .map(|v| format!("`{}`", v.to_snake_case()))
            .join(", ");
        let unknown_macro_arg_err = format!(
            "unknown macro argument, expected an integer bit-length (e.g. `32`), an opt-in flag ({opt_in_names}), or an opt-in preset ({preset_names})"
        );

        // explicit bit length
        if input.peek(LitInt) {
            let lit: LitInt = input.parse()?;
            return Ok(MacroArg::ExplicitBitLength(ExplicitBitLengthArg {
                bit_length: lit.base10_parse()?,
                lit,
            }));
        }

        // opt ins as identifiers
        if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse()?;

            let ident_pascal_case = ident.to_string().to_upper_camel_case();

            if let Ok(opt_in) = OptIn::from_str(&ident_pascal_case) {
                return Ok(MacroArg::OptIn(OptInArg { opt_in, ident }));
            } else if let Ok(opt_in_preset) = OptInPreset::from_str(&ident_pascal_case) {
                return Ok(MacroArg::OptInPreset(OptInPresetArg {
                    opt_in_preset,
                    ident,
                }));
            } else {
                return Err(syn::Error::new_spanned(&ident, unknown_macro_arg_err));
            }
        }

        Err(input.error(unknown_macro_arg_err))
    }
}

struct RawMacroArgs(Punctuated<MacroArg, Comma>);
impl Parse for RawMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Punctuated::<MacroArg, Comma>::parse_terminated(input).map(Self)
    }
}

struct OptInArgsCollector(Vec<OptInArg>);
impl OptInArgsCollector {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn add_opt_in(&mut self, arg: OptInArg) -> Result<(), syn::Error> {
        if let Some(existing_arg) = self
            .0
            .iter()
            .find(|existing_arg| existing_arg.opt_in == arg.opt_in)
        {
            let mut err = syn::Error::new_spanned(arg.ident, "duplicate opt in arg");
            err.combine(syn::Error::new_spanned(
                existing_arg.ident.clone(),
                "conflicts with this previous opt in arg",
            ));
            return Err(err);
        }
        Ok(self.0.push(arg))
    }
}

#[derive(Default)]
struct MacroArgs {
    explicit_bit_length: Option<usize>,
    opt_ins: HashSet<OptIn>,
}
impl MacroArgs {
    pub fn filter_opt_in_code(
        &self,
        opt_in: OptIn,
        code: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        if self.opt_ins.contains(&opt_in) {
            code
        } else {
            quote::quote! {}
        }
    }
}
impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let raw_args: RawMacroArgs = input.parse()?;

        let mut explicit_bit_length_arg: Option<ExplicitBitLengthArg> = None;
        let mut opt_in_args = OptInArgsCollector::new();
        for arg in raw_args.0 {
            match arg {
                MacroArg::ExplicitBitLength(arg) => {
                    if let Some(existing_arg) = explicit_bit_length_arg {
                        let mut err = syn::Error::new_spanned(
                            arg.lit,
                            "found more than one explicit bit length argument but only one is allowed",
                        );
                        err.combine(syn::Error::new_spanned(
                            existing_arg.lit,
                            "conflicts with this previous explicit bit length argument",
                        ));
                        return Err(err);
                    }
                    explicit_bit_length_arg = Some(arg);
                }
                MacroArg::OptIn(arg) => {
                    opt_in_args.add_opt_in(arg)?;
                }
                MacroArg::OptInPreset(opt_in_preset_arg) => {
                    for opt_in in opt_in_preset_arg.opt_in_preset.opt_ins() {
                        opt_in_args.add_opt_in(OptInArg {
                            opt_in: *opt_in,
                            ident: opt_in_preset_arg.ident.clone(),
                        })?;
                    }
                }
            }
        }
        Ok(MacroArgs {
            explicit_bit_length: explicit_bit_length_arg.map(|arg| arg.bit_length),
            opt_ins: opt_in_args.0.iter().map(|arg| arg.opt_in).collect(),
        })
    }
}

fn impl_bitpiece(
    args_tokens: proc_macro::TokenStream,
    input_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_args = parse_macro_input!(args_tokens as MacroArgs);
    let input = parse_macro_input!(input_tokens as DeriveInput);

    if !are_generics_empty(&input.generics) {
        return not_supported_err("generics");
    }

    match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => bitpiece_named_struct(&input, fields, macro_args),
            syn::Fields::Unnamed(_) => not_supported_err("unnamed structs"),
            syn::Fields::Unit => not_supported_err("empty structs"),
        },
        syn::Data::Enum(data_enum) => bitpiece_enum(&input, data_enum, macro_args),
        syn::Data::Union(_) => not_supported_err("unions"),
    }
}
