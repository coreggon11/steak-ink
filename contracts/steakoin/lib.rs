#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod steakoin {
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Steakoin {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for Steakoin {}

    impl Steakoin {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
