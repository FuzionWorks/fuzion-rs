use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Admins, ConfigMsg, ExtendedFlowSchedule, FlowType, FlowTypeConfigMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    PendingConfig {},
    Flow {
        id: u64,
    },
    FlowList {
        start_after: Option<u64>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
    FlowMakerList {
        maker: Addr,
        start_after: Option<u64>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
    FlowTakerList {
        taker: Addr,
        start_after: Option<u64>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
    FlowDenomList {
        denom: String,
        start_after: Option<u64>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
    FlowTemplate {
        id: u64,
    },
    FlowTemplateList {
        start_after: Option<u64>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
    FlowTemplateMakerList {
        maker: Addr,
        start_after: Option<u64>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
    Claimable {
        id: u64,
    },
    /// Statistics
    Statistics {
        stats_key: String,
        stats_key_type: String,
        height: Option<u64>,
    },
    /// Whitelist Paged list
    StatisticsList {
        stats_key: Option<String>,
        start_after: Option<(String, String)>,
        limit: Option<u32>,
        sort_asc: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub admins: Admins,
    pub flow_type_config: Vec<FlowTypeConfigMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingConfigResponse {
    pub current_config: ConfigMsg,
    pub pending_config: Option<ConfigMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExtendedFlowResponse {
    pub id: u64,
    pub flow_type: FlowType,
    pub maker: Addr,
    pub taker: Addr,
    pub denom: String,
    pub identifier: Option<String>,
    pub genesis_time: u64,
    pub schedules: Vec<ExtendedFlowSchedule>,
    pub last_claim_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimableAmountResponse {
    pub id: u64,
    pub flow_type: FlowType,
    pub denom: String,
    pub taker: Addr,
    pub claimable_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct StatisticsResponse {
    pub stats_key: String,
    pub stats_key_type: String,
    pub stats_value: Option<Uint128>,
}
