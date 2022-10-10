use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ysip::pair::PairInfo;

pub const FEE_SCALE_FACTOR: Uint128 = Uint128::new(10_000);
pub const FEE_DECIMAL_PRECISION: Uint128 = Uint128::new(10u128.pow(20));

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub pair_info: PairInfo,
    pub factory_addr: Addr,
    pub fees: Fees,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Fees {
    pub protocol_fee_recipient: Addr,
    pub protocol_fee_percent: Decimal,
    pub lp_fee_percent: Decimal,
}

pub const CONFIG: Item<Config> = Item::new("config");
