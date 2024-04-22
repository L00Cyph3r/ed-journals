use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SellDronesEvent {
    #[serde(rename = "Type")]
    pub kind: SellDronesEventType,
    pub count: u16,
    pub sell_price: u64,
    pub total_sale: u64,
}

// TODO this is the same as buy drones
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum SellDronesEventType {
    Drones,
}
