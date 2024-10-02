use near_sdk::{testing_env, NearToken};

use crate::{
    contract::tests::{account_1, account_2, usdt_account, utils::setup, TEST_USERNAME},
    contract::USER_REGISTRATION_STORAGE_USAGE_DEPOSIT,
    types::common_types::{UsdtBalance, UsdtBalanceExt},
};

#[test]
fn test_create_task() {
    let (mut contract, _) = setup(None, Some(account_1()));

    const TASK_1_ID: &str = "task_1";

    let task_1 = contract.tasks.get(TASK_1_ID);

    assert!(task_1.is_none(), "Task should not exist");

    assert!(
        contract.tasks_per_owner.get(&account_1()).is_none(),
        "Account 1 shouldn't have tasks as task owner"
    );

    assert!(
        contract.tasks_per_engineer.get(&account_2()).is_none(),
        "Account 2 shouldn't have tasks as task contractor "
    );

    let reward = 1000;

    contract.create_task(
        String::from(TASK_1_ID),
        account_2(),
        UsdtBalance::from_usdt(reward),
    );

    let task_from_blockchain = contract.tasks.get(TASK_1_ID).expect("Task should exist");
    assert_eq!(
        task_from_blockchain.reward,
        UsdtBalance::from_usdt(reward),
        "Reward should be 1000"
    );

    contract
        .tasks_per_owner
        .get(&account_1())
        .expect("Account 1 should have task as task owner");

    contract
        .tasks_per_engineer
        .get(&account_2())
        .expect("Account 2 should have task an task contractor");
}

#[test]
fn test_remove_task() {
    let (mut contract, _) = setup(None, Some(account_1()));

    const TASK_1_ID: &str = "task_1";

    let reward = 1000;

    contract.create_task(
        String::from(TASK_1_ID),
        account_2(),
        UsdtBalance::from_usdt(reward),
    );

    contract.tasks.get(TASK_1_ID).expect("Task should exist");

    contract.remove_task(String::from(TASK_1_ID));

    let task_1 = contract.tasks.get(TASK_1_ID);

    assert!(task_1.is_none(), "Task should not exist");

    assert!(
        contract.tasks_per_owner.get(&account_1()).is_none(),
        "Account 1 shouldn't have tasks as task owner"
    );

    assert!(
        contract.tasks_per_engineer.get(&account_2()).is_none(),
        "Account 2 shouldn't have tasks as task contractor "
    );
}

#[test]
fn test_sign_task_as_owner() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";

    let reward = 1000;
    let reward_plus_owners_fee = 1005;

    contract.create_task(
        String::from(TASK_1_ID),
        account_2(),
        UsdtBalance::from_usdt(reward),
    );

    contract.register_customer(TEST_USERNAME.to_string());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_usdt(reward_plus_owners_fee),
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.sign_task_as_owner(TEST_USERNAME.to_string(), String::from(TASK_1_ID));

    let task_from_blockchain = contract.tasks.get(TASK_1_ID).expect("Task should exist");
    assert!(
        task_from_blockchain.signed_by_owner_on.is_some(),
        "Signed by owner should be Some"
    );
}

#[test]
#[should_panic(expected = "Operation forbidden. You must be an owner of the task.")]
fn test_sign_task_as_owner_wront_owner() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";

    let reward = 1000;
    let reward_plus_owners_fee = 1005;

    contract.create_task(
        String::from(TASK_1_ID),
        account_2(),
        UsdtBalance::from_usdt(reward),
    );

    contract.register_customer(TEST_USERNAME.to_string());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_usdt(reward_plus_owners_fee),
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    testing_env!(context.predecessor_account_id(account_2()).build());

    contract.sign_task_as_owner(TEST_USERNAME.to_string(), String::from(TASK_1_ID));
}
