use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree as TokenTree2};
use quote::quote;

use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Attribute, Data, DeriveInput, Error, Fields, Path, Result,
};

#[derive(Debug)]
struct PunctuatedParser<T, P>
where
    T: Parse,
    P: Parse,
{
    punct: Punctuated<T, P>,
}

impl<T, P> Parse for PunctuatedParser<T, P>
where
    T: Parse,
    P: Parse,
{
    fn parse(input: ParseStream) -> Result<Self> {
        Punctuated::parse_terminated(input).map(|p| PunctuatedParser { punct: p })
    }
}

#[proc_macro_derive(Mapper, attributes(from))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let data = match ast.data {
        Data::Struct(d) => d,
        _ => {
            panic!("Currently only supports Structs");
        }
    };

    let named_fields = match data.fields {
        Fields::Named(f) => f,
        _ => panic!("only named fields are required"),
    };

    let fields_init: Vec<_> = named_fields
        .named
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref();
            quote! {
                #ident: source.#ident,
            }
        })
        .collect();

    let from_types = match get_from_types(ast.attrs.iter()) {
        Ok(f) => f,
        Err(reason) => return reason.to_compile_error().into(),
    };

    let struct_ident = ast.ident;

    let from_definitions = from_types.iter().map(|ty| {
        quote! {
            impl From<#ty> for #struct_ident {
                fn from(source: #ty) -> Self {
                    Self {
                        #(
                            #fields_init
                        )*
                    }
                }
            }
        }
    });

    let result_stream = quote! {
        #(
            #from_definitions
        )*
    };
    result_stream.into()
}

fn get_from_types<'a, I>(mut attrs: I) -> Result<Punctuated<Path, Comma>>
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
        TokenTree2::Group(g) => {
            let wrapper: Result<PunctuatedParser<Path, Comma>> = parse2(g.stream());
            wrapper
                .map(|r| r.punct)
                .map_err(|_| Error::new(Span::call_site(), "could not parse the from attribute"))
        }
        _ => Err(Error::new(Span::call_site(), "Invalid from attribute")),
    }
}
