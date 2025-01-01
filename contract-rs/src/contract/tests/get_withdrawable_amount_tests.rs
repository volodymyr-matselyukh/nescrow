use near_sdk::{testing_env, NearToken};
use rust_decimal_macros::dec;

use crate::{
    contract::{tests::{account_1, account_1_username, account_2, account_2_username, usdt_account, utils::setup, TEST_USERNAME}, USER_REGISTRATION_STORAGE_USAGE_DEPOSIT},
    types::common_types::{UsdtBalance, UsdtBalanceExt},
};

#[test]
fn test_get_withdrawable_amount_for_multiple_wallets() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let human_deposit_from_wallet_1 = dec!(100);
    let human_deposit_from_wallet_2 = dec!(330);

    contract.register_customer(TEST_USERNAME.to_string(), account_1());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt( human_deposit_from_wallet_1),
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_2(),
        UsdtBalance::from_human_to_usdt(human_deposit_from_wallet_2),
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    let withdrawable_amount_for_wallet_1 =
        contract.get_withdrawable_amount_by_account(String::from(TEST_USERNAME), account_1());

    assert_eq!(
        withdrawable_amount_for_wallet_1, human_deposit_from_wallet_1,
        "Withdrawable amounts for wallet 1 should match"
    );

    let withdrawable_amount_for_wallet_2 =
        contract.get_withdrawable_amount_by_account(String::from(TEST_USERNAME), account_2());

    assert_eq!(
        withdrawable_amount_for_wallet_2, human_deposit_from_wallet_2,
        "Withdrawable amounts for wallet 2 should match"
    );
}

#[test]
fn test_get_withdrawable_amount_for_multiple_usernames() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let human_deposit_from_username_1 = dec!(100);
    let human_deposit_from_username_2 = dec!(330);

    contract.register_customer(account_1_username().to_string(), account_1());
    contract.register_customer(account_2_username().to_string(), account_1());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt( human_deposit_from_username_1),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(human_deposit_from_username_2),
        String::from(format!("{{\"username\": \"{}\"}}", account_2_username())),
    );

    let withdrawable_amount_for_wallet_1 =
        contract.get_withdrawable_amount_by_account(String::from(account_1_username()), account_1());

    assert_eq!(
        withdrawable_amount_for_wallet_1, human_deposit_from_username_1,
        "Withdrawable amounts for wallet 1 should match"
    );

    let withdrawable_amount_for_wallet_2 =
        contract.get_withdrawable_amount_by_account(String::from(account_2_username()), account_1());

    assert_eq!(
        withdrawable_amount_for_wallet_2, human_deposit_from_username_2,
        "Withdrawable amounts for wallet 2 should match"
    );
}

#[test]
fn test_get_withdrawable_amount_when_task_exist() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";
    let reward = 1000;
    let deposit = 1065;

    contract.register_customer(account_1_username(), account_1());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_usdt(deposit),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.create_task(
        String::from(TASK_1_ID),
        account_1_username(),
        account_2(),
        account_2_username(),
        UsdtBalance::from_usdt(reward),
    );

    let expected_withdrawable_amount = UsdtBalance::from_usdt(10);
    let withdrawable_amount_for_wallet_1 =
        contract.get_withdrawable_amount_by_account(account_1_username(), account_1());

    assert_eq!(
        expected_withdrawable_amount, withdrawable_amount_for_wallet_1,
        "Withdrawable amounts for wallet 1 is wrong"
    );
}

#[test]
fn test_get_total_deposit() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT * 2
        ))
        .build());

    let deposit1 = 1000;
    let deposit2 = 1500;

    contract.register_customer(account_1_username(), account_1());
    contract.register_customer(account_2_username(), account_2());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_usdt(deposit1),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

        contract.ft_on_transfer(
        &account_2(),
        UsdtBalance::from_usdt(deposit2),
        String::from(format!("{{\"username\": \"{}\"}}", account_2_username())),
    );

    let total_deposit = contract.get_total_deposit();
    let expected_total_deposit = UsdtBalance::from_usdt(deposit1 + deposit2);

    assert_eq!(
        total_deposit, expected_total_deposit,
        "Total deposit is wrong"
    );
}