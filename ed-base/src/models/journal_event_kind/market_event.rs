use crate::models::journal_event_kind::shared::station::station_type::StationType;
use serde::Deserialize;

/// Market data is written to a separate file called `market.json`.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MarketEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub station_type: StationType,
    pub star_system: String,
}
