use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return AdminResponse
    Admin {},
    /// Retrieve a chain
    Chain { chain_name: String },
    /// Retrieve all chains
    ListChains {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Retrieve an asset
    Asset { symbol: String, chain_name: String },
    /// Retrieve an asset
    AssetByDenom { denom: String },
    /// Retrieve all assets
    ListAssets {
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve an asset by base
    ListAssetByBase {
        base: String,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve an asset by Display
    ListAssetByDisplay {
        display: String,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve an asset by Display
    ListAssetBySymbol {
        symbol: String,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve an asset by IBC Hash
    ListAssetByIbcHash {
        hash: String,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve an asset by IBC Path and Base Denom
    ListAssetByIbcPathAndBaseDenom {
        path: String,
        base_denom: String,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve an asset by Total Supply
    ListAssetByTotalSupply {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Retrieve Assets by Curated Denoms
    ListAssetByCuratedDenom {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Retrieve an ibc path
    IbcPath { chain_1: String, chain_2: String },
    /// Retrieve all ibc paths
    ListIbcPaths {
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Retrieve all ibc paths
    ListIbcPathDenoms {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Retrieve total supply
    ListTotalSupply {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Retrieve Assets by Curated Denoms
    ListCuratedDenoms {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Retrieve Last Change of Curated Denoms
    CuratedDenomsLastChange {},
    /// Retrieve Fuzion Chain Config
    FuzionChainConfig {
        chain_name: String,
        network_type: String,
    },
    /// Retrieve Chain Config List
    ListFuzionChainConfig {
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
}
