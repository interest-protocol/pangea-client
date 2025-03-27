use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::H256;

use crate::{
    core::types::{
        arche::{CollateralEventType, CollateralState, LoanEventType},
        ChainId,
    },
    query::Bound,
    utils::serialize_comma_separated,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetCollateralsRequest {
    #[serde(
        default = "default_chains",
        serialize_with = "serialize_comma_separated"
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
    pub event_type__in: HashSet<CollateralEventType>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub state__in: HashSet<CollateralState>,
}

impl Default for GetCollateralsRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            event_type__in: HashSet::new(),
            asset_address__in: HashSet::new(),
            state__in: HashSet::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetLoansRequest {
    #[serde(
        default = "default_chains",
        serialize_with = "serialize_comma_separated"
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
    pub event_type__in: HashSet<LoanEventType>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        alias = "collateral_asset__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub asset_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub liquidator_address__in: HashSet<H256>,
}

impl Default for GetLoansRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            event_type__in: HashSet::new(),
            address__in: HashSet::new(),
            asset_address__in: HashSet::new(),
            liquidator_address__in: HashSet::new(),
        }
    }
}

fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::MOVEMENT])
}
