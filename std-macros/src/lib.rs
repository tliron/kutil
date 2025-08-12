// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Various Rust utilities to enhance the standard library.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/kutil).
*/

mod attributes;
mod derive_display;
mod derive_from_str;

// See: https://petanode.com/posts/rust-proc-macro/

/// Procedural macro for `#[derive(Display)]`.
#[proc_macro_derive(Display, attributes(display, strings))]
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = syn::parse_macro_input!(input);

    match input.data {
        syn::Data::Enum(_) => derive_display::Generator::generate(&mut input),

        _ => Err(syn::Error::new(input.ident.span(), "`Display`: not an enum")),
    }
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

/// Procedural macro for `#[derive(FromStr)]`.
#[proc_macro_derive(FromStr, attributes(from_str, strings))]
pub fn derive_from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = syn::parse_macro_input!(input);

    match input.data {
        syn::Data::Enum(_) => derive_from_str::Generator::generate(&mut input),

        _ => Err(syn::Error::new(input.ident.span(), "`FromStr`: not an enum")),
    }
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
