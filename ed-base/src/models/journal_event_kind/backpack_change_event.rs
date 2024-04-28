use serde::Deserialize;
use crate::models::journal_event_kind::shared::odyssey::item::Item;
use crate::models::journal_event_kind::shared::odyssey::item_type::ItemType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEvent {
    #[serde(default)]
    pub added: Vec<BackpackChangeEventItem>,

    #[serde(default)]
    pub removed: Vec<BackpackChangeEventItem>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BackpackChangeEventItem {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,

    #[serde(rename = "Type")]
    pub kind: ItemType,
}
