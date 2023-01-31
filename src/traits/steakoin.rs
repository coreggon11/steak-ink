use openbrush::traits::{
    AccountId,
    Balance,
};
use crate::libs::error::SteakErr;

#[openbrush::trait_definition]
pub trait Steakoin {
    #[ink(message)]
    fn steak(&mut self, amount: Balance) -> Result<(), SteakErr>;

    #[ink(message)]
    fn unsteak(&mut self, amount: Balance) -> Result<(), SteakErr>;

    #[ink(message)]
    fn voting_power(&self, account: AccountId) -> Balance;
}
