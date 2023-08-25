use cosmwasm_std::{Addr, Uint128};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct BalancesResp {
    pub balances: Vec<Uint128>
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalancesResp)]
    Balances {
        address: Addr,
        tokens: Vec<Addr>,
    },
}



