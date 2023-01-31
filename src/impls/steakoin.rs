use crate::error::SteakErr;
pub use crate::traits::steakoin::*;
use openbrush::{
    contracts::psp22,
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
        Storage,
        Timestamp,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub staked: Mapping<AccountId, (Balance, Timestamp)>,
    pub _reserved: Option<()>,
}

impl<T> Steakoin for T
where
    T: Storage<Data>,
    T: Storage<psp22::Data>,
    T: psp22::Internal,
{
    fn steak(&mut self, amount: Balance) -> Result<(), SteakErr> {
        Ok(())
    }

    fn unsteak(&mut self, amount: Balance) -> Result<(), SteakErr> {
        Ok(())
    }

    fn voting_power(&self, account: AccountId) -> Balance {
        0
    }
}
