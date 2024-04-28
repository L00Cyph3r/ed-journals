use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierBankTransferEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub deposit: u64,
    pub player_balance: u64,
    pub carrier_balance: u64,
}
