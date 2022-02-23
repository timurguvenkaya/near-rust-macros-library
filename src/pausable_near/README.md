
# Pausable NEAR

Pausable near is a macro that adds pausability to NEAR smart contracts.

## Methods
All methods are intentionally made private to pick the methods to whitelist and add access control to them. 


```rust

      pub trait Pausable {

        fn toggle_pause(&mut self);
        fn pause(&mut self);
        fn unpause(&mut self);
        fn when_not_paused(&self);

    }

```

- __toogle_pause__: Toggles between paused and unpaused
- __pause__: Pauses the contract
- __unpause__: Unpauses the contract
- __when_not_paused__: Checks whether the function is not paused
  

## Usage
You can run the test application in the **example** folder, which is a fork of **StatusMessage** by calling `./build.sh` and then `./deploy.sh`. Please update `./deploy.sh` to have your accounts. For your projects, you have to include the `near_macros` crate.

The only thing needed is to add the `#[require(Pausable)]` attribute macro to your main struct to begin using methods from this macro. Please also note that `#[require(Pausable)]` macro already includes `#[derive(BorshDeserialize, BorshSerialize)]`. Therefore, please do not derive it the second time on your main struct, where the `#[require(Pausable)]` is used. 

```rust
use near_macros::{init_macro, require};
...

#[near_bindgen]
#[derive(PanicOnDefault)]
#[require(Pausable)]
pub struct StatusMessage {
    data: String,
}

```

Then, to begin using methods in the Pausable NEAR, you have first to call the `init_macro!()`  with the `pausable` argument and the struct initialization as the last argument. 

```rust

#[near_bindgen]
impl StatusMessage {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");

        let constructor = init_macro!([
            "pausable",
            Self {
                data: String::from("SOME DATA")
            }
        ]);

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

```

## TODOS
- Finishing up tests.
- Doing audit for this macro.