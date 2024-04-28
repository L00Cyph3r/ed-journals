use serde::Deserialize;

use crate::models::journal_event_kind::shared::station::station_info::StationInfo;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ApproachSettlementEvent {
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(flatten)]
    pub station_info: Option<StationInfo>,

    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub body_name: String,
    pub latitude: f32,
    pub longitude: f32,
}
