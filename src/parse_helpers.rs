use crate::PunctuatedParser;
use proc_macro2::{Span, TokenTree};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    Ident, Path, Token,
};
// TODO: check if we can remove the public access modifiers.
#[derive(Debug, Clone)]
pub struct FieldModifier {
    pub mapper_options: Vec<MapperOptions>,
}
#[derive(Debug, Clone)]
pub struct UseFns {
    pub key: Ident,
    pub use_fns: Punctuated<Path, Comma>,
}
#[derive(Debug, Clone)]
pub struct UseFields {
    pub key: Ident,
    pub use_fields: Punctuated<Ident, Comma>,
}

#[derive(Debug, Clone)]
pub enum MapperOptions {
    UseFns(UseFns),
    UseFields(UseFields),
}

// TODO: Refactor.
impl Parse for MapperOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key.to_string() == "use_fields" {
            input.parse::<Token!(=)>()?;
            let use_fields_tokens = input.parse::<TokenTree>()?;
            match use_fields_tokens {
                TokenTree::Group(g) => {
                    let paths: PunctuatedParser<Ident, Comma> = parse2(g.stream())?;
                    let use_fields = UseFields {
                        key,
                        use_fields: paths.punct,
                    };
                    Ok(MapperOptions::UseFields(use_fields))
                }
                _ => Err(syn::Error::new(Span::call_site(), "Expected a group")),
            }
        } else if key.to_string() == "use_fns" {
            input.parse::<Token!(=)>()?;
            let use_fields_tokens = input.parse::<TokenTree>()?;
            match use_fields_tokens {
                TokenTree::Group(g) => {
                    let paths: PunctuatedParser<Path, Comma> = parse2(g.stream())?;
                    let use_fns = UseFns {
                        key,
                        use_fns: paths.punct,
                    };
                    Ok(MapperOptions::UseFns(use_fns))
                }
                _ => Err(syn::Error::new(Span::call_site(), "Expected a group")),
            }
        } else {
            return Err(syn::Error::new(
                Span::call_site(),
                "expected one of use_fields, use_fns",
            ));
        }
    }
}

impl Parse for FieldModifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated_parser: PunctuatedParser<MapperOptions, Comma> = input.parse()?;
        let mapper_options: Vec<MapperOptions> = punctuated_parser.punct.into_iter().collect();
        // println!("fm: {:#?}", mapper_options);
        Ok(Self {
            mapper_options: mapper_options,
        })
    }
}
