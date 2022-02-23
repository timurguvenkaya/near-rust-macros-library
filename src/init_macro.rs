use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::collections::LinkedList;

use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, Lit};

pub fn init_macro(input: TokenStream) -> TokenStream {
    let array = parse_macro_input!(input as syn::ExprArray);

    let macro_list = LinkedList::from(["access", "pausable"]);

    let mut literals = Vec::new();
    let mut struct_field_tk = TokenStream2::new();

    if array.elems.is_empty() {
        return quote! {compile_error!("Array cannot be empty");}.into();
    }

    if let Some(Expr::Struct(s)) = array.elems.last() {
        if s.fields.is_empty() {
            return quote! {compile_error!("Struct cannot be empty")}.into();
        }

        for field in s.fields.iter() {
            let mut field_tk = quote! {#field};
            field_tk.extend(quote! {,});

            struct_field_tk.extend(field_tk);
        }
    } else {
        return quote! {compile_error!("The last element has to be struct")}.into();
    }

    for element in array.elems.iter() {
        match element {
            Expr::Lit(l) => literals.push(ExprLit::from(l.clone())),
            Expr::Struct(_s) => continue,
            _ => return quote! {compile_error!("Please include only literals")}.into(),
        };
    }

    let mut new_fields_tk = TokenStream2::new();

    for literal in literals.iter() {
        if let Lit::Str(l) = literal.lit.clone() {
            if !macro_list.contains(&&*l.value()) {
                return quote! {compile_error!("Macro does not exist")}.into();
            }

            if l.value().is_empty() {
                return quote! {compile_error!("Value cannot be empty")}.into();
            }

            if l.value() == "pausable" {
                let new_fields = quote! {pause_status: false,};

                new_fields_tk.extend(new_fields);
            }

            if l.value() == "access" {
                let new_fields = quote! {access_control_roles: near_sdk::collections::UnorderedMap::new(StorageKeyAccessControl::Roles.into_bytes()),};

                new_fields_tk.extend(new_fields);
            }
        } else {
            return quote! {compile_error!("Only &str type is accepted")}.into();
        }
    }

    quote! {
        Self {
            #struct_field_tk
            #new_fields_tk
        }
    }
    .into()
}
