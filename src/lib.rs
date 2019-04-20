extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn apply_fsdb(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
    -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        use serde::{Serialize, Deserialize};
        #[derive(Serialize, Deserialize)]
        #input
    };
    output.into()
}

#[proc_macro_derive(JSONable)]
pub fn _v0_1_0_jsonable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let identity = &input.ident;
    // let data = &input.data;

    let result = quote! {
        impl #identity {
            fn fsdb_json(&self) -> String {
                serde_json::to_string_pretty(&self).unwrap().to_string()
            }
        }
    };

    result.into()
}
