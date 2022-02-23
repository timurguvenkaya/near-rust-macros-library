use proc_macro::TokenStream;

use quote::quote;
use syn::Ident;

pub fn access_control(item_ident: Ident) -> (TokenStream, TokenStream) {
    let core_ts = quote! {

        pub enum StorageKeyAccessControl {
            Roles,
            AdminRole(String),
            RoleData(String)
        }

        impl StorageKeyAccessControl {
            pub fn to_string(&self) -> String {
                match self {
                    StorageKeyAccessControl::Roles => "rol".to_string(),
                    StorageKeyAccessControl::AdminRole(adm) => format!("{}adm", adm),
                    StorageKeyAccessControl::RoleData(data) => format!("{}data", data),
                }
            }

            pub fn into_bytes(&self) -> std::vec::Vec<u8> {
                self.to_string().into_bytes()
            }
        }

        #[derive(near_sdk::borsh::BorshDeserialize, near_sdk::borsh::BorshSerialize)]
        pub struct AccessControlRoleData {
            members: near_sdk::collections::LookupSet<near_sdk::AccountId>,
            admin_role: near_sdk::collections::LookupMap<String, String>,
     }


        pub trait AccessControl {
            fn add_role(&mut self, role: &String);

            fn has_role(&self, role: &String, account: &near_sdk::AccountId) -> bool;

            fn check_role(&self, role: &String, account: &near_sdk::AccountId);

            fn assert_role(&self, role: &String);

            fn get_role_admin(&self, role: &String) -> String;

            fn get_account_roles(&self, account: &near_sdk::AccountId) -> Vec<String>;

            fn grant_role(&mut self, role: &String, account: &near_sdk::AccountId);

            fn setup_account_role(&mut self, role: &String, account: &near_sdk::AccountId);

            fn revoke_role(&mut self, role: &String, account: &near_sdk::AccountId);

            fn set_admin_role(&mut self, role: &String, admin_role: &String);

            fn add_role_member(&mut self, role: &String, account: &near_sdk::AccountId);
        }


            #[near_bindgen]
            impl AccessControl for #item_ident {

                #[private]
                fn add_role(&mut self, role: &String) {

                    // Check that role is not already registered
                    if self.access_control_roles.get(role).is_none() {
                        let mut role_data = AccessControlRoleData {
                            members: near_sdk::collections::LookupSet::new(StorageKeyAccessControl::RoleData(role.to_string()).into_bytes()),
                            admin_role: near_sdk::collections::LookupMap::new(StorageKeyAccessControl::AdminRole(role.to_string()).into_bytes()),
                        };

                        role_data
                            .admin_role
                            .insert(role, &"default_admin".to_string());
                        self.access_control_roles.insert(role, &role_data);

                        near_sdk::env::log(format!("Role {} is added", role).as_bytes())
                    }
                }

                fn has_role(&self, role: &String, account: &near_sdk::AccountId) -> bool {
                    let role_data = self.access_control_roles.get(role);

                    match role_data {
                        Some(r) => r.members.contains(account),
                        None => near_sdk::env::panic(format!("Role: {} does not exist", role).as_bytes())
                    }
                }


                fn check_role(&self, role: &String, account: &near_sdk::AccountId) {
                    if !self.has_role(role, account) {
                        env::panic(format!("Account {} , is missing: {} role", account, role).as_bytes());
                    }
                }


                fn assert_role(&self, role: &String) {
                    self.check_role(role, &near_sdk::env::predecessor_account_id())
                }

                fn get_role_admin(&self, role: &String) -> String {
                    let role_data = self.access_control_roles.get(role);

                    match role_data {
                        Some(r) => {

                            r.admin_role.get(role).unwrap().to_string()
                        }
                        None => near_sdk::env::panic(format!("Role: {} does not exist", role).as_bytes())
                    }
                }

                fn get_account_roles(&self, account: &near_sdk::AccountId) -> std::vec::Vec<String> {
                    let mut found_role = std::vec::Vec::new();

                    for role in self.access_control_roles.keys() {
                        if self.has_role(&role, account) {
                            found_role.push(role);
                        }
                    }

                    found_role
                }

                fn grant_role(&mut self, role: &String, account: &near_sdk::AccountId) {
                    self.assert_role(&self.get_role_admin(role));
                    self.add_role_member(role, account);
                }

                #[private]
                fn setup_account_role(&mut self, role: &String, account: &near_sdk::AccountId) {

                    self.add_role(role);
                    self.add_role_member(role, account);
                }

                fn revoke_role(&mut self, role: &String, account: &near_sdk::AccountId) {
                    self.assert_role(&self.get_role_admin(role));

                    if self.has_role(role, account) {
                        let role_data = self.access_control_roles.get(role);

                        match role_data {
                            Some(mut r) => {
                                r.members.remove(account);

                                near_sdk::env::log(format!("Role {} is revoked from {}", role, account).as_bytes())

                            }
                            None => near_sdk::env::panic(format!("Role: {} does not exist", role).as_bytes()),
                        }
                    }
                }

                fn set_admin_role(&mut self, role: &String, admin_role: &String) {

                    self.assert_role(&self.get_role_admin(role));

                    if self.get_role_admin(role) != *admin_role {
                        let role_data = self.access_control_roles.get(role);

                        match role_data {
                            Some(mut r) => {
                                r.admin_role.get(role).unwrap().clear();

                                r.admin_role.insert(role, &admin_role.to_string());

                                near_sdk::env::log(
                                    format!(
                                        "Changed admin role from: {}. To: {}",
                                        r.admin_role.get(role).unwrap(),
                                        admin_role
                                    )
                                    .as_bytes(),
                                );
                            }
                            None => near_sdk::env::panic(format!("Role: {} does not exist", role).as_bytes()),
                        }
                    }
                }

                #[private]
                fn add_role_member(&mut self, role: &String, account: &near_sdk::AccountId) {

                    if !self.has_role(role, account) {
                        let role_data = self.access_control_roles.get(role);

                        near_sdk::env::log(format!("Setting role: {}. To: {}", role, account).as_bytes());

                        match role_data {
                            Some(mut r) => {
                                r.members.insert(account);

                                near_sdk::env::log(format!("Account {} is added to {}", account, role).as_bytes())

                            }
                            None => near_sdk::env::panic(format!("Role: {} does not exist", role).as_bytes()),
                        }
                    }
                }

                }

    };

    (quote! {#core_ts}.into(), quote! {access_control_roles: near_sdk::collections::UnorderedMap<String, AccessControlRoleData>,}.into())
}
