use crate::traits::steakoin::SteakoinRef;
pub use crate::traits::voting::*;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
        Timestamp,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    pub proposals: Mapping<Id, (String, Timestamp, Vec<(String, Balance)>)>,
    pub last_id: Id,
    pub steakoin: AccountId,
    pub _reserved: Option<()>,
}

#[openbrush::modifier_definition]
fn has_voting_power<R, T, F>(instance: &mut T, body: F) -> Result<R, SteakErr>
where
    T: Storage<Data>,
    F: FnOnce(&mut T) -> Result<R, SteakErr>,
{
    if !SteakoinRef::voting_power(&instance.data().steakoin, T::env().caller()) == 0 {
        return Err(SteakErr::NoVotingPower)
    }
    body(instance)
}

impl<T> Voting for T
where
    T: Storage<Data>,
{
    fn propose(&mut self, name: String, options: Vec<String>, duration: Timestamp) -> Result<(), SteakErr> {
        let id = self.data().last_id;

        // must have at least 1_000_000 voting power to propose
        if SteakoinRef::voting_power(&self.data().steakoin, Self::env().caller()) < 1_000_000 * 10_u128.pow(18) {
            return Err(SteakErr::NotEnoughVotingPower)
        }
        if options.len() > 4 {
            return Err(SteakErr::MaxFourOptions)
        }
        if duration < 24 * 60 * 60 * 1000 {
            return Err(SteakErr::AtLeastOneDay)
        }

        self.data().proposals.insert(
            &id,
            &(
                name,
                Self::env().block_timestamp() + duration,
                options.iter().map(|option| (option.clone(), 0)).collect(),
            ),
        );

        self.data().last_id += 1;
        Ok(())
    }

    #[openbrush::modifiers(has_voting_power)]
    fn vote(&mut self, proposal_id: Id, option: u8) -> Result<(), SteakErr> {
        // exists
        if let Some((name, expiration, options)) = self.data().proposals.get(&proposal_id) {
            // expired
            if expiration < Self::env().block_timestamp() {
                return Err(SteakErr::ProposalExpired)
            }
            // correct option
            if option >= options.len() as u8 {
                return Err(SteakErr::IncorrectOption)
            }

            let mut original = options;
            original[option as usize] = (
                original[option as usize].0.clone(),
                original[option as usize].1 + SteakoinRef::voting_power(&self.data().steakoin, Self::env().caller()),
            );

            self.data()
                .proposals
                .insert(&proposal_id, &(name, expiration, original));
        } else {
            return Err(SteakErr::ProposalDoesNotExist)
        }
        Ok(())
    }

    fn get_votes(&self, proposal_id: Id) -> Vec<(String, Balance)> {
        self.data().proposals.get(&proposal_id).unwrap_or_default().2
    }
}
