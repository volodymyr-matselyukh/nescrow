use near_sdk::{testing_env, NearToken};

use crate::{tests::TEST_EMAIL, USER_REGISTRATION_STORAGE_USAGE_DEPOSIT};

use super::utils::setup;

#[test]
fn test_register_account() {
    let (mut contract, mut context) = setup(None, None);

    assert!(contract.deposits.get(TEST_EMAIL).is_none(), "Deposit should not exist");

    testing_env!(context.attached_deposit(NearToken::from_millinear(USER_REGISTRATION_STORAGE_USAGE_DEPOSIT)).build());
    contract.register_customer(TEST_EMAIL.to_string());

    assert!(contract.deposits.get(TEST_EMAIL).is_some(), "Deposit should exist");
}