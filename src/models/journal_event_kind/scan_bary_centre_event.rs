use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanBaryCentreEvent {
    pub star_system: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    pub ascending_node: f32,
    pub mean_anomaly: f32,
}
