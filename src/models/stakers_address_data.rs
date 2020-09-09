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
pub struct StakersAddressData {
    #[serde(rename = "poolsArray", skip_serializing_if = "Option::is_none")]
    pub pools_array: Option<Vec<String>>,
    /// Total staked (in RUNE) across all pools.
    #[serde(rename = "totalStaked", skip_serializing_if = "Option::is_none")]
    pub total_staked: Option<String>,
    /// Total value of earnings (in RUNE) across all pools.
    #[serde(rename = "totalEarned", skip_serializing_if = "Option::is_none")]
    pub total_earned: Option<String>,
    /// Average of all pool ROIs.
    #[serde(rename = "totalROI", skip_serializing_if = "Option::is_none")]
    pub total_roi: Option<String>,
}

impl StakersAddressData {
    pub fn new() -> StakersAddressData {
        StakersAddressData {
            pools_array: None,
            total_staked: None,
            total_earned: None,
            total_roi: None,
        }
    }
}


