#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw0::maybe_addr;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, FeederResponse, InstantiateMsg, PriceResponse, QueryMsg};
use crate::state::{State, ADMIN, PRICES, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // set admin of the contract, which has the authority to change the whitelisted feeder
    ADMIN.set(deps.branch(), Some(info.sender.clone()))?;

    // save the current allowed feeder addr
    let state = State {
        feeder: deps.api.addr_validate(&msg.feeder)?,
    };
    STATE.save(deps.storage, &state)?;

    // initialize symbols in the map
    // after this, only price feeds of these init symbols are allowed
    for symbol in msg.symbols.clone() {
        PRICES.save(deps.storage, &symbol, &0)?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("symbols", msg.symbols.join(",")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;
    match msg {
        ExecuteMsg::SetPrice { symbol, price } => execute_set_price(deps, info, symbol, price),
        ExecuteMsg::UpdateAdmin { addr } => {
            Ok(ADMIN.execute_update_admin(deps, info, maybe_addr(api, addr)?)?)
        }
        ExecuteMsg::UpdateFeeder { addr } => execute_update_feeder(deps, info, addr),
    }
}

pub fn execute_set_price(
    deps: DepsMut,
    info: MessageInfo,
    symbol: String,
    price: u64,
) -> Result<Response, ContractError> {
    // Check that the sender is the whitelisted feeder
    if STATE.load(deps.storage)?.feeder != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    // Save the new price data
    PRICES.save(deps.storage, &symbol, &price)?;

    Ok(Response::new()
        .add_attribute("method", "execute_set_price")
        .add_attribute("symbol", symbol)
        .add_attribute("price", format!("{}", price)))
}

pub fn execute_update_feeder(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {
    // Asset that the sender is the contract's owner
    if !ADMIN.is_admin(deps.as_ref(), &info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    // Update the feeder addr with the new validated addr
    let checked: Addr = deps.api.addr_validate(&addr)?;
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.feeder = checked;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("method", "execute_update_feeder")
        .add_attribute("feeder", addr))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice { symbol } => to_binary(&query_price(deps, symbol)?),
        QueryMsg::GetAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::GetFeeder {} => to_binary(&query_feeder(deps)?),
    }
}

fn query_price(deps: Deps, symbol: String) -> StdResult<PriceResponse> {
    // Load price of the given symbol
    let price = PRICES.load(deps.storage, &symbol)?;
    Ok(PriceResponse { price: price })
}

fn query_feeder(deps: Deps) -> StdResult<FeederResponse> {
    // Load state containing the feeder addr
    let state = STATE.load(deps.storage)?;
    Ok(FeederResponse {
        feeder: state.feeder.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    fn get_symbols() -> Vec<String> {
        vec!["BTC".to_string(), "ETH".to_string(), "LUNA".to_string()]
    }
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let symbols = get_symbols();
        let msg = InstantiateMsg {
            symbols: symbols.clone(),
            feeder: "feeder".into(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        for symbol in symbols {
            let res = query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::GetPrice { symbol: symbol },
            )
            .unwrap();
            let value: PriceResponse = from_binary(&res).unwrap();
            assert_eq!(0, value.price);
        }
    }

    #[test]
    fn set_price() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let symbols = get_symbols();
        let msg = InstantiateMsg {
            symbols: symbols.clone(),
            feeder: "feeder".into(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("feeder", &coins(2, "token"));
        let msg = ExecuteMsg::SetPrice {
            symbol: "BTC".into(),
            price: 46621000000000,
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetPrice {
                symbol: "BTC".into(),
            },
        )
        .unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(46621000000000, value.price);
    }

    #[test]
    fn feeder_authorization() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let symbols = get_symbols();
        let msg = InstantiateMsg {
            symbols: symbols.clone(),
            feeder: "feeder".into(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Update feeder
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateFeeder {
            addr: "feeder2".into(),
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // feeder is set to `feeder2`, hence `feeder` is now unauthorized
        let info = mock_info("feeder", &coins(2, "token"));
        let msg = ExecuteMsg::SetPrice {
            symbol: "BTC".into(),
            price: 46621000000000,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => (),
            _ => panic!("Must return unauthorized error"),
        }
    }
}
