use std::collections::HashSet;
use near_sdk::store::{IterableMap, LookupMap};
use near_sdk::{near, AccountId, NearToken};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::enums::storage_keys::StorageKeys;
use crate::types::common_types::{TaskId, UsdtBalance};
use crate::types::task::Task;

pub mod customer_registration;
pub mod deposits_withdrawals;
pub mod tasks;
pub mod utils;

//no calculations performed, just guessing. This also includes gas for tasks approval.
const USER_REGISTRATION_STORAGE_USAGE_DEPOSIT: u128 = NearToken::from_millinear(10).as_yoctonear();
const USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT: u128 = NearToken::from_millinear(10).as_yoctonear();

const NESCROW_OWNER_FEE: Decimal = dec!(0.005);
const NESCROW_FREELANCER_FEE: Decimal = dec!(0.005);
const NESCROW_DISPUTE_RESOLUTION_FEE: Decimal = dec!(0.05);

const NESCROW_BENEFICIARY_ACCOUNT_NAME: &str = "nescrow.testnet";

#[near(contract_state)]
pub struct Nescrow {
    deposits: LookupMap<String, IterableMap<AccountId, UsdtBalance>>, //user name as a root level key
    tasks: LookupMap<TaskId, Task>,
    tasks_per_owner: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_per_engineer: IterableMap<AccountId, HashSet<TaskId>>,
}

impl Default for Nescrow {
    fn default() -> Self {
        Self {
            deposits: LookupMap::new(StorageKeys::Deposits),
            tasks: LookupMap::new(StorageKeys::Tasks),
            tasks_per_owner: IterableMap::new(StorageKeys::TasksPerOwner),
            tasks_per_engineer: IterableMap::new(StorageKeys::TasksPerEngineer),
        }
    }
}

#[cfg(test)]
mod tests;
