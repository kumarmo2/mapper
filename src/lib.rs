#[macro_use]
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::quote;

use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident,
};

#[proc_macro_derive(Mapper, attributes(from))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let from_ident;
    let data: DataStruct;
    let named_fields: FieldsNamed;

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

    if let Some(ident) = get_from_ident(ast.attrs.iter()) {
        from_ident = ident;
    } else {
        panic!("problem with from attribute");
    }

    let struct_ident = ast.ident;

    let result_stream = quote! {
        impl From<#from_ident> for #struct_ident {
            fn from(source: #from_ident) -> Self {
                Self {
                    #(
                        #fields_init
                    )*
                }
            }
        }
    };

    result_stream.into()

    //println!("ast: {:#?}", ast);
    //TokenStream::new()
}

// TODO: Make this return with Result instead of Option.
// TODO: Refactor.
fn get_from_ident<'a, I>(mut attrs: I) -> Option<Ident>
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

    let from_idents: Vec<Ident>;
    match from_type_token {
        TokenTree2::Group(g) => {
            from_idents = g
                .stream()
                .into_iter()
                .filter(|tt| match tt {
                    TokenTree2::Ident(_) => true,
                    _ => false,
                })
                .map(|tt| {
                    match tt {
                        TokenTree2::Ident(ident) => ident,
                        _ => {
                            //TODO: refactor this.
                            panic!("Will not be executed ever.");
                        }
                    }
                })
                .collect();
        }
        _ => {
            return None;
        }
    }

    from_idents.first().and_then(|ident| Some(ident.clone()))
}
