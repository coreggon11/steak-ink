#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod steakoin {

    #[ink(storage)]
    #[derive(Default)]
    pub struct Steakoin {}

    impl Steakoin {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self::default()
        }
    }
}
