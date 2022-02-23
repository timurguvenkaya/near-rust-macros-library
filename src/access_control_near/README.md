# Access Control NEAR

Access Control NEAR is a library for implementing role-based access control model in NEAR smart contracts. It is partially inspired by OpenZeppelin's access control implementation. 

## Architecture

The **Roles** map consists of **Role** to **Role Data** mapping. New members are added to the **members** set by inserting new **AccountId**. Each role has an **Admin Role**, whose members are allowed to perform privileged actions on the role that derives it. Default **admin_role** for all created roles is `default_admin` 

![diagram1.png](https://github.com/timurguvenkaya/near-rust-macros-library/blob/master/src/access_control_near/images/diagram1.png) 

![diagram2.png](https://github.com/timurguvenkaya/near-rust-macros-library/blob/master/src/access_control_near/images/diagram2.png)

### Methods

There are private and public methods. Private methods can only be called by the smart contract itself.

#### Public Methods

```rust
fn has_role(&self, role: &String, account: &AccountId) -> bool;

fn check_role(&self, role: &String, account: &AccountId);

fn assert_role(&self, role: &String);

fn get_role_admin(&self, role: &String) -> String;

fn get_account_roles(&self, account: &AccountId) -> Vec<String>;

fn grant_role(&mut self, role: &String, account: &AccountId);

fn revoke_role(&mut self, role: &String, account: &AccountId);

fn set_admin_role(&mut self, role: &String, admin_role: &String);

```

- **has_role**: Checks if given account has the given role. Returns bool
- **check_role**: Checks if given account has the given role. Panics with a message
- **assert_role**: Checks whether the caller has given role. Panics with a message. Internally calls **check_role** with `env::predecessor_account_id()`
- **get_role_admin**: View method. Gets the admin role of a given role. Returns String
- **get_account_roles**: View method. Gets all roles that the given account has. Returns a vector containing all the roles
- **grant_role**: Can only be called by the role admin of given role. Grants given role to given account.
- **revoke_role**: Can only be called by the role admin of given role. Revokes given role for given account.
- **set_admin_role**:  Can only be called by the role admin of given role. Sets the new admin role for given role.


#### Private Methods

```rust
fn setup_account_role(&mut self, role: &String, account: &AccountId);
```

- **setup_account_role**: Sets the given role to given account. If role does not exist it first creates by calling **add_role.**

#### Private Helper Methods

```rust
fn add_role(&mut self, role: &String);

fn add_role_member(&mut self, role: &String, account: &AccountId);
```

- **add_role**: Adds a new role and sets `default_admin` as the **admin_role**
- **add_role_member**: Adds a new member to a role.

## Usage

You can run the test application in the **example** folder, which is a fork of **StatusMessage** by calling `./build.sh` and then `./deploy.sh` . Please update `./deploy.sh` to have your accounts. For your own projects you have to include `near_macros` crate.

The only thing needed is to add `#[require(Access)]` attribute macro to your main struct to begin using methods from this library. Please also note that `#[require(Access)]`  macro already includes `#[derive(BorshDeserialize, BorshSerialize)]`.Therefore, please do not derive it the second time on your main struct, where the `#[require(Access)]` is used. 

```rust
use near_macros::{require, init_macro};
...

const DEFAULT_ADMIN: &str = "default_admin";
const MINTER: &str = "minter";
const MANAGER: &str = "manager";

#[near_bindgen]
#[derive(PanicOnDefault)]
#[require(Access)]
pub struct StatusMessage {
    data: String,
}
```

Then, to begin using methods in the Access Control NEAR and setup initial roles, you have first to call the `init_macro!()`  with the `access` argument and the struct initialization as the last argument. After that, you can setup roles you want to use. 

```rust
const DEFAULT_ADMIN: &str = "default_admin";
const MINTER: &str = "minter";
const MANAGER: &str = "manager";

.....
#[near_bindgen]
impl StatusMessage {

      #[init]
    pub fn new(owner: AccountId, minter: AccountId, manager: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let mut constructor = init_macro!([
            "access",
            Self {
                data: String::from("SOME DATA"),
            }
        ]);

        constructor.setup_account_role(&DEFAULT_ADMIN.to_string(), &owner);
        constructor.setup_account_role(&MINTER.to_string(), &minter);
        constructor.setup_account_role(&MANAGER.to_string(), &manager);

        constructor
    }

       pub fn get_data(&self) -> String {
        self.check_role(&DEFAULT_ADMIN.to_string(), &env::predecessor_account_id());
        self.data.clone()
    }

}
```

From now on, you can directly use Access Control NEAR methods within your smart contract. 

## TODOS

- Finishing up tests.
- Doing an audit for this library.
- Making stricter version, where one account is only allowed to have one role.
