use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ThargoidWar {
    pub current_state: ThargoidWarState,
    pub next_state_success: ThargoidWarState,
    pub next_state_failure: ThargoidWarState,
    pub success_state_reached: bool,
    pub war_progress: f32,
    pub remaining_ports: u8,

    // TODO parse this to a special struct
    // TODO check when this is [None]
    pub estimated_remaining_time: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ThargoidWarState {
    #[serde(rename = "Thargoid_Stronghold")]
    Stronghold,

    #[serde(rename = "Thargoid_Probing")]
    Probing,

    #[serde(rename = "Thargoid_Controlled")]
    Controlled,

    #[serde(rename = "Thargoid_Recovery")]
    Recovery,

    #[serde(rename = "")]
    Unspecified,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
