use quote::quote_spanned;
use syn::Generics;

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
