#[macro_use]
use proc_macro::TokenStream;
use proc_macro2::TokenTree as TokenTree2;

use syn::{parse_macro_input, Attribute, DeriveInput, Ident};

#[proc_macro_derive(Mapper, attributes(from))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    /*
        let from_ident;

        if let Some(ident) = get_from_ident(&ast.attrs) {
            from_ident = ident;
        } else {
            panic!("problem with from attribute");
        }
    */
    get_from_ident(&ast.attrs);

    println!("ast: {:#?}", ast);
    TokenStream::new()
}

// TODO: Make this return with Result instead of Option.
fn get_from_ident(attrs: &Vec<Attribute>) -> Option<Ident> {
    if attrs.len() < 1 {
        return None;
    }

    // Get the first "From" Type Attribute
    let attr = attrs.iter().find(|attr| {
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

    //if from_tokens.clone().into_iter().len() != 1 {
    //}

    None
}
