use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, Lit};

#[proc_macro_derive(Entity, attributes(EntityName))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut entity_name = struct_name.to_string(); // Default to struct name

    for attr in input.attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("EntityName") {
                    if let Lit::Str(lit) = nv.lit {
                        entity_name = lit.value();
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn table_name() -> &'static str {
                #entity_name
            }
        }
    };

    TokenStream::from(expanded)
}
