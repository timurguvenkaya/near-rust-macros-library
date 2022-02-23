use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, PanicOnDefault};

mod storage;

use storage::StorageKey;

setup_alloc!();

const DEFAULT_ADMIN: &str = "default_admin";
const MINTER: &str = "minter";
const MANAGER: &str = "manager";

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RoleData {
    members: LookupSet<AccountId>,
    admin_role: LookupMap<String, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StatusMessage {
    records: LookupMap<String, String>,
    roles: UnorderedMap<String, RoleData>,
}

#[near_bindgen]
impl StatusMessage {
    #[init]
    pub fn new(owner: AccountId, minter: AccountId, manager: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let mut constructor = Self {
            records: LookupMap::new(StorageKey::Records.into_bytes()),
            roles: UnorderedMap::new(StorageKey::Roles.into_bytes()),
        };

        constructor.setup_account_role(&DEFAULT_ADMIN.to_string(), &owner);
        constructor.setup_account_role(&MINTER.to_string(), &minter);
        constructor.setup_account_role(&MANAGER.to_string(), &manager);

        constructor
    }

}

pub trait AccessControl {
    fn add_role(&mut self, role: &String);

    fn has_role(&self, role: &String, account: &AccountId) -> bool;

    fn check_role(&self, role: &String, account: &AccountId);

    fn assert_role(&self, role: &String);

    fn assert_self(&mut self);

    fn get_role_admin(&self, role: &String) -> String;

    fn get_account_roles(&self, account: &AccountId) -> Vec<String>;

    fn grant_role(&mut self, role: &String, account: &AccountId);

    fn setup_account_role(&mut self, role: &String, account: &AccountId);

    fn delete_role_member(&mut self, role: &String, account: &AccountId);

    fn revoke_role(&mut self, role: &String, account: &AccountId);

    fn set_admin_role(&mut self, role: &String, admin_role: &String);

    fn add_role_member(&mut self, role: &String, account: &AccountId);
}

#[near_bindgen]
impl AccessControl for StatusMessage {
    fn add_role(&mut self, role: &String) {
        self.assert_self();
        // Check that role is not already registered
        if self.roles.get(role).is_none() {
            let mut role_data = RoleData {
                members: LookupSet::new(StorageKey::RoleData(role.to_string()).into_bytes()),
                admin_role: LookupMap::new(StorageKey::AdminRole(role.to_string()).into_bytes()),
            };

            role_data
                .admin_role
                .insert(role, &"default_admin".to_string());
            self.roles.insert(role, &role_data);

            env::log(format!("Role {} is added", role).as_bytes())
        }
    }

    fn has_role(&self, role: &String, account: &AccountId) -> bool {
        let role_data = self.roles.get(role);

        match role_data {
            Some(r) => r.members.contains(account),
            None => env::panic(format!("Role: {} does not exist", role).as_bytes()),
        }
    }

    fn check_role(&self, role: &String, account: &AccountId) {
        if !self.has_role(role, account) {
            env::panic(format!("Account {} , is missing: {} role", account, role).as_bytes());
        }
    }

    fn assert_role(&self, role: &String) {
        self.check_role(role, &env::predecessor_account_id())
    }

    fn assert_self(&mut self) {
        if env::predecessor_account_id() != env::current_account_id() {
            env::panic("Function is private".as_bytes())
        }
    }

    fn get_role_admin(&self, role: &String) -> String {
        let role_data = self.roles.get(role);

        match role_data {
            Some(r) => r.admin_role.get(role).unwrap().to_string(),
            None => env::panic(format!("Role: {} does not exist", role).as_bytes()),
        }
    }

    fn get_account_roles(&self, account: &AccountId) -> Vec<String> {
        let mut found_role = Vec::new();

        for role in self.roles.keys() {
            if self.has_role(&role, account) {
                found_role.push(role);
            }
        }

        found_role
    }

    fn grant_role(&mut self, role: &String, account: &AccountId) {
        self.assert_role(&self.get_role_admin(role));
        self.add_role_member(role, account);
    }

    fn setup_account_role(&mut self, role: &String, account: &AccountId) {
        self.assert_self();

        self.add_role(role);
        self.add_role_member(role, account);
    }

    fn delete_role_member(&mut self, role: &String, account: &AccountId) {
        self.assert_self();

        if self.has_role(role, account) {
            let role_data = self.roles.get(role);

            match role_data {
                Some(mut r) => {
                    r.members.remove(account);

                    env::log(format!("Role {} is revoked from {}", role, account).as_bytes())
                }
                None => env::panic(format!("Role: {} does not exist", role).as_bytes()),
            }
        }
    }

    fn revoke_role(&mut self, role: &String, account: &AccountId) {
        self.assert_role(&self.get_role_admin(role));

        self.delete_role_member(role, account)
    }

    fn set_admin_role(&mut self, role: &String, admin_role: &String) {
        self.assert_role(&self.get_role_admin(role));

        if self.get_role_admin(role) != *admin_role {
            let role_data = self.roles.get(role);

            match role_data {
                Some(mut r) => {
                    r.admin_role.get(role).unwrap().clear();

                    r.admin_role.insert(role, &admin_role.to_string());

                    env::log(
                        format!(
                            "Changed admin role from: {}. To: {}",
                            r.admin_role.get(role).unwrap(),
                            admin_role
                        )
                        .as_bytes(),
                    );
                }
                None => env::panic(format!("Role: {} does not exist", role).as_bytes()),
            }
        }
    }

    fn add_role_member(&mut self, role: &String, account: &AccountId) {
        self.assert_self();
        if !self.has_role(role, account) {
            let role_data = self.roles.get(role);

            env::log(format!("Setting role: {}. To: {}", role, account).as_bytes());

            match role_data {
                Some(mut r) => {
                    r.members.insert(account);

                    env::log(format!("Account {} is added to {}", account, role).as_bytes())
                }
                None => env::panic(format!("Role: {} does not exist", role).as_bytes()),
            }
        }
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_utils::{get_logs, VMContextBuilder};
//     use {testing_env, VMContext};

//     fn get_context(is_view: bool) -> VMContext {
//         VMContextBuilder::new()
//             .signer_account_id("bob_near".parse().unwrap())
//             .predecessor_account_id("alice_near".parse().unwrap())
//             .current_account_id("smart_near".parse().unwrap())
//             .is_view(is_view)
//             .build()
//     }

//     #[test]
//     fn test_panic() {

//     }
// }
