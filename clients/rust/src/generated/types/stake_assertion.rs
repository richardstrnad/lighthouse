//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::EquatableOperator;
use crate::generated::types::IntegerOperator;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StakeAssertion {
    DelegationVoterPubkey {
        #[cfg_attr(
            feature = "serde",
            serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
        )]
        value: Pubkey,
        operator: EquatableOperator,
    },
    DelegationStake {
        value: u64,
        operator: IntegerOperator,
    },
    DelegationActivationEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    DelegationDeactivationEpoch {
        value: u64,
        operator: IntegerOperator,
    },
    CreditsObserved {
        value: u64,
        operator: IntegerOperator,
    },
}
