use cosmwasm_std::{Addr, Coin, Empty, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};
use crate::{execute, instantiate, migrate, query};

pub struct CountingContract(Addr);

impl CountingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query).with_migrate(migrate);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        admin: impl Into<Option<&'a Addr>>,
        counter: impl Into<Option<u64>>,
        minimal_donation: Coin,
    ) -> StdResult<Self> {
        let admin = admin.into();
        let counter = counter.into().unwrap_or_default();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                counter,
                minimal_donation,
            },
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(CountingContract)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn migrate(app: &mut App, contract: Addr, code_id: u64, sender: &Addr) -> StdResult<Self> {
        app.migrate_contract(sender.clone(), contract.clone(), &Empty {}, code_id)
            .map_err(|err| err.downcast().unwrap())
            .map(|_| Self(contract))
    }

    #[track_caller]
    pub fn donate(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Donate {}, funds)
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn reset(
        &self,
        app: &mut App,
        sender: &Addr,
        counter: impl Into<Option<u64>>,
    ) -> Result<(), ContractError> {
        let counter = counter.into().unwrap_or_default();
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecMsg::Reset { counter },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn withdraw(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Withdraw {}, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    #[track_caller]
    pub fn withdraw_to(
        &self,
        app: &mut App,
        sender: &Addr,
        receiver: &Addr,
        funds: impl Into<Option<Vec<Coin>>>,
    ) -> Result<(), ContractError> {
        let funds = funds.into().unwrap_or_default();
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecMsg::WithdrawTo {
                receiver: receiver.to_string(),
                funds,
            },
            &[],
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn query_value(&self, app: &App) -> StdResult<ValueResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Value {})
    }
}

impl From<CountingContract> for Addr {
    fn from(contract: CountingContract) -> Self {
        contract.0
    }
}
