use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::H256;

use crate::{core::types::ChainId, query::Bound, utils::serialize_comma_separated};

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetPoolsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "token0__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "token1__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "tokens__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub tokens_address__in: HashSet<H256>,
}

impl Default for GetPoolsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::new(),
            token0_address__in: HashSet::new(),
            token1_address__in: HashSet::new(),
            tokens_address__in: HashSet::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetLiquidityRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "token0__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "token1__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "tokens__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub tokens_address__in: HashSet<H256>,
}

impl Default for GetLiquidityRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::new(),
            token0_address__in: HashSet::new(),
            token1_address__in: HashSet::new(),
            tokens_address__in: HashSet::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetSwapsRequest {
    #[serde(default = "default_chains")]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "token0__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "token1__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub token1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "tokens__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub tokens_address__in: HashSet<H256>,
}

impl Default for GetSwapsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            pool_address__in: HashSet::new(),
            token0_address__in: HashSet::new(),
            token1_address__in: HashSet::new(),
            tokens_address__in: HashSet::new(),
        }
    }
}

fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::MOVEMENT])
}
