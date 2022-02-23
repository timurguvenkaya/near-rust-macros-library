use near_sdk::borsh;
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_macros::{require, init_macro};

#[near_bindgen]
#[derive(PanicOnDefault)]
#[require(Pausable, Access)]
pub struct StatusMessage {
    data: String,
}

#[near_bindgen]
impl StatusMessage {
    #[init]
    pub fn new() -> Self {

        let constructor = init_macro!(["access", "pausable", Self {
            data: String::from("SOME DATA")
        }]);

        constructor
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
