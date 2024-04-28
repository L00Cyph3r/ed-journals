use crate::models::journal_event_kind::shared::ship::ship_module::ShipModule;
use crate::models::journal_event_kind::shared::ship::ship_slot::ShipSlot;
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub slot: ShipSlot,
    pub sell_item: ShipModule,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localized: String,
    pub sell_price: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}
