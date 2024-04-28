use serde::Deserialize;
use crate::models::journal_event_kind::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ProspectedAsteroidEvent {
    pub materials: Vec<ProspectedAsteroidEventMaterial>,
    pub content: ProspectedAsteroidEventContent,

    pub motherlode_material: Option<Commodity>,

    #[serde(rename = "MotherlodeMaterial_Localised")]
    pub motherlode_material_localized: Option<String>,

    #[serde(rename = "Content_Localised")]
    pub content_localized: Option<String>,
    pub remaining: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ProspectedAsteroidEventMaterial {
    pub name: Commodity,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub proportion: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ProspectedAsteroidEventContent {
    #[serde(rename = "$AsteroidMaterialContent_High;")]
    HighMaterialContent,

    #[serde(rename = "$AsteroidMaterialContent_Low;")]
    LowMaterialContent,
}
