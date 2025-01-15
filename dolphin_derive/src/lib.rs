use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

#[proc_macro_derive(Entity, attributes(EntityName, DbType))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut entity_name = struct_name.to_string(); // Default to struct name
    let mut db_type_string = "SQL".to_string(); // Default to SQL

    // Parse the attributes for custom values
    for attr in input.attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let Meta::NameValue(nv) = meta {
                if nv.path.is_ident("EntityName") {
                    if let Lit::Str(ref lit) = nv.lit {
                        entity_name = lit.value();
                    }
                }
                if nv.path.is_ident("DbType") {
                    if let Lit::Str(ref lit) = nv.lit {
                        db_type_string = lit.value();
                    }
                }
            }
        }
    }

    // Generate the Entity trait implementation based on DbType
    let expanded = quote! {

     
        

        pub trait Entity {
            fn entity_name() -> &'static str;
            fn find_by_id(id: u32) -> String;
            fn find_by_id_with_backend(id: u32) -> String;
        }

        impl Entity for #struct_name {
            fn entity_name() -> &'static str {
                #entity_name
            }

            fn find_by_id(id: u32) -> String {
                format!("SELECT * FROM {} WHERE id = {}", Self::entity_name(), id)
            }

            fn find_by_id_with_backend(id: u32) -> String {

                let query =  get_db_strategy(DbBackend::from_str("SQL"));  
                return query.find();
            }



        }
    };

    TokenStream::from(expanded)
}
