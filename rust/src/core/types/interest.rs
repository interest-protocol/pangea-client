#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
        }
    }
}

impl TryFrom<i32> for Side {
    type Error = crate::Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Side::Buy),
            1 => Ok(Side::Sell),
            _ => Err(crate::Error::UnknownEventType(v)),
        }
    }
}
