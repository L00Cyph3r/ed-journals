use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::shipyard::models::shipyard_entry::ShipyardEntry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Shipyard {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
    pub horizons: bool,

    #[serde(rename = "AllowCobraMkIV")]
    pub allow_cobra_mk_iv: bool,
    pub price_list: Vec<ShipyardEntry>,
}
