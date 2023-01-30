pub use ink_prelude::vec::Vec;
use openbrush::contracts::psp22::extensions::{
    burnable::*,
    mintable::*,
};
pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type SteakoinRef = dyn Steakoin + PSP22;

#[openbrush::trait_definition]
pub trait Steakoin {
    #[ink(message)]
    fn steak(&mut self, amount: Balance) -> Result<(), SteakErr>;

    #[ink(message)]
    fn unsteak(&mut self) -> Result<(), SteakErr>;
}