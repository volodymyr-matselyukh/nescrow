use near_sdk::{testing_env, NearToken, PromiseError};
use rust_decimal_macros::dec;

use crate::{
    contract::{
        tests::{
            account_1, account_1_username, account_2, usdt_account, utils::setup, TEST_USERNAME,
        },
        USER_REGISTRATION_STORAGE_USAGE_DEPOSIT,
    },
    types::common_types::{UsdtBalance, UsdtBalanceExt},
};

#[test]
fn test_ft_on_transfer() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let human_money_deposit = dec!(1);
    let usdt_deposit = UsdtBalance::from_human_to_usdt(human_money_deposit);

    contract.register_customer(TEST_USERNAME.to_string(), account_1());

    contract.ft_on_transfer(
        &account_1(),
        usdt_deposit,
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    let actual_deposit = contract.get_deposit_by_username(String::from(TEST_USERNAME));

    assert_eq!(actual_deposit, human_money_deposit, "Deposit should match");
}

#[test]
fn test_ft_on_transfer_multiple_wallets() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let usdt_deposit = dec!(1);

    contract.register_customer(TEST_USERNAME.to_string(), account_1());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(usdt_deposit),
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_2(),
        UsdtBalance::from_human_to_usdt(usdt_deposit),
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    let actual_deposit = contract.get_deposit_by_username(String::from(TEST_USERNAME));
    let expected_deposit = dec!(2);

    assert_eq!(actual_deposit, expected_deposit, "Deposit should match");
}

#[test]
fn test_ft_transfer_callback() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let usdt_deposit = dec!(100);

    contract.register_customer(account_1_username().to_string(), account_1());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(usdt_deposit),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

    let deposit_before_manipulations =
        contract.get_deposit_by_username(String::from(account_1_username()));
    let expected_deposit_before_manipulations = dec!(100);

    assert_eq!(
        deposit_before_manipulations, expected_deposit_before_manipulations,
        "Deposits before manipulations should match"
    );

    let withdrawal_ammount = dec!(40);
    let successful_promise: Result<(), PromiseError> = Ok(());

    // simulating withdrawal
    contract.ft_transfer_callback(
        successful_promise,
        account_1_username(),
        account_1(),
        withdrawal_ammount, // this method is called by USDT contract. USDT contract uses 6 digits numbers
    );

    let deposit_after_manipulations =
        contract.get_deposit_by_username(String::from(account_1_username()));
    let expected_deposit_after_manipulations = dec!(60);

    assert_eq!(
        deposit_after_manipulations, expected_deposit_after_manipulations,
        "Deposits after manipulations should match"
    );
}
