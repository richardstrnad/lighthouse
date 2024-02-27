//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::ComparableOperator;
use crate::generated::types::IntegerOperator;
use crate::generated::types::MetaAssertion;
use crate::generated::types::StakeAssertion;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StakeAccountAssertion {
    State(u8, ComparableOperator),
    MetaAssertion(MetaAssertion),
    StakeAssertion(StakeAssertion),
    StakeFlags(u8, IntegerOperator),
}
