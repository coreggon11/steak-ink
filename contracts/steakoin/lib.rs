#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod steakoin {
    use openbrush::{
        contracts::psp22::{
            Data as PSP22Data,
            *,
        },
        traits::Storage,
    };
    use steak_ink::impls::steakoin::{
        Data as SteakoinData,
        *,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct SteakoinContract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
        staking: SteakoinData,
    }

    impl PSP22 for SteakoinContract {}

    impl Steakoin for SteakoinContract {}

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {}
    }
}
