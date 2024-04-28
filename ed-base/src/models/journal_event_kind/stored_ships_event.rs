use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipsEvent {
    pub station_name: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub ships_here: Vec<StoredShipEventLocalShip>,
    pub ships_remote: Vec<StoredShipEventRemoteShip>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventLocalShip {
    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,
    pub name: Option<String>,
    pub value: u64,
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventRemoteShip {
    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,
    pub name: String,

    #[serde(default)]
    pub in_transit: bool,

    #[serde(flatten)]
    pub storage_location: Option<StoredShipEventStorageLocation>,

    pub value: u64,
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredShipEventStorageLocation {
    pub star_system: String,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,
    pub transfer_price: u64,
    pub transfer_time: u32,
}
