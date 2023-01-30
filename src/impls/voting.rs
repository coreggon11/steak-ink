pub use crate::traits::voting::*;
use openbrush::{
    storage::Mapping,
    traits::{
        Storage,
        Timestamp,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub proposals: Mapping<Id, Vec<(String, Balance, Timestamp)>>,
    pub last_id: Id,
    pub steakoin: AccountId,
    pub _reserved: Option<()>,
}

impl<T> Voting for T
where
    T: Storage<Data>,
{
    fn propose(&mut self, name: String, options: Vec<String>) -> Result<(), SteakErr> {
        let id = self.data().last_id;

        // must have at least 1_000_000 voting power to propose
        if StakingRef::voting_power(&steakoin, Self::env().caller()) < 1_000_000 * 10_u128.pow(18) {
            return Err(SteakErr::NotEnoughVotingPower)
        }

        self.data().last_id += 1;
        Ok(())
    }

    fn vote(&mut self, proposal_id: Id, option: u8) -> Result<(), SteakErr> {
        Ok(())
    }
}

pub trait Internal {}

impl<T> Internal for T where T: Storage<Data> {}
