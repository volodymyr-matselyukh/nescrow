use near_sdk::{testing_env, NearToken};

use crate::{
    contract::USER_REGISTRATION_STORAGE_USAGE_DEPOSIT,
    contract::tests::{account_1, account_2, usdt_account, utils::setup, TEST_USERNAME},
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

    let usdt_deposit_from_wallet_1 = UsdtBalance::from_usdt(100);
    let usdt_deposit_from_wallet_2 = UsdtBalance::from_usdt(330);

    contract.register_customer(TEST_USERNAME.to_string());

    contract.ft_on_transfer(
        &account_1(),
        usdt_deposit_from_wallet_1,
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    contract.ft_on_transfer(
        &account_2(),
        usdt_deposit_from_wallet_2,
        String::from(format!("{{\"username\": \"{}\"}}", TEST_USERNAME)),
    );

    let withdrawable_amount_for_wallet_1 =
        contract.get_withdrawable_amount_by_account(String::from(TEST_USERNAME), account_1());

    assert_eq!(
        withdrawable_amount_for_wallet_1, usdt_deposit_from_wallet_1,
        "Withdrawable amounts for wallet 1 should match"
    );

    let withdrawable_amount_for_wallet_2 =
        contract.get_withdrawable_amount_by_account(String::from(TEST_USERNAME), account_2());

    assert_eq!(
        withdrawable_amount_for_wallet_2, usdt_deposit_from_wallet_2,
        "Withdrawable amounts for wallet 2 should match"
    );
}
