#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod steakoin {
    use ink::codegen::EmitEvent;
    use openbrush::{
        contracts::psp22::*,
        traits::{
            DefaultEnv,
            Storage,
        },
    };
    use steak_ink::impls::steakoin::*;

    #[ink(event)]
    pub struct SteakEvent {
        #[ink(topic)]
        staker: AccountId,
        amount: Balance,
        timestamp: Timestamp,
    }

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

    impl steak_ink::Internal for SteakoinContract {
        fn emit_steaked(&mut self, amount: Balance, timestamp: Timestamp) {
            Self::env().emit_event({
                SteakEvent {
                    staker: Self::env().caller(),
                    amount,
                    timestamp,
                }
            })
        }
    }

    impl SteakoinContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
                ._mint_to(instance.env().caller(), 1_000_000 * 10u128.pow(18))
                .expect("Should mint");
            instance
        }
    }
}
