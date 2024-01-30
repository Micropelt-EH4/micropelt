use proc_macro::TokenStream;
use quote::quote;

const ATTRIBUTE_PATH: &str = "partial_close";

#[proc_macro_derive(PartialClose, attributes(partial_close))]
pub fn derive_partial_close(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(s) => derive_struct(&input.ident, s),
        syn::Data::Enum(e) => derive_enum(&input.ident, e),
        syn::Data::Union(_) => {
            unimplemented!("#derive[PartialClose] is not implemented for unions")
        }
    }
}

fn derive_struct(ident: &syn::Ident, s: &syn::DataStruct) -> TokenStream {
    let mut inner = quote!();

    match &s.fields {
        syn::Fields::Named(named) => {
            for (i, field) in named.named.iter().enumerate() {
                if i != 0 {
                    inner.extend(quote!(&& ))
                }

                let name = &field.ident;
                match (attributes_get_resoultion(&field.attrs), type_is_option(&field.ty)) {
                    (Some(resolution), true) => inner.extend(quote!(
                        (match (self.#name, other.#name) {
                            (Some(inner_s), Some(inner_o)) => (inner_s - inner_o).abs() < #resolution,
                            (Some(_), None) => false,
                            (None, Some(_)) => false,
                            (None, None) => true,
                        }
                    ))),
                    (Some(resolution), false) => inner.extend(quote!(
                        (self.#name - other.#name).abs() < #resolution)
                    ),
                    (None, _) => inner.extend(quote!(
                        self.#name == other.#name
                    ))
                }
            }
        }
        _ => unimplemented!(
            "#[derive(PartialClose)] is not implemented for structs with unnamed fields"
        ),
    }

    quote!(
        impl #ident {
            fn partial_close(&self, other: &Self) -> bool {
                #inner
            }
        }
    )
    .into()
}

fn derive_enum(ident: &syn::Ident, e: &syn::DataEnum) -> TokenStream {
    let mut inner = quote!();

    for variant in e.variants.iter() {
        let name = &variant.ident;
        match variant.fields {
            syn::Fields::Named(_) => unimplemented!(
                "#[derive(PartialClose)] is not implemented for enums with a named field"
            ),
            syn::Fields::Unnamed(_) => {
                if let Some(resolution) = attributes_get_resoultion(&variant.attrs) {
                    inner.extend(
                        quote!((Self::#name(s), Self::#name(o)) => (*s - *o).abs() < #resolution,),
                    )
                } else {
                    inner.extend(quote!((Self::#name(s), Self::#name(o)) => s.eq(o),))
                }
            }
            syn::Fields::Unit => inner.extend(quote!((Self::#name, Self::#name) => true,)),
        }
    }

    quote!(
        impl #ident {
            fn partial_close(&self, other: &Self) -> bool {
                match (self, other) {
                    #inner
                    _ => false,
                }
            }
        }
    )
    .into()
}

fn type_is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(path) = ty {
        if let Some(segment) = path.path.segments.first() {
            return segment.ident == "Option";
        }
    }

    false
}

fn attributes_get_resoultion(attributes: &Vec<syn::Attribute>) -> Option<f32> {
    for attr in attributes {
        if attr.path().is_ident(ATTRIBUTE_PATH) {
            let assign = attr.parse_args::<syn::ExprAssign>().unwrap();
            let resolution = *(assign.right);
            match resolution {
                syn::Expr::Lit(r) => match r.lit {
                    syn::Lit::Float(f) => return Some(f.base10_parse().unwrap()),
                    _ => unimplemented!("Expected a resolution as a float"),
                },
                _ => unimplemented!("Expected a resolution `(resoultion = x.y)`"),
            }
        }
    }
    None
}
