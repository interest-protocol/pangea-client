use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::H256;

use crate::{core::types::ChainId, query::Bound, utils::serialize_comma_separated};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetPricesRequest {
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
        alias = "price_id__in",
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub identifier__in: HashSet<H256>,
}

impl Default for GetPricesRequest {
    fn default() -> Self {
        Self {
            chains: default_chains(),
            from_block: Bound::default(),
            to_block: Bound::default(),
            identifier__in: HashSet::new(),
        }
    }
}

pub fn default_chains() -> HashSet<ChainId> {
    HashSet::from([ChainId::MOVEMENT])
}
