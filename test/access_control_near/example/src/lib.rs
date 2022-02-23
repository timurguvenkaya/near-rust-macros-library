use near_sdk::borsh;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

use near_macros::{init_macro, require};

near_sdk::setup_alloc!();

const DEFAULT_ADMIN: &str = "default_admin";
const MINTER: &str = "minter";
const MANAGER: &str = "manager";

#[near_bindgen]
#[derive(PanicOnDefault)]
#[require(Access, Pausable)]
pub struct StatusMessage {
    data: String,
}

#[near_bindgen]
impl StatusMessage {
    #[init]
    pub fn new(owner: AccountId, minter: AccountId, manager: AccountId, data: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let mut constructor = init_macro!(["access", "pausable", Self { data: data }]);

        constructor.setup_account_role(&DEFAULT_ADMIN.to_string(), &owner);
        constructor.setup_account_role(&MINTER.to_string(), &minter);
        constructor.setup_account_role(&MANAGER.to_string(), &manager);

        constructor
    }

    // CONSTRUCTOR FOR TESTING
    #[init]
    pub fn without_setup(data: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let constructor = init_macro!(["access", "pausable", Self { data: data }]);

        constructor
    }

    pub fn get_data(&self) -> String {
        self.when_not_paused();
        self.data.clone()
    }

    pub fn pub_toggle_pause(&mut self) {
        self.check_role(&DEFAULT_ADMIN.to_string(), &env::predecessor_account_id());
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
    fn setup_role_works() {
        let context = get_context(String::from("alice.testnet"), 0);
        testing_env!(context);

        let contract = StatusMessage::new(
            String::from("timurguvenkaya.testnet"),
            String::from("mike.testnet"),
            String::from("jane.testnet"),
            String::from("SOME DATA"),
        );

        let got_data = contract.get_data();

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timurguvenkaya.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        assert_eq!(got_data, String::from("SOME DATA"))
    }

    #[test]
    fn check_role_works_allowed() {
        let context = get_context(String::from("timur.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

        let role_vec = vec![
            ("timur.testnet".to_string(), "default_admin".to_string()),
            ("mike.testnet".to_string(), "minter".to_string()),
            ("jane.testnet".to_string(), "manager".to_string()),
        ];

        for (member, role) in role_vec.iter() {
            let mut role_data = AccessControlRoleData {
                members: near_sdk::collections::LookupSet::new(
                    StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
                ),
                admin_role: near_sdk::collections::LookupMap::new(
                    StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
                ),
            };

            role_data.members.insert(member);
            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());

            contract.access_control_roles.insert(role, &role_data);
        }

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        contract.pub_toggle_pause()
    }

    #[test]
    #[should_panic(expected = r#"Account mike.testnet , is missing: default_admin role"#)]
    fn check_role_works_denied() {
        let context = get_context(String::from("mike.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

        let role_vec = vec![
            ("timur.testnet".to_string(), "default_admin".to_string()),
            ("mike.testnet".to_string(), "minter".to_string()),
            ("jane.testnet".to_string(), "manager".to_string()),
        ];

        for (member, role) in role_vec.iter() {
            let mut role_data = AccessControlRoleData {
                members: near_sdk::collections::LookupSet::new(
                    StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
                ),
                admin_role: near_sdk::collections::LookupMap::new(
                    StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
                ),
            };

            role_data.members.insert(member);
            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());

            contract.access_control_roles.insert(role, &role_data);
        }

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        contract.pub_toggle_pause()
    }

    #[test]
    fn set_admin_role_works_allowed() {
        let context = get_context(String::from("timur.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

        let role_vec = vec![
            ("timur.testnet".to_string(), "default_admin".to_string()),
            ("mike.testnet".to_string(), "minter".to_string()),
            ("jane.testnet".to_string(), "manager".to_string()),
        ];

        for (member, role) in role_vec.iter() {
            let mut role_data = AccessControlRoleData {
                members: near_sdk::collections::LookupSet::new(
                    StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
                ),
                admin_role: near_sdk::collections::LookupMap::new(
                    StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
                ),
            };

            role_data.members.insert(member);
            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());

            contract.access_control_roles.insert(role, &role_data);
        }

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        contract.set_admin_role(&"minter".to_string(), &"manager".to_string());

        assert_eq!(
            contract.get_role_admin(&"minter".to_string()),
            "manager".to_string()
        )
    }

    #[test]
    #[should_panic(expected = r#"Account mike.testnet , is missing: default_admin role"#)]
    fn set_admin_role_works_denied() {
        let context = get_context(String::from("mike.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

        let role_vec = vec![
            ("timur.testnet".to_string(), "default_admin".to_string()),
            ("mike.testnet".to_string(), "minter".to_string()),
            ("jane.testnet".to_string(), "manager".to_string()),
        ];

        for (member, role) in role_vec.iter() {
            let mut role_data = AccessControlRoleData {
                members: near_sdk::collections::LookupSet::new(
                    StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
                ),
                admin_role: near_sdk::collections::LookupMap::new(
                    StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
                ),
            };

            role_data.members.insert(member);
            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());

            contract.access_control_roles.insert(role, &role_data);
        }

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        contract.set_admin_role(&"minter".to_string(), &"manager".to_string());
    }

    #[test]
    fn grant_role_works_allowed() {
        let context = get_context(String::from("timur.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

        let role_vec = vec![
            ("timur.testnet".to_string(), "default_admin".to_string()),
            ("mike.testnet".to_string(), "minter".to_string()),
            ("jane.testnet".to_string(), "manager".to_string()),
        ];

        for (member, role) in role_vec.iter() {
            let mut role_data = AccessControlRoleData {
                members: near_sdk::collections::LookupSet::new(
                    StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
                ),
                admin_role: near_sdk::collections::LookupMap::new(
                    StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
                ),
            };

            role_data.members.insert(member);
            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());

            contract.access_control_roles.insert(role, &role_data);
        }

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        contract.grant_role(&"manager".to_string(), &"mike.testnet".to_string());

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));

        assert_eq!(mike_role[0].to_string(), "minter".to_string());
        assert_eq!(mike_role[1].to_string(), "manager".to_string());
    }

    #[test]
    #[should_panic(expected = r#"Account mike.testnet , is missing: default_admin role"#)]
    fn grant_role_works_denied() {
        let context = get_context(String::from("mike.testnet"), 0);
        testing_env!(context);

        let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

        let role_vec = vec![
            ("timur.testnet".to_string(), "default_admin".to_string()),
            ("mike.testnet".to_string(), "minter".to_string()),
            ("jane.testnet".to_string(), "manager".to_string()),
        ];

        for (member, role) in role_vec.iter() {
            let mut role_data = AccessControlRoleData {
                members: near_sdk::collections::LookupSet::new(
                    StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
                ),
                admin_role: near_sdk::collections::LookupMap::new(
                    StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
                ),
            };

            role_data.members.insert(member);
            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());

            contract.access_control_roles.insert(role, &role_data);
        }

        let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
        let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
        let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

        assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
        assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
        assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

        contract.grant_role(&"manager".to_string(), &"mike.testnet".to_string());
    }

    // #[test]
    // fn only_role_admin_can_grant_role() {
    //     let context = get_context(String::from("jane.testnet"), 0);
    //     testing_env!(context);

    //     let mut contract = StatusMessage::without_setup(String::from("SOME DATA"));

    //     let role_vec = vec![
    //         ("timur.testnet".to_string(), "default_admin".to_string()),
    //         ("mike.testnet".to_string(), "minter".to_string()),
    //         ("jane.testnet".to_string(), "manager".to_string()),
    //     ];

    //     for (member, role) in role_vec.iter() {
    //         let mut role_data = AccessControlRoleData {
    //             members: near_sdk::collections::LookupSet::new(
    //                 StorageKeyAccessControl::RoleData(role.to_string()).into_bytes(),
    //             ),
    //             admin_role: near_sdk::collections::LookupMap::new(
    //                 StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes(),
    //             ),
    //         };

    //         role_data.members.insert(member);

    //         if role == "minter".to_string() {
    //             role_data.admin_role.insert(role, &"manager".to_string());
    //         } else {
    //             role_data
    //                 .admin_role
    //                 .insert(role, &"default_admin".to_string());
    //         }

    //         contract.access_control_roles.insert(role, &role_data);
    //     }

    //     let mike_role = contract.get_account_roles(&String::from("mike.testnet"));
    //     let timur_role = contract.get_account_roles(&String::from("timur.testnet"));
    //     let jane_role = contract.get_account_roles(&String::from("jane.testnet"));

    //     assert_eq!(timur_role.first().unwrap(), &String::from("default_admin"));
    //     assert_eq!(mike_role.first().unwrap(), &String::from("minter"));
    //     assert_eq!(jane_role.first().unwrap(), &String::from("manager"));

    //     contract.grant_role()
    // }
}
