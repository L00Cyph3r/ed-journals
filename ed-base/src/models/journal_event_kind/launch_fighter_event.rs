use crate::models::journal_event_kind::shared::ship::fighter_loadout::FighterLoadout;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}
