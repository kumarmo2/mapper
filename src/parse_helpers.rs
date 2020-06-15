use proc_macro2::{Span, TokenTree};
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    Attribute, Error, FieldsNamed, Ident, Path, Result, Token,
};

#[derive(Debug)]
pub struct PunctuatedParser<T, P>
where
    T: Parse,
    P: Parse,
{
    pub punct: Punctuated<T, P>,
}

impl<T, P> Parse for PunctuatedParser<T, P>
where
    T: Parse,
    P: Parse,
{
    fn parse(input: ParseStream) -> Result<Self> {
        match Punctuated::parse_terminated(input) {
            Ok(r) => Ok(PunctuatedParser { punct: r }),
            Err(reason) => Err(reason),
        }
    }
}
// TODO: check if we can remove the public access modifiers.
#[derive(Debug)]
pub struct FieldModifier {
    pub mapper_options: Vec<MapperOptions>,
}
#[derive(Debug)]
pub struct UseFns {
    pub key: Ident,
    pub use_fns: Punctuated<Path, Comma>,
}
#[derive(Debug)]
pub struct UseFields {
    pub key: Ident,
    pub use_fields: Punctuated<Ident, Comma>,
}

#[derive(Debug)]
pub enum MapperOptions {
    UseFns(UseFns),
    UseFields(UseFields),
}

// TODO: Refactor.
impl Parse for MapperOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token!(=)>()?;
        if key.to_string() == "use_fields" {
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
            let use_fns_tokens = input.parse::<TokenTree>()?;
            match use_fns_tokens {
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
        Ok(Self {
            mapper_options: mapper_options,
        })
    }
}

pub fn get_from_types<'a, I>(mut attrs: I) -> Result<Punctuated<Path, Comma>>
where
    I: Iterator<Item = &'a Attribute>,
{
    // Get the first "From" Type Attribute
    let attr = match attrs.find(|attr| {
        attr.path
            .segments
            .iter()
            .find(|seg| seg.ident.to_string() == "from")
            .map_or(false, |_| true)
    }) {
        Some(a) => a,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "from attribute is compulsory",
            ))
        }
    };

    let mut tokens = attr.tokens.clone().into_iter();
    let from_type_token = if let Some(t) = tokens.nth(0) {
        t
    } else {
        return Err(Error::new(
            Span::call_site(),
            "No valid from attribute found",
        ));
    };

    match from_type_token {
        TokenTree::Group(g) => {
            let wrapper: Result<PunctuatedParser<Path, Comma>> = parse2(g.stream());
            wrapper
                .map(|r| r.punct)
                .map_err(|_| Error::new(Span::call_site(), "could not parse the from attribute"))
        }
        _ => Err(Error::new(Span::call_site(), "Invalid from attribute")),
    }
}

pub fn get_field_to_field_modifier_map(
    named_fields: &FieldsNamed,
) -> HashMap<Ident, FieldModifier> {
    let map: HashMap<Ident, FieldModifier> = named_fields
        .named
        .iter()
        .filter_map(|f| {
            f.attrs
                .iter()
                .find(|attr| {
                    attr.path
                        .segments
                        .iter()
                        .nth(0)
                        .map_or(false, |segment| segment.ident.to_string() == "mapper")
                })
                .and_then(|mapper_attr| {
                    match mapper_attr.parse_args::<FieldModifier>() {
                        Ok(fm) => Some((f.ident.as_ref().unwrap().clone(), fm)), //returning a tuple so that it can be collected into a HashMap.
                        Err(_) => panic!("error parsing the mapper attribute"),
                    }
                })
        })
        .collect();
    map
}
