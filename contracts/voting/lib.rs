#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod voting {
    use openbrush::traits::Storage;
    // use steak_ink::impls::voting::{
    //     Data as VotingData,
    //     *,
    // };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct VotingContract {
        // #[storage_field]
        // voting: VotingData,
    }

    // impl Voting for VotingContract {}

    impl VotingContract {
        #[ink(constructor)]
        pub fn new(steakoin_account: AccountId) -> Self {
            let mut instance = Self::default();
            // instance.voting.steakoin = steakoin_account;
            instance
        }

        #[ink(message)]
        pub fn message(&self) {}
    }
}
