use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub counter: u64,
    pub minimal_donation: Coin,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
