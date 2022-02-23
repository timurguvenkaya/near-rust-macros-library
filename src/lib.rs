extern crate proc_macro;

mod access_control_near;
mod init_macro;
mod pausable_near;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::collections::LinkedList;
use syn::parse::Parser;
use syn::{parse_macro_input, Ident};

use quote::quote;

use access_control_near::access_control_near::access_control;
use pausable_near::pausable_near::pausable;

#[proc_macro_attribute]
pub fn require(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::ItemStruct);

    let item_ident = &item.ident;
    let item_vis = &item.vis;
    let original_fields = &item.fields;

    let allowed_imports = LinkedList::from(["access", "pausable"]);

    let mut fields = TokenStream2::new();
    let mut gen = TokenStream::new();

    let arg_vec: Vec<&Ident> = Vec::new();

    for field in original_fields.iter() {
        let field_tk = quote! {#field,};
        fields.extend(field_tk)
    }

    let mut main_ts = TokenStream2::new();

    if args.is_empty() {
        return quote! {compile_error!("Please specify macros you want to import.");}.into();
    }

    let args_parsed =
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated.parse(args);

    if args_parsed.is_ok() {
        let punctuated_array = args_parsed.as_ref().unwrap();

        for arg in punctuated_array.iter() {
            let ident = &arg.segments.first().unwrap().ident;

            if !allowed_imports.contains(&&*ident.to_string().to_lowercase()) {
                return quote! {compile_error!("There is no such macro");}.into();
            }

            if arg_vec.contains(&&ident.clone()) {
                return quote! {compile_error!("Please do not enter duplicate macros");}.into();
            }

            if ident.to_string().to_lowercase() == "access" {
                let (access_control_ts, field_ts) = access_control(item_ident.clone());

                fields.extend(TokenStream2::from(field_ts));

                gen.extend(access_control_ts);
            }

            if ident.to_string().to_lowercase() == "pausable" {
                let (pausable_ts, field_ts) = pausable(item_ident.clone());

                fields.extend(TokenStream2::from(field_ts));

                gen.extend(pausable_ts);
            }
        }

        let main_struct = quote! {
            #[derive(near_sdk::borsh::BorshDeserialize, near_sdk::borsh::BorshSerialize)]
                    #item_vis struct #item_ident {
                     #fields
                 }
        };

        main_ts.extend(main_struct);
        main_ts.extend(TokenStream2::from(gen));
    } else {
        return quote! {compile_error!("Failed to parse");}.into();
    }

    main_ts.into()
}

#[proc_macro]
pub fn init_macro(input: TokenStream) -> TokenStream {
    init_macro::init_macro(input)
}
