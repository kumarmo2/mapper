mod parse_helpers;

use parse_helpers::{
    get_field_to_field_modifier_map, get_from_types, FieldModifier, MapperOptions,
};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;

use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Mapper, attributes(from, mapper))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let generics = ast.generics;

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

    let map: HashMap<Ident, FieldModifier> = get_field_to_field_modifier_map(&named_fields);

    let from_types = match get_from_types(ast.attrs.iter()) {
        Ok(f) => f,
        Err(reason) => return reason.to_compile_error().into(),
    };

    let struct_ident = ast.ident;

    let from_definitions = from_types.iter().enumerate().map(|(from_type_index, ty)| {
        let fields_init: Vec<_> = named_fields
            .named
            .iter()
            .map(|f| {
                let field_modifier = map.get(f.ident.as_ref().unwrap());
                let dest_field_ident = f.ident.as_ref();

                if let None = field_modifier {
                    let ident = f.ident.as_ref();
                    quote! {
                        #dest_field_ident: source.#ident,
                    }
                } else {
                    let fm = field_modifier.unwrap();

                    let use_fields = fm.mapper_options.iter().find(|mo| match mo {
                        MapperOptions::UseFields(_) => true,
                        _ => false,
                    });

                    let use_fns = fm.mapper_options.iter().find(|mo| match mo {
                        MapperOptions::UseFns(_) => true,
                        _ => false,
                    });

                    let source_field_ident = use_fields.map_or_else(
                        || Some(f.ident.as_ref().unwrap().clone()),
                        |mo| {
                            if let MapperOptions::UseFields(uf) = mo {
                                uf.use_fields.iter().nth(from_type_index).map_or_else(
                                    || Some(f.ident.as_ref().unwrap().clone()),
                                    |ident| Some(ident.clone()),
                                )
                            } else {
                                panic!("cannot reach here");
                            }
                        },
                    );
                    let dest_field_ident = f.ident.as_ref();
                    match use_fns {
                        None => {
                            quote! {
                                #dest_field_ident: source.#source_field_ident,
                            }
                        }
                        Some(uf) => {
                            if let MapperOptions::UseFns(uf) = uf {
                                uf.use_fns.iter().nth(from_type_index).map_or_else(
                                    || {
                                        quote! {
                                            #dest_field_ident: source.#source_field_ident,
                                        }
                                    },
                                    |path| {
                                        quote! {
                                            #dest_field_ident: #path(source.#source_field_ident),
                                        }
                                    },
                                )
                            } else {
                                panic!("cannot reach here");
                            }
                        }
                    }
                }
            })
            .collect();
        quote! {
            impl#generics From<#ty> for #struct_ident#generics {
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
