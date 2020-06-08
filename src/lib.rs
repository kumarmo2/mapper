use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::quote;

use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Attribute, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Path, Result,
};

#[derive(Debug)]
struct PuctuatedParser<T, P>
where
    T: Parse,
    P: Parse,
{
    punct: Punctuated<T, P>,
}

impl<T, P> Parse for PuctuatedParser<T, P>
where
    T: Parse,
    P: Parse,
{
    fn parse(input: ParseStream) -> Result<Self> {
        let result: Result<Punctuated<T, P>> = Punctuated::parse_terminated(input);
        match result {
            Ok(p) => Ok(PuctuatedParser { punct: p }),
            Err(_) => panic!("some error"),
        }
    }
}

#[proc_macro_derive(Mapper, attributes(from))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let data: DataStruct;
    let named_fields: FieldsNamed;

    // println!("ast: {:#?}", ast);

    match ast.data {
        Data::Struct(d) => {
            data = d;
        }
        _ => {
            panic!("Currently only supports Structs");
        }
    }

    match data.fields {
        Fields::Named(f) => named_fields = f,
        _ => panic!("only named fields are required"),
    }

    let fields_init: Vec<TokenStream2> = named_fields
        .named
        .clone()
        .into_iter()
        .map(|f| {
            let ident = f.ident;
            quote! {
                #ident: source.#ident,
            }
        })
        .collect();

    let from_types;
    if let Some(f) = get_from_types(ast.attrs.iter()) {
        from_types = f;
    } else {
        panic!("error parsing the from types");
    }

    // println!("from_types: {:#?}", from_types);

    let struct_ident = ast.ident;

    let from_definitions2 = from_types.iter().map(|ty| {
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
            #from_definitions2
        )*
    };
    result_stream.into()
}

// TODO: Make this return with Result instead of Option.
fn get_from_types<'a, I>(mut attrs: I) -> Option<Punctuated<Path, Comma>>
where
    I: Iterator<Item = &'a Attribute>,
{
    // Get the first "From" Type Attribute
    let attr = attrs.find(|attr| {
        match attr
            .path
            .segments
            .iter()
            .find(|seg| seg.ident.to_string() == "from")
        {
            Some(_) => true,
            None => false,
        }
    })?;

    let mut tokens = attr.tokens.clone().into_iter();
    let from_type_token;
    if let Some(t) = tokens.nth(0) {
        from_type_token = t;
    } else {
        return None;
    }

    match from_type_token {
        TokenTree2::Group(g) => {
            let ts: TokenStream2 = g.stream();
            let wrapper: Result<PuctuatedParser<Path, Comma>> = parse2(ts);
            match wrapper {
                Ok(r) => {
                    return Some(r.punct);
                }
                Err(_) => {
                    return None;
                }
            }
        }
        _ => {
            return None;
        }
    }
}
