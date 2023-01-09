/*
 * API
 *
 * Audius V1 API
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectedWallets {
    #[serde(rename = "erc_wallets")]
    pub erc_wallets: Vec<String>,
    #[serde(rename = "spl_wallets")]
    pub spl_wallets: Vec<String>,
}

impl ConnectedWallets {
    pub fn new(erc_wallets: Vec<String>, spl_wallets: Vec<String>) -> ConnectedWallets {
        ConnectedWallets {
            erc_wallets,
            spl_wallets,
        }
    }
}


