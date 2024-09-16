use crate::{
    tests::{account_1, account_2, utils::setup},
    types::common_types::{UsdtBalance, UsdtBalanceExt},
};

#[test]
fn test_create_task() {
    let (mut contract, _) = setup(None, Some(account_1()));

    const TASK_1_ID: &str = "task_1";

    let task_1 = contract.tasks.get(TASK_1_ID);

    assert!(task_1.is_none(), "Task should not exist");

    contract.create_task(
        String::from(TASK_1_ID),
        account_2(),
        UsdtBalance::from_usdt(1000),
    );

    contract.tasks.get(TASK_1_ID).expect("Task should exist");
    contract
        .tasks_per_owner
        .get(&account_1())
        .expect("Account 1 should have tasks");
}
