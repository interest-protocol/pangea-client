use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    core::types::{movement::TransactionType, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};
use fuel_core_types::fuel_types::Address; // use fuel for now 32 bit is good.

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMoveLogsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub address__in: HashSet<Address>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub module__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub event_name__in: HashSet<String>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub type_tag__in: HashSet<String>,
}

impl Default for GetMoveLogsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            address__in: HashSet::new(),
            module__in: HashSet::new(),
            event_name__in: HashSet::new(),
            type_tag__in: HashSet::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMoveTxsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Exclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub transaction_type__in: HashSet<TransactionType>,
}

impl Default for GetMoveTxsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            transaction_type__in: HashSet::new(),
        }
    }
}

pub fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::MOVEMENT])
}
