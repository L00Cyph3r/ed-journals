use crate::models::journal_event_kind::shared::station::mission_type::MissionType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MissionRedirectedEvent {
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    pub name: MissionType,
    pub localised_name: String,
    pub new_destination_station: String,
    pub new_destination_system: String,
    pub old_destination_station: String,
    pub old_destination_system: String,
}
