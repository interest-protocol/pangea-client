use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(AsRefStr, Clone, Copy, Debug, Hash, Deserialize, Serialize, PartialEq, Eq)]
pub enum TransactionType {
    #[serde(alias = "UserTransaction")]
    UserTransaction = 0,
    #[serde(alias = "DirectGenesisTransaction")]
    DirectGenesisTransaction = 1,
    #[serde(alias = "ScriptGenesisTransaction")]
    ScriptGenesisTransaction = 2,
    #[serde(alias = "BlockMetadata")]
    BlockMetadata = 3,
    #[serde(alias = "BlockMetadataWithRandomness")]
    BlockMetadataWithRandomness = 4,
    #[serde(alias = "StateCheckpoint")]
    StateCheckpoint = 5,
    #[serde(alias = "DKGResultValidatorTransaction")]
    DKGResultValidatorTransaction = 6,
    #[serde(alias = "ObservedJWKUpdateValidatorTransaction")]
    ObservedJWKUpdateValidatorTransaction = 7,
    #[serde(alias = "BlockEpilogue")]
    BlockEpilogue = 8,
}

impl TransactionType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            TransactionType::UserTransaction => "UserTransaction",
            TransactionType::DirectGenesisTransaction => "DirectGenesisTransaction",
            TransactionType::ScriptGenesisTransaction => "ScriptGenesisTransaction",
            TransactionType::BlockMetadata => "BlockMetadata",
            TransactionType::BlockMetadataWithRandomness => "BlockMetadataWithRandomness",
            TransactionType::StateCheckpoint => "StateCheckpoint",
            TransactionType::DKGResultValidatorTransaction => "DKGResultValidatorTransaction",
            TransactionType::ObservedJWKUpdateValidatorTransaction => {
                "ObservedJWKUpdateValidatorTransaction"
            }
            TransactionType::BlockEpilogue => "BlockEpilogue",
        }
    }
}

impl TryFrom<i32> for TransactionType {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(TransactionType::UserTransaction),
            1 => Ok(TransactionType::DirectGenesisTransaction),
            2 => Ok(TransactionType::ScriptGenesisTransaction),
            3 => Ok(TransactionType::BlockMetadata),
            4 => Ok(TransactionType::BlockMetadataWithRandomness),
            5 => Ok(TransactionType::StateCheckpoint),
            6 => Ok(TransactionType::DKGResultValidatorTransaction),
            7 => Ok(TransactionType::ObservedJWKUpdateValidatorTransaction),
            8 => Ok(TransactionType::BlockEpilogue),
            _ => Err(crate::Error::UnknownTransactionType(v)),
        }
    }
}
