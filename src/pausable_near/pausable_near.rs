use proc_macro::TokenStream;

use quote::quote;
use syn::Ident;

pub fn pausable(item_ident: Ident) -> (TokenStream, TokenStream) {
    let core_ts = quote! {


        pub trait Pausable {

        fn toggle_pause(&mut self);
        fn pause(&mut self);
        fn unpause(&mut self);
        fn when_not_paused(&self);

    }



        impl Pausable for #item_ident {

            fn toggle_pause(&mut self) {

                if !self.pause_status {
                    self.pause()
                } else {
                    self.unpause()
                }

            }

            fn pause(&mut self) {

                    self.pause_status = true;
                    near_sdk::env::log(b"The system is paused")

            }

            fn unpause(&mut self) {

                    self.pause_status = false;
                    near_sdk::env::log(b"The system is unpaused")

            }

            fn when_not_paused(&self) {
                if self.pause_status {
                    near_sdk::env::panic(b"Function is paused")
                }
            }


        }


        };

    (
        quote! {#core_ts}.into(),
        quote! {pause_status: bool,}.into(),
    )
}
