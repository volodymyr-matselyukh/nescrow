use near_sdk::{testing_env, NearToken};

use crate::contract::{tests::{account_1, TEST_USERNAME}, USER_REGISTRATION_STORAGE_USAGE_DEPOSIT};

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
