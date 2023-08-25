use crate::msg::{BalancesResp, QueryMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Balances { address, tokens } => to_binary(&query::balances(deps, address, tokens)?),
    }
}

mod query {
    use cosmwasm_std::{Addr, StdError, Uint128};

    use super::*;

    pub fn balances(deps: Deps, user_address: Addr, tokens: Vec<Addr>) -> StdResult<BalancesResp> {
        let result = tokens.iter().map(|contract_addr| {
            let res: cw20::BalanceResponse = deps
                .querier
                .query_wasm_smart(
                    contract_addr.clone(),
                    &cw20::Cw20QueryMsg::Balance {
                        address: user_address.to_string(),
                    },
                )
                .unwrap_or_else(|e| match e {
                    StdError::NotFound { .. } => cw20::BalanceResponse {
                        balance: Uint128::zero(),
                    },
                    // _ => panic!("Unexpected error: {}", e),
                    _ => cw20::BalanceResponse {
                        balance: Uint128::zero(),
                    },
                });
            return res.balance;
        });
        Ok(BalancesResp {
            balances: result.collect(),
        })
    }
}

#[allow(dead_code)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn balances_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        // Test no contracts
        let resp: BalancesResp = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::Balances {
                    // TODO: Update address
                    address: Addr::unchecked("input"),
                    tokens: vec![],
                },
            )
            .unwrap();
        assert_eq!(resp, BalancesResp { balances: vec![] });

        // TODO: Test case for contracts
    }
}
