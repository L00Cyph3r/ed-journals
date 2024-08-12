use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::logs::cargo_event::CargoEventVessel;
use crate::modules::cargo::models::cargo_entry::CargoEntry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub vessel: CargoEventVessel,
    pub count: u16,
    pub inventory: Vec<CargoEntry>,
}
