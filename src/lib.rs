#[macro_use]
use proc_macro::TokenStream;
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

    let from_tokens = &attr.tokens;

    /*
        if from_tokens.iter().len() != 1 {
            return None;
        }
    */

    None
}
