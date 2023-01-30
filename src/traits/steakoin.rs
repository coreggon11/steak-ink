pub use crate::libs::error::SteakErr;
pub use ink::prelude::vec::Vec;
pub use openbrush::{
    contracts::psp22::*,
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type SteakoinRef = dyn Steakoin + PSP22;

#[openbrush::trait_definition]
pub trait Steakoin {
    #[ink(message)]
    fn steak(&mut self, amount: Balance) -> Result<(), SteakErr>;

    #[ink(message)]
    fn unsteak(&mut self, amount: Balance) -> Result<(), SteakErr>;

    #[ink(message)]
    fn voting_power(&self, account: AccountId) -> u128;
}
