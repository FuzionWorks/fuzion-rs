use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct FeeCollector {
    pub address: Addr,
    pub percentage: u8,
}

#[cw_serde]
pub struct FeeReceiver {
    pub address: Addr,
    pub amount: Coin,
}

#[cw_serde]
pub struct FeeResponse {
    pub receiver_amount: Vec<Coin>,
    pub fee_receivers: Vec<FeeReceiver>,
}
