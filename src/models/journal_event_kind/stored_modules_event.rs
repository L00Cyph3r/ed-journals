use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
    pub items: Vec<StoredModulesEventItem>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEventItem {
    // TODO look into replacing this with an enum
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub storage_slot: u16,

    #[serde(default)]
    pub in_transit: bool,

    /// This is [None] when the module is in transit. [in_transit] will be true.
    #[serde(flatten)]
    pub storage_location: Option<StoredModulesEventStorageLocation>,

    pub buy_price: u64,
    pub hot: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEventStorageLocation {
    pub star_system: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub transfer_cost: u64,
    pub transfer_time: u32,
}
