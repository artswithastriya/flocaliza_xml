use proc_macro2::TokenStream;
use syn::{
    punctuated::{self, Punctuated},
    Ident,
};

fn process(r#type: &syn::Type, tokens: &mut Vec<TokenStream>) -> syn::Result<syn::Type> {
    use std::borrow::Borrow;
    use syn::Type::*;
    match r#type {
        Array(r#type) => process(r#type.elem.as_ref(), tokens),
        Reference(r#type) => process(r#type.elem.as_ref(), tokens),
        Slice(r#type) => process(r#type.elem.as_ref(), tokens),

        Path(path) => {
            if path.path.segments.len() == 1 {
                let segment = path.path.segments.iter().nth(0).unwrap();
                if segment.ident.to_string() == "Localized" {
                    if let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments {
                        if arguments.args.len() > 1 {
                            return Err(syn::Error::new_spanned(
                                &arguments,
                                "You can only insert one type argument to Localized<T>",
                            ));
                        }
              
                            use syn::GenericArgument::*;
                            match  arguments.args.first().unwrap() {
                                Type(r#type) => Ok(Path(syn::TypePath {
                                    path: syn::Path {
                                        leading_colon: path.path.leading_colon,
                                        segments: {
                                            let mut punctuated = Punctuated::new();
                                            use syn::spanned::Spanned;
                                            punctuated.push(syn::PathSegment {
                                                ident: Ident::new(
                                                    &format!("{}", {
                                                        match r#type {
                                                       
                                                            Path(path) => path.path.segments.last().unwrap().ident.clone(),
                                                            _ => return Err(syn::Error::new_spanned(&r#type, "Can only use path in Localized<T> for now."))
                                                        }
                                                    }),
                                                    r#type.span(),
                                                ),
                                                arguments: {
                                                        match r#type {
                                                       
                                                            Path(path) => path.path.segments.last().unwrap().arguments.clone(),
                                                            _ => return Err(syn::Error::new_spanned(&r#type, "Can only use path in Localized<T> for now."))
                                                        }
                                                    },
                                            });

                                            punctuated
                                        },
                                    },
                                    qself: None,
                                })),
                                argument => return Err(syn::Error::new_spanned(&argument, "Can only accept Type argument in Localized<T>"))
                            }
                        
                        
                    } else {
                        unreachable!()
                    }
                } else {
                    Ok(r#type.clone())
                }
            } else {
                Ok(r#type.clone())
            }
        }

        Tuple(tuple) => {
            let mut elems = syn::punctuated::Punctuated::new();

            for item in &tuple.elems {
                use quote::ToTokens;
                elems.push(process(&item, tokens)?);
                
            }

            Ok(Tuple(syn::TypeTuple {
                paren_token: tuple.paren_token,
                elems,
            }))
        }
        _ => Ok(r#type.clone())
    }
}

pub fn input(input: syn::ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let mut input = input;

    let mut elements = Vec::new();
    for mut field in &mut input.fields {
        field.ty = process(&field.ty, &mut elements)?;
    }

    let structs = quote::quote! {
        #(#elements)*

        #input
    };
    
    Ok(structs)
}
