use openbrush::{
    contracts::psp22::PSP22Error,
    traits::String,
};
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SteakErr {
    PSP22Error(PSP22Error),
    AmountMustBeAboveZero,
    NothingToWithdraw,

    NotEnoughVotingPower,
    MaxFourOptions,
    AtLeastOneDay,
    NoVotingPower,
    ProposalDoesNotExist,
    ProposalExpired,
    IncorrectOption,
}

impl From<SteakErr> for PSP22Error {
    fn from(err: SteakErr) -> Self {
        match err {
            SteakErr::PSP22Error(err) => err,
            _ => PSP22Error::Custom(String::from("Custom")),
        }
    }
}

impl From<PSP22Error> for SteakErr {
    fn from(err: PSP22Error) -> Self {
        SteakErr::PSP22Error(err)
    }
}
