use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DisembarkEvent {
    // TODO check when this is optional
    #[serde(rename = "ID")]
    pub id: Option<u32>,
    pub star_system: String,
    pub system_address: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u32,
    pub on_station: bool,
    pub on_planet: bool,

    // TODO probably only set when [on_station] is true.
    pub station_name: Option<String>,

    // TODO probably only set when [on_station] is true.
    pub station_type: Option<StationType>,

    // TODO probably only set when [on_station] is true.
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    #[serde(rename = "SRV")]
    pub srv: bool,

    #[serde(default)]
    pub taxi: bool,

    #[serde(default)]
    pub multicrew: bool,
}
