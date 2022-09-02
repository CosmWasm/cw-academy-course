use cosmwasm_std::{Coin, DepsMut, MessageInfo, Response, StdResult};

use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

pub fn instantiate(
    deps: DepsMut,
    info: MessageInfo,
    counter: u64,
    minimal_donation: Coin,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
    MINIMAL_DONATION.save(deps.storage, &minimal_donation)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::msg::ValueResp;
    use crate::state::COUNTER;

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod exec {
    use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult};

    use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if minimal_donation.amount.is_zero()
            || info.funds.iter().any(|coin| {
                coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
            })
        {
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;
        }

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, counter: u64) -> StdResult<Response> {
        COUNTER.save(deps.storage, &counter)?;

        let resp = Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balance,
        };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }
}
