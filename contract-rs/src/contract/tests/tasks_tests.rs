use near_sdk::{json_types::U128, testing_env, NearToken};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    contract::{
        tests::{
            account_1, account_1_username, account_2, account_2_username, trusted_admin_account,
            trusted_admin_username, usdt_account, utils::setup,
        },
        utils::get_nescrow_beneficiary_contract,
        NESCROW_BENEFICIARY_USERNAME, USER_REGISTRATION_STORAGE_USAGE_DEPOSIT,
        USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT,
    },
    types::common_types::{UsdtBalance, UsdtBalanceExt},
};

#[test]
fn test_create_task() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

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
    let deposit = dec!(1055);

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    contract.register_customer(account_1_username(), account_1());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(deposit),
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
    let (mut contract, mut context) = setup(None, Some(account_1()));

    const TASK_1_ID: &str = "task_1";

    let reward = 1000;
    let deposit = dec!(1055);

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    contract.register_customer(account_1_username(), account_1());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(deposit),
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

    contract.tasks.get(TASK_1_ID).expect("Task should exist");

    contract.remove_task(String::from(TASK_1_ID));

    let task_1 = contract.tasks.get(TASK_1_ID);

    assert!(task_1.is_none(), "Task should not exist");

    assert!(
        contract.tasks_per_owner.get(&account_1()).unwrap().len() == 0,
        "Account 1 shouldn't have tasks as task owner"
    );

    assert!(
        contract.tasks_per_engineer.get(&account_2()).unwrap().len() == 0,
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
    let reward_plus_owners_fee = dec!(1055);

    contract.register_customer(account_1_username(), account_1());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(reward_plus_owners_fee),
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

    contract.sign_task_as_owner(TASK_1_ID.to_string(), String::from("hash"));

    let task_from_blockchain = contract.tasks.get(TASK_1_ID).expect("Task should exist");
    assert!(
        task_from_blockchain.signed_by_owner_on.is_some(),
        "Signed by owner should be Some"
    );
}

#[test]
#[should_panic(expected = "Operation forbidden. You must be an owner of the task.")]
fn test_sign_task_as_owner_wrong_owner() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";

    let reward = 1000;
    let reward_plus_owners_fee = dec!(1055);

    contract.register_customer(account_1_username(), account_1());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(reward_plus_owners_fee),
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

    testing_env!(context.predecessor_account_id(account_2()).build());

    contract.sign_task_as_owner(TASK_1_ID.to_string(), String::from(TASK_1_ID));
}

#[test]
#[should_panic(expected = "You have not enought deposit to cover the reward for this task.")]
fn test_create_task_with_not_enough_deposit() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";

    let reward = dec!(1000);
    let reward_plus_owners_fee = dec!(1054); // 1000 + 1000 * 0.05(dispute reservation) + 1000 * 0.01(nescrow fee) - 1(to make code throw an exception)

    contract.register_customer(account_1_username(), account_1());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    // ft_on_transfer is called by usdt contract only. So, here we convert human money to USDT contract money.
    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(reward_plus_owners_fee),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.create_task(
        String::from(TASK_1_ID),
        account_1_username(),
        account_2(),
        account_2_username(),
        reward,
    );

    testing_env!(context.predecessor_account_id(account_2()).build());

    contract.sign_task_as_owner(TASK_1_ID.to_string(), String::from(TASK_1_ID));
}

#[test]
fn test_deposits_assigned_correctly_after_task_approval() {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";

    let reward = dec!(1000);
    let reward_plus_owners_fee = dec!(1055); // 1000 + 1000 * 0.05(dispute reservation) + 1000 * 0.005(nescrow fee)

    contract.register_customer(account_1_username(), account_1());
    contract.register_customer(account_2_username(), account_2());
    contract.register_customer(
        String::from(NESCROW_BENEFICIARY_USERNAME),
        get_nescrow_beneficiary_contract(),
    );

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(reward_plus_owners_fee),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.create_task(
        String::from(TASK_1_ID),
        account_1_username(),
        account_2(),
        account_2_username(),
        reward,
    );

    let owner_deposit =
        contract.get_withdrawable_amount_by_account(account_1_username(), account_1());

    assert_eq!(owner_deposit, dec!(0), "Owner deposit should be zero");

    contract.sign_task_as_owner(TASK_1_ID.to_string(), String::from(TASK_1_ID));

    testing_env!(context.predecessor_account_id(account_2()).build());

    contract.sign_task_as_contractor(TASK_1_ID.to_string(), String::from(TASK_1_ID));

    contract.submit_work(TASK_1_ID.to_string());

    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.approve_task(TASK_1_ID.to_string());

    // check balances

    let expected_contractor_deposit = dec!(995); // 1000 - 5
    let expected_owner_deposit = dec!(50); // 1000 * 0.05(dispute reservation)
    let expected_nescrow_deposit = dec!(10); // owner_fee + contractor_fee

    let contractor_deposit =
        contract.get_withdrawable_amount_by_account(account_2_username(), account_2());
    let owner_deposit =
        contract.get_withdrawable_amount_by_account(account_1_username(), account_1());
    let nescrow_deposit = contract.get_withdrawable_amount_by_account(
        String::from(NESCROW_BENEFICIARY_USERNAME),
        get_nescrow_beneficiary_contract(),
    );

    assert_eq!(
        contractor_deposit, expected_contractor_deposit,
        "Contractor deposit should match"
    );
    assert_eq!(
        owner_deposit, expected_owner_deposit,
        "Owner deposit should match"
    );
    assert_eq!(
        nescrow_deposit, expected_nescrow_deposit,
        "Nescrow deposit should match"
    );
}

fn test_deposits_assigned_correctly_after_task_dispute_resolution(
    resolution_percentage: u8,
    expected_owner_deposit: Decimal,
    expected_contractor_deposit: Decimal,
    expected_nescrow_admin_deposit: Decimal,
    expected_nescrow_platform_deposit: Decimal,
) {
    let (mut contract, mut context) = setup(None, Some(account_1()));

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT
        ))
        .build());

    const TASK_1_ID: &str = "task_1";

    let reward = dec!(1000);
    let reward_plus_owners_fee = dec!(1055); // 1000 + 1000 * 0.05(dispute reservation) + 1000 * 0.005(nescrow fee)

    contract.register_customer(account_1_username(), account_1());
    contract.register_customer(account_2_username(), account_2());
    contract.register_customer(
        String::from(NESCROW_BENEFICIARY_USERNAME),
        get_nescrow_beneficiary_contract(),
    );
    contract.register_customer(trusted_admin_username(), trusted_admin_account());

    testing_env!(context.predecessor_account_id(usdt_account()).build());

    contract.ft_on_transfer(
        &account_1(),
        UsdtBalance::from_human_to_usdt(reward_plus_owners_fee),
        String::from(format!("{{\"username\": \"{}\"}}", account_1_username())),
    );

    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.create_task(
        String::from(TASK_1_ID),
        account_1_username(),
        account_2(),
        account_2_username(),
        reward,
    );

    let owner_deposit =
        contract.get_withdrawable_amount_by_account(account_1_username(), account_1());

    assert_eq!(owner_deposit, dec!(0), "Owner deposit should be zero");

    contract.sign_task_as_owner(TASK_1_ID.to_string(), String::from(TASK_1_ID));

    testing_env!(context.predecessor_account_id(account_2()).build());

    contract.sign_task_as_contractor(TASK_1_ID.to_string(), String::from(TASK_1_ID));

    contract.submit_work(TASK_1_ID.to_string());

    // initiating dispute
    testing_env!(context.predecessor_account_id(account_1()).build());

    contract.initiate_dispute(TASK_1_ID.to_string());

    // resolve dispute
    testing_env!(context
        .predecessor_account_id(trusted_admin_account())
        .build());

    contract.resolve_dispute(
        TASK_1_ID.to_string(),
        trusted_admin_username(),
        resolution_percentage,
    );

    // check balances
    let contractor_deposit =
        contract.get_withdrawable_amount_by_account(account_2_username(), account_2());
    let owner_deposit =
        contract.get_withdrawable_amount_by_account(account_1_username(), account_1());
    let nescrow_deposit = contract.get_withdrawable_amount_by_account(
        String::from(NESCROW_BENEFICIARY_USERNAME),
        get_nescrow_beneficiary_contract(),
    );
    let nescrow_admin_deposit = contract.get_withdrawable_amount_by_account(
        String::from(trusted_admin_username()),
        trusted_admin_account(),
    );

    assert_eq!(
        contractor_deposit, expected_contractor_deposit,
        "Contractor deposit should match"
    );
    assert_eq!(
        owner_deposit, expected_owner_deposit,
        "Owner deposit should match"
    );
    assert_eq!(
        nescrow_deposit, expected_nescrow_platform_deposit,
        "Nescrow deposit should match"
    );
    assert_eq!(
        nescrow_admin_deposit, expected_nescrow_admin_deposit,
        "Nescrow admin deposit should match"
    );
}

#[test]
fn test_80_percentage_resolution() {
    test_deposits_assigned_correctly_after_task_dispute_resolution(
        80,
        dec!(200), // 200(1000 - 1000 * 80 / 100 - resolution percentage for owner)
        dec!(745), // 800(1000 * 80 / 100 - resolution percentage for contractor)
        dec!(100), // 50(dispute fee from owner) + 50(dispute fee from contractor)
        dec!(10),  // 5(owner_fee) + 5(contractor_fee)
    );
}

#[test]
fn test_0_percentage_resolution() {
    test_deposits_assigned_correctly_after_task_dispute_resolution(
        0,
        dec!(945), // 945(1000 - 50(1000 * 0.05 - dispute fee from contractor) - 5(1000 * 0.005))
        dec!(0),
        dec!(100), // 50(dispute fee from owner) + 50(dispute fee from contractor)
        dec!(10), // 5(owner_fee) + 5(contractor_fee)
    );
}

#[test]
fn test_3_percentage_resolution() {
    test_deposits_assigned_correctly_after_task_dispute_resolution(
        3,
        dec!(945), // 945(1000 - 30(1000 * 0.03) - 20(1000 * 0.05 - 30 - dispute fee from contractor plus the rest from owner) - 5(1000 * 0.005))
        dec!(0),
        dec!(100), // 50(dispute fee from owner) + 50(dispute fee from contractor)
        dec!(10), // 5(owner_fee) + 5(contractor_fee)
    );
}

#[test]
fn test_5_percentage_resolution() {
    test_deposits_assigned_correctly_after_task_dispute_resolution(
        5,
        dec!(945), // 945(1000 - 50(1000 * 0.05) - 1000 - 5(1000 * 0.005) dispute fee from contractor
        dec!(0),
        dec!(100), // 50(dispute fee from owner) + 50(dispute fee from contractor)
        dec!(10), // 5(owner_fee) + 5(contractor_fee)
    );
}

#[test]
fn test_6_percentage_resolution() {
    test_deposits_assigned_correctly_after_task_dispute_resolution(
        6,
        dec!(940), // 940(1000 - 60(1000 * 0.05) dispute fee from contractor
        dec!(5), // 10(1000 * 0.06 - 1000 * 0.05) contractor's deposit minus fees
        dec!(100), // 50(dispute fee from owner) + 50(dispute fee from contractor)
        dec!(10), // 5(owner_fee) + 5(contractor_fee)
    );
}
