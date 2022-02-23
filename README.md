
# NEAR Macros

Near Macros is a collection of contracts and macros that can be added directly to any project to add additional functionality without slowing down development. 

## NOTE
Currently, those macros are in the process of audit. Therefore, use it at your own risk in production.

## Directory Structure
For each contract, there is macro and its respective macro expansion. You can find examples for both cases in **example** folder.

```
.
├── src/
│   ├── access_control_near/
│   │   ├── access_control_near.rs
│   │   └── mod.rs
│   ├── pausable_near/
│   │   ├── pausable_near.rs
│   │   └── mod.rs
│   ├── ...
│   ├── init_macro.rs --> Macro for initialization new struct fields
│   └── lib.rs
└── test/
    ├── access_control_near/
    │   ├── example/ --> Example with near_macros
    │   └── example_expanded/ --> Example without near_macros
    ├── pausable_near/
    │   ├── example/ --> Example with near_macros
    │   └── example_expanded/ --> Example without near_macros
    ├── ...
    └── src/
        └── lib.rs --> Macro expansion testing

```
## Usage

All contracts and macros come with their __README.md__ file explaining the usage. If you want to combine different macros, you must add their names into `require` attribute macro. If a particular macro adds a new field to a struct, that new field has to be initialized with `init_macro` in the constructor. You can find all available macro names below. Please note that the project is in the __alpha__ stage; hence you need to pull it directly from Github to use (there is no deployed create yet).

- Access Control Macro: __Access__
- Pausable Macro: __Pausable__



Example(Pausable + Access):

```rust
use near_macros::{require, init_macro};
...

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
    pub fn new(owner: AccountId, minter: AccountId, manager: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let mut constructor = init_macro!([
            "access",
            "pausable",
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
        self.when_not_paused();
        self.data.clone()
    }

    pub fn pub_toggle_pause(&mut self) {
        self.check_role(&DEFAULT_ADMIN.to_string(), &env::predecessor_account_id());
        self.toggle_pause()
    }
}


```

## TODOS
- Finishing up tests.

## Authors

- [@timurguvenkaya](https://github.com/timurguvenkaya)


## License

[MIT](https://choosealicense.com/licenses/mit/)

