pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::msg::ValueResp;
    use crate::state::COUNTER;

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}
