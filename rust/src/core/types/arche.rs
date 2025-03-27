#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum CollateralEventType {
    AssetCollateralSupportAdded,
    AssetCollateralSupportRemoved,
    AssetCollateralActivated,
    AssetCollateralDeactivated,
    NewLoanToValue,
    NewAnnualPercentageYield,
    NewLiquidationDiscount,
    NewMaxBorrowableAmount,
    AddedToWhitelist,
    RemovedFromWhitelist,
    BorrowFeeUpdated,
}

impl CollateralEventType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssetCollateralSupportAdded => "AssetCollateralSupportAdded",
            Self::AssetCollateralSupportRemoved => "AssetCollateralSupportRemoved",
            Self::AssetCollateralActivated => "AssetCollateralActivated",
            Self::AssetCollateralDeactivated => "AssetCollateralDeactivated",
            Self::NewLoanToValue => "NewLoanToValue",
            Self::NewAnnualPercentageYield => "NewAnnualPercentageYield",
            Self::NewLiquidationDiscount => "NewLiquidationDiscount",
            Self::NewMaxBorrowableAmount => "NewMaxBorrowableAmount",
            Self::AddedToWhitelist => "AddedToWhitelist",
            Self::RemovedFromWhitelist => "RemovedFromWhitelist",
            Self::BorrowFeeUpdated => "BorrowFeeUpdated",
        }
    }
}

impl TryFrom<i32> for CollateralEventType {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::AssetCollateralSupportAdded),
            1 => Ok(Self::AssetCollateralSupportRemoved),
            2 => Ok(Self::AssetCollateralActivated),
            3 => Ok(Self::AssetCollateralDeactivated),
            4 => Ok(Self::NewLoanToValue),
            5 => Ok(Self::NewAnnualPercentageYield),
            6 => Ok(Self::NewLiquidationDiscount),
            7 => Ok(Self::NewMaxBorrowableAmount),
            8 => Ok(Self::AddedToWhitelist),
            9 => Ok(Self::RemovedFromWhitelist),
            10 => Ok(Self::BorrowFeeUpdated),
            _ => Err(crate::Error::UnknownEventType(v)),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum CollateralState {
    Activated,
    Deactivated,
    Removed,
}

impl CollateralState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Activated => "Activated",
            Self::Deactivated => "Deactivated",
            Self::Removed => "Removed",
        }
    }
}

impl TryFrom<i32> for CollateralState {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Activated),
            1 => Ok(Self::Deactivated),
            2 => Ok(Self::Removed),
            _ => Err(crate::Error::UnknownEventType(v)),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum LoanEventType {
    Borrowed,
    Repaid,
    PositionLiquidated,
    InterestSpread,
    BorrowFeeCharged,
    InterestAccrued,
    CollateralDeposited,
    CollateralWithdrawn,
}

impl LoanEventType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Borrowed => "Borrowed",
            Self::Repaid => "Repaid",
            Self::PositionLiquidated => "PositionLiquidated",
            Self::InterestSpread => "InterestSpread",
            Self::BorrowFeeCharged => "BorrowFeeCharged",
            Self::InterestAccrued => "InterestAccrued",
            Self::CollateralDeposited => "CollateralDeposited",
            Self::CollateralWithdrawn => "CollateralWithdrawn",
        }
    }
}

impl TryFrom<i32> for LoanEventType {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Borrowed),
            1 => Ok(Self::Repaid),
            2 => Ok(Self::PositionLiquidated),
            3 => Ok(Self::InterestSpread),
            4 => Ok(Self::BorrowFeeCharged),
            5 => Ok(Self::InterestAccrued),
            6 => Ok(Self::CollateralDeposited),
            7 => Ok(Self::CollateralWithdrawn),
            _ => Err(crate::Error::UnknownEventType(v)),
        }
    }
}
