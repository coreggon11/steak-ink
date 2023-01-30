pub use crate::libs::error::SteakErr;
pub use ink::prelude::vec::Vec;
pub use openbrush::traits::{
    Balance,
    String,
};

#[openbrush::wrapper]
pub type VotingRef = dyn Voting;

pub type Id = u128;

#[openbrush::trait_definition]
pub trait Voting {
    #[ink(message)]
    fn propose(&mut self, name: String, options: Vec<String>) -> Result<(), SteakErr>;

    #[ink(message)]
    fn vote(&mut self, proposal_id: Id, option: u8) -> Result<(), SteakErr>;
}
