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
pub struct InlineResponse2001 {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "txs", skip_serializing_if = "Option::is_none")]
    pub txs: Option<Vec<crate::models::TxDetails>>,
}

impl InlineResponse2001 {
    pub fn new() -> InlineResponse2001 {
        InlineResponse2001 {
            count: None,
            txs: None,
        }
    }
}


