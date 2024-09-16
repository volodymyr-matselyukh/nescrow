use std::str::FromStr;

use near_sdk::AccountId;

pub mod register_customer_tests;
pub mod setup_tests;
pub mod ft_on_transfer_tests;
pub mod get_withdrawable_amount_tests;
pub mod tasks_tests;

pub mod utils;

const TEST_EMAIL: &str = "test@gmail.com";

fn owner() -> AccountId {
    AccountId::from_str("owner.testnet").unwrap()
}

fn account_1() -> AccountId {
    AccountId::from_str("account_1.testnet").unwrap()
}

fn account_2() -> AccountId {
    AccountId::from_str("account_2.testnet").unwrap()
}

fn usdt_account() -> AccountId {
    AccountId::from_str("usdt.fakes.testnet").unwrap()
}