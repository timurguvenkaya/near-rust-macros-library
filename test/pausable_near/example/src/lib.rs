use near_macros::{init_macro, require};
use near_sdk::borsh;
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault};

setup_alloc!();

#[near_bindgen]
#[derive(PanicOnDefault)]
#[require(Pausable)]
pub struct StatusMessage {
    data: String,
}

#[near_bindgen]
impl StatusMessage {
    #[init]
    pub fn new(data: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let constructor = init_macro!(["pausable", Self { data: data }]);

        constructor
    }

    pub fn get_data(&self) -> String {
        self.when_not_paused();
        self.data.clone()
    }

    pub fn pub_toggle_pause(&mut self) {
        self.toggle_pause()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    #[should_panic(expected = r#"Function is paused"#)]
    fn should_pause() {
        let context = get_context(String::from("timurguvenkaya.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::new(String::from("SOME DATA"));

        let got_data = contract.get_data();

        assert_eq!(got_data, String::from("SOME DATA"));

        contract.pub_toggle_pause();

        assert_eq!(contract.pause_status, true);

        contract.get_data();
    }

    #[test]
    fn should_unpause() {
        let context = get_context(String::from("timurguvenkaya.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::new(String::from("SOME DATA"));

        let got_data = contract.get_data();

        assert_eq!(got_data, String::from("SOME DATA"));

        contract.pub_toggle_pause();

        assert_eq!(contract.pause_status, true);

        contract.pub_toggle_pause();

        assert_eq!(contract.pause_status, false);

        assert_eq!(contract.get_data(), String::from("SOME DATA"));
    }
}
