#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod steakoin {
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };
    use steak_ink::impls::steakoin::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct SteakoinContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        steakoin: steak_ink::Data,
    }

    impl PSP22 for SteakoinContract {}

    impl Steakoin for SteakoinContract {}

    impl SteakoinContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
