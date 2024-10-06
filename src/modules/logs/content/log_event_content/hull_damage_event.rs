use serde::{Deserialize, Serialize};

/// Fired when the player received hull damage.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct HullDamageEvent {
    /// The current health of the ship.
    pub health: f32,
    pub player_pilot: bool,

    // TODO sometimes this is missing. Should check if it missing has special meaning
    #[serde(default)]
    pub fighter: bool,
}
