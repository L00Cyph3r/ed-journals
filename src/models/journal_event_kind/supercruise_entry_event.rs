use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseEntryEvent {
    pub star_system: String,
    pub system_address: u64,

    #[serde(default)]
    pub taxi: bool,

    #[serde(default)]
    pub multicrew: bool,
}
