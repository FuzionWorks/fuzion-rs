use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Admins, Contracts, FlowTypeConfigMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Admins,
    pub contracts: Contracts,
    pub flow_type_config: Vec<FlowTypeConfigMsg>,
}
