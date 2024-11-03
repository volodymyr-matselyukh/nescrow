use near_sdk::{env, AccountId};

use super::NESCROW_BENEFICIARY_ACCOUNT_NAME;

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
