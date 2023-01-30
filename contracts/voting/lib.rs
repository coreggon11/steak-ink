#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod steakoin {
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;

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
