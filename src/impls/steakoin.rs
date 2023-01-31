use crate::error::SteakErr;
pub use crate::traits::steakoin::*;
use ink::prelude::vec::*;
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
    T: psp22::Internal,
    T: psp22::PSP22,
{
    fn steak(&mut self, amount: Balance) -> Result<(), SteakErr> {
        let caller = Self::env().caller();
        let staked = &self.data::<Data>().staked.get(&caller);

        if amount == 0 {
            return Err(SteakErr::AmountMustBeAboveZero)
        }

        if let Some(staking_data) = staked {
            // accumulate
            self._accummulate(&staking_data)?;
        } else {
            // new stake
            let _ = &self
                .data::<Data>()
                .staked
                .insert(&caller, &(amount, Self::env().block_timestamp()));
        }

        self.emit_steaked(amount, Self::env().block_timestamp());
        self.transfer_from(caller, Self::env().account_id(), amount, Vec::default())?;

        Ok(())
    }

    fn unsteak(&mut self, amount: Balance) -> Result<(), SteakErr> {
        let caller = Self::env().caller();
        let staked = &self.data::<Data>().staked.get(&caller);

        if amount == 0 {
            return Err(SteakErr::AmountMustBeAboveZero)
        }

        if let Some(staking_data) = staked {
            // accumulate
            let new_amount = self._accummulate(&staking_data)?;
            if amount >= new_amount {
                // withdraw max
                self.data().staked.remove(&caller);
                self.transfer_from(Self::env().account_id(), caller, new_amount, Vec::default())?;
            } else {
                self.data()
                    .staked
                    .insert(&caller, &(new_amount - amount, Self::env().block_timestamp()));
                self.transfer_from(Self::env().account_id(), caller, amount, Vec::default())?;
            }
        } else {
            // no stake
            return Err(SteakErr::NothingToWithdraw)
        }

        Ok(())
    }

    fn voting_power(&self, account: AccountId) -> Balance {
        let staked = &self.data::<Data>().staked.get(&account);

        if let Some(staking_data) = staked {
            return self._get_accumulated_amount(&staking_data)
        } else {
            0
        }
    }
}

pub trait Internal {
    fn _accummulate(&mut self, staking_data: &(Balance, Timestamp)) -> Result<Balance, SteakErr>;

    fn _get_accumulated_amount(&self, staking_data: &(Balance, Timestamp)) -> Balance;

    fn emit_steaked(&mut self, amount: Balance, timestamp: Timestamp);
}

impl<T> Internal for T
where
    T: Storage<Data>,
    T: psp22::Internal,
{
    fn _accummulate(&mut self, staking_data: &(Balance, Timestamp)) -> Result<Balance, SteakErr> {
        let last_amount = staking_data.0;
        let reward = self._get_accumulated_amount(staking_data);
        if reward > 0 {
            // if reward is too small we don't update
            self.data().staked.insert(
                &Self::env().caller(),
                &(last_amount + reward, Self::env().block_timestamp()),
            );
            // we will mint to the contract
            self._mint_to(Self::env().account_id(), reward)?;
        }

        Ok(last_amount + reward)
    }

    fn _get_accumulated_amount(&self, staking_data: &(Balance, Timestamp)) -> Balance {
        let last_amount = staking_data.0;
        let staked_time = staking_data.1;

        let current_time = Self::env().block_timestamp();
        let delta = current_time - staked_time;

        // 11% APR
        let one_year = 365 * 24 * 60 * 60 * 1000;
        let per_year = ((last_amount * 111) - (last_amount * 100)) / 100;
        return per_year * delta as u128 / one_year
    }

    default fn emit_steaked(&mut self, _: Balance, _: Timestamp) {}
}
