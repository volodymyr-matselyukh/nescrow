use near_sdk::{testing_env, NearToken};

use crate::{
    tests::{account_1, account_2, usdt_account, utils::setup, TEST_EMAIL},
    types::common_types::{UsdtBalance, UsdtBalanceExt},
    USER_REGISTRATION_STORAGE_USAGE_DEPOSIT,
};

#[test]
fn test_ft_on_transfer() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let usdt_deposit = UsdtBalance::from_usdt(1);

    contract.register_customer(TEST_EMAIL.to_string());

    contract.ft_on_transfer(
        &account_1(),
        usdt_deposit,
        String::from(format!("{{\"email\": \"{}\"}}", TEST_EMAIL)),
    );

    let deposit = contract.get_deposit_by_email(String::from(TEST_EMAIL));

    assert_eq!(deposit, usdt_deposit, "Deposit should match");
}

#[test]
fn test_ft_on_transfer_multiple_wallets() {
    let (mut contract, mut context) = setup(None, Some(usdt_account()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    let usdt_deposit = UsdtBalance::from_usdt(1);

    contract.register_customer(TEST_EMAIL.to_string());

    contract.ft_on_transfer(
        &account_1(),
        usdt_deposit,
        String::from(format!("{{\"email\": \"{}\"}}", TEST_EMAIL)),
    );

    contract.ft_on_transfer(
        &account_2(),
        usdt_deposit,
        String::from(format!("{{\"email\": \"{}\"}}", TEST_EMAIL)),
    );

    let actual_deposit = contract.get_deposit_by_email(String::from(TEST_EMAIL));
    let expected_deposit = UsdtBalance::from_usdt(2);   

    assert_eq!(actual_deposit, expected_deposit, "Deposit should match");
}
