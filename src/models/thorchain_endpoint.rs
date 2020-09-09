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
pub struct ThorchainEndpoint {
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(rename = "pub_key", skip_serializing_if = "Option::is_none")]
    pub pub_key: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl ThorchainEndpoint {
    pub fn new() -> ThorchainEndpoint {
        ThorchainEndpoint {
            chain: None,
            pub_key: None,
            address: None,
        }
    }
}


