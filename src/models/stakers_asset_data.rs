/*
 * Midgard Public API
 *
 * The Midgard Public API queries THORChain and any chains linked via the Bifröst and prepares information about the network to be readily available for public users. The API parses transaction event data from THORChain and stores them in a time-series database to make time-dependent queries easy. Midgard does not hold critical information. To interact with BEPSwap and Asgardex, users should query THORChain directly.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * Contact: devs@thorchain.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StakersAssetData {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Represents ownership of a pool.
    #[serde(rename = "stakeUnits", skip_serializing_if = "Option::is_none")]
    pub stake_units: Option<String>,
    #[serde(rename = "dateFirstStaked", skip_serializing_if = "Option::is_none")]
    pub date_first_staked: Option<i64>,
    #[serde(rename = "heightLastStaked", skip_serializing_if = "Option::is_none")]
    pub height_last_staked: Option<i64>,
}

impl StakersAssetData {
    pub fn new() -> StakersAssetData {
        StakersAssetData {
            asset: None,
            stake_units: None,
            date_first_staked: None,
            height_last_staked: None,
        }
    }
}

