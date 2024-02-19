use std::fmt;

use fuzion_std::FeeCollector;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),
    ConfirmConfig {},
    CreateFlows {
        flow_list: Vec<FlowCreate>,
    },
    CreateFlowTemplate {
        flow_template: FlowTemplateCreate,
    },
    CreateFlowFromTemplate {
        template_id: u64,
        taker: Addr,
        identifier: Option<String>,
    },
    Claim {
        id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigMsg {
    pub admins: Admins,
    pub contracts: Contracts,
    pub flow_type_config: Vec<FlowTypeConfigMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateConfigMsg {
    pub admins: Option<Admins>,
    pub contracts: Option<Contracts>,
    pub flow_type_config: Option<Vec<FlowTypeConfigMsg>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Admins {
    pub config_admin: Addr,
    pub confirm_admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Contracts {
    pub oracle_price_contract: Addr,
    pub utilities_contract: Addr,
    pub funds_distributor_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FlowTypeConfigMsg {
    pub flow_type: FlowType,
    pub fee: Decimal,
    pub partner_fee: Decimal,
    pub fee_collectors: Vec<FeeCollector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FlowTemplateCreate {
    pub flow_type: FlowType,
    pub genesis_time: u64,
    pub maker: Option<Addr>,
    pub identifier: Option<u64>,
    pub schedules: Vec<FlowTemplateSchedule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FlowCreate {
    pub flow_type: FlowType,
    pub taker: Addr,
    pub denom: String,
    pub identifier: Option<String>,
    pub genesis_time: u64,
    pub schedules: Vec<FlowSchedule>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum FlowType {
    Vesting,
}

impl fmt::Display for FlowType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FlowTemplateSchedule {
    pub start_time: u64,
    pub end_time: u64,
    pub cliff_end_time: u64,
    pub percent: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FlowSchedule {
    pub start_time: u64,
    pub end_time: u64,
    pub cliff_end_time: u64,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExtendedFlowSchedule {
    pub start_time: u64,
    pub end_time: u64,
    pub cliff_end_time: u64,
    pub amount: Uint128,
    pub claimed: Uint128,
    pub claimable: Uint128,
}

impl ExtendedFlowSchedule {
    pub fn new(
        start_time: u64,
        end_time: u64,
        cliff_end_time: u64,
        amount: Uint128,
        claimed: Uint128,
        claimable: Uint128,
    ) -> Self {
        Self {
            start_time,
            end_time,
            cliff_end_time,
            amount,
            claimed,
            claimable,
        }
    }
}
