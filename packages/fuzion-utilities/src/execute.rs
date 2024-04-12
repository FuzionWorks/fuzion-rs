use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Change the admin
    UpdateAdmin {
        admin: Option<String>,
    },
    /// Upload Chain Info from chain-registry
    UploadChain {
        chain_info: Box<ChainInfo>,
        asset_list: Option<AssetList>,
    },
    /// Upload Chain Info from chain-registry
    UploadChains {
        chain_list: Vec<ChainsAndAssets>,
    },
    /// Upload Chain Info from chain-registry
    UploadAsset {
        asset: AssetList,
    },
    /// Upload Chain Info from chain-registry
    UploadAssets {
        asset_list: Vec<AssetList>,
    },
    /// Delete asset from chain-registry
    DeleteAsset {
        chain_name: String,
        base: String,
    },
    /// Upload IBC Path  
    UploadIbc {
        ibc_path: IBCPath,
    },
    UploadIbcs {
        ibc_path_list: Vec<IBCPath>,
    },
    UploadIbcDenoms {
        ibc_denom_list: Vec<IBCPathDenomUpload>,
    },
    UploadTotalSupply {
        total_supply_list: Vec<Coin>,
    },
    UploadCuratedDenoms {
        curated_denom_list: Vec<Coin>,
    },
    /// Delete asset from chain-registry
    DeleteCuratedDenom {
        denom: String,
    },
    UploadFuzionChainConfig {
        fuzion_chain_config_list: Vec<FuzionChainConfig>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct ChainsAndAssets {
    pub chain_info: ChainInfo,
    pub asset_list: Option<AssetList>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct ChainInfo {
    pub chain_name: String,
    pub status: String,
    pub network_type: String,
    pub pretty_name: String,
    pub chain_id: String,
    pub bech32_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_home: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slip44: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genesis: Option<Genesis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codebase: Option<Codebase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Peers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Apis>,
    #[serde(rename = "logo_URIs", skip_serializing_if = "Option::is_none")]
    pub logo_uris: Option<LogoURIs>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Genesis {
    pub genesis_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Codebase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_version: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::<String>::new")]
    pub compatible_versions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Peers {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::<Seed>::new")]
    pub seeds: Vec<Seed>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default = "Vec::<PersistentPeer>::new"
    )]
    pub persistent_peers: Vec<PersistentPeer>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Seed {
    pub id: String,
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct PersistentPeer {
    pub id: String,
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Apis {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::<Rpc>::new")]
    pub rpc: Vec<Rpc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::<Rest>::new")]
    pub rest: Vec<Rest>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::<Grpc>::new")]
    pub grpc: Vec<Grpc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Rpc {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Rest {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Grpc {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct AssetList {
    pub chain_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub name: String,
    pub display: String,
    pub symbol: String,
    #[serde(rename = "logo_URIs", skip_serializing_if = "Option::is_none")]
    pub logo_uris: Option<LogoURIs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coingecko_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct LogoURIs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]

pub struct IBCPath {
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(alias = "chain-1")]
    pub chain_1: Chain1,
    #[serde(alias = "chain-2")]
    pub chain_2: Chain2,
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]

pub struct IBCPathDenom {
    pub hash: String,
    pub path: String,
    pub base_denom: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct IBCPathDenomUpload {
    pub path: String,
    pub base_denom: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FuzionChainConfig {
    pub chain_name: String,
    pub chain_display_name: String,
    pub chain_id: String,
    pub chain_lcd_url: String,
    pub chain_rpc_url: String,
    pub connect_type: String,
    pub network_type: String,
    pub chain_api_urls: Vec<ChainApiUrls>,
    pub chain_contracts: Vec<FuzionContractsConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]

pub struct Chain1 {
    #[serde(alias = "chain-name")]
    pub chain_name: String,
    #[serde(alias = "client-id")]
    pub client_id: String,
    #[serde(alias = "connection-id")]
    pub connection_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct Chain2 {
    #[serde(alias = "chain-name")]
    pub chain_name: String,
    #[serde(alias = "client-id")]
    pub client_id: String,
    #[serde(alias = "connection-id")]
    pub connection_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]

pub struct Channel {
    #[serde(alias = "chain-1")]
    pub chain_1: ChannelChain1,
    #[serde(alias = "chain-2")]
    pub chain_2: ChannelChain2,
    pub ordering: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]

pub struct ChannelChain1 {
    #[serde(alias = "channel-id")]
    pub channel_id: String,
    #[serde(alias = "port-id")]
    pub port_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub struct ChannelChain2 {
    #[serde(alias = "channel-id")]
    pub channel_id: String,
    #[serde(alias = "port-id")]
    pub port_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]

pub struct Tags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Represents an IBC path tag
pub enum Tag {
    Dex(String),
    Preferred(bool),
    Properties(String),
    Status(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FuzionContractsConfig {
    pub contract_name: String,
    pub contract_address: Addr,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainApiUrls {
    pub api_name: String,
    pub api_url: String,
}
