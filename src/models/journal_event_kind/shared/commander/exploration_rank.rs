use serde::Deserialize;
use thiserror::Error;
use crate::models::journal_event_kind::shared::commander::exobiology_rank::{ExobiologyRank, ExobiologyRankError};
use crate::try_from_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ExplorationRank {
    Aimless,
    MostlyAimless,
    Scout,
    Surveyor,
    Trailblazer,
    Pathfinder,
    Ranger,
    Pioneer,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,

    #[cfg(not(feature = "strict"))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum ExplorationRankError {
    #[error("Unknown exploration rank with id '{0}'")]
    UnknownExplorationRank(u8),
}

impl TryFrom<u8> for ExplorationRank {
    type Error = ExplorationRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExplorationRank::Aimless),
            1 => Ok(ExplorationRank::MostlyAimless),
            2 => Ok(ExplorationRank::Scout),
            3 => Ok(ExplorationRank::Surveyor),
            4 => Ok(ExplorationRank::Trailblazer),
            5 => Ok(ExplorationRank::Pathfinder),
            6 => Ok(ExplorationRank::Ranger),
            7 => Ok(ExplorationRank::Pioneer),
            8 => Ok(ExplorationRank::Elite),
            9 => Ok(ExplorationRank::EliteI),
            10 => Ok(ExplorationRank::EliteII),
            11 => Ok(ExplorationRank::EliteIII),
            12 => Ok(ExplorationRank::EliteIV),
            13 => Ok(ExplorationRank::EliteV),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ExplorationRank::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(ExplorationRankError::UnknownExplorationRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => ExplorationRank);

