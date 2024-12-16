use near_sdk::{env, AccountId};
use rust_decimal::Decimal;

use crate::types::{common_types::UsdtBalance, task::Task};

use super::{NESCROW_BENEFICIARY_ACCOUNT_NAME, NESCROW_DISPUTE_RESOLUTION_FEE, NESCROW_OWNER_FEE};

pub fn get_usdt_contract() -> AccountId {
    let current_account_id = env::current_account_id();

    if current_account_id.to_string().ends_with(".testnet") {
        return "usdt.fakes.testnet".parse().unwrap();
    }

    if current_account_id.to_string().ends_with(".near") {
        return "usdt.near".parse().unwrap();
    }

    panic!("unknown network");
}

pub fn get_nescrow_beneficiary_contract() -> AccountId {
    return NESCROW_BENEFICIARY_ACCOUNT_NAME.parse().unwrap();
}

pub fn get_dispute_resolution_amount(reward: UsdtBalance) -> UsdtBalance {
    return NESCROW_DISPUTE_RESOLUTION_FEE * reward;
}

pub fn get_task_reserverd_amount(task: &Task) -> Decimal {
    let task_reward_with_fees = task.reward
        + task.reward * NESCROW_OWNER_FEE
        + task.reward * NESCROW_DISPUTE_RESOLUTION_FEE;

    return task_reward_with_fees;
}

pub fn get_trusted_admin_accounts() -> Vec<AccountId> {
    let current_account_id = env::current_account_id();

    if current_account_id.to_string().ends_with(".testnet") {
        return vec!["nescrow_admin.testnet".parse().unwrap()];
    }

    if current_account_id.to_string().ends_with(".near") {
        panic!("Not in production yet");
    }

    panic!("Unknown network");
}
