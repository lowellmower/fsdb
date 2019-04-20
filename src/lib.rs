extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn apply_fsdb(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
    -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        use serde::{Serialize, Deserialize};
        #[derive(Serialize, Deserialize)]
        #input
    };

    output.into()
}

#[proc_macro_derive(JSONable)]
pub fn _v0_1_0_jsonable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let identity = &input.ident;

    let output = quote::quote! {
        impl #identity {
            fn fsdb_json(&self) -> String {
                serde_json::to_string_pretty(&self).unwrap().to_string()
            }
        }
    };

    output.into()
}
