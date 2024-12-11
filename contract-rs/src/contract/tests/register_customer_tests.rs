use near_sdk::{testing_env, NearToken};

use crate::contract::{
    tests::{account_1, account_1_username, account_2, TEST_USERNAME},
    USER_REGISTRATION_STORAGE_USAGE_DEPOSIT,
};

use super::utils::setup;

#[test]
fn test_register_account() {
    let (mut contract, mut context) = setup(None, None);

    assert!(
        contract.deposits.get(TEST_USERNAME).is_none(),
        "Deposit should not exist"
    );

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());
    contract.register_customer(TEST_USERNAME.to_string(), account_1());

    assert!(
        contract.deposits.get(TEST_USERNAME).is_some(),
        "Deposit should exist"
    );
}

#[test]
fn test_register_two_accounts() {
    let (mut contract, mut context) = setup(None, None);

    assert!(
        contract.deposits.get(&account_1_username()).is_none(),
        "Deposit should not exist"
    );

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    contract.register_customer(account_1_username().to_string(), account_1());

    let keys_len = contract
        .deposits
        .get(&account_1_username())
        .expect("Deposits should exist for account_1")
        .keys()
        .len();

    assert!(keys_len == 1, "One account should be registered");

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    contract.register_customer(account_1_username().to_string(), account_2());

    let keys_len = contract
        .deposits
        .get(&account_1_username())
        .expect("Deposits should exist for account_1")
        .keys()
        .len();

    assert!(keys_len == 2, "Two accounts should be registered");
}
