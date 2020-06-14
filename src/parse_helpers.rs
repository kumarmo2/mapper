use crate::PunctuatedParser;
use proc_macro2::{Span, TokenTree};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    Ident, Token,
};
// TODO: check if we can remove the public access modifiers.
#[derive(Debug, Clone)]
pub struct FieldModifier {
    pub use_fields: Punctuated<Ident, Comma>,
}

impl Parse for FieldModifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let use_field_key: Ident = input.parse()?;
        if use_field_key.to_string() != "use_fields" {
            return Err(syn::Error::new(
                Span::call_site(),
                "expected use_field key in the mapper attribute",
            ));
        }
        input.parse::<Token!(=)>()?;
        let use_fields_tokens = input.parse::<TokenTree>()?;
        match use_fields_tokens {
            TokenTree::Group(g) => {
                let paths: PunctuatedParser<Ident, Comma> = parse2(g.stream())?;
                Ok(Self {
                    use_fields: paths.punct,
                })
            }
            _ => Err(syn::Error::new(Span::call_site(), "Expected a group")),
        }
    }
}
