use near_sdk::AccountId;

use crate::Nescrow;

pub fn setup(
    contract_owner_account_id: Option<AccountId>,
    contract_predecessor_account_id: Option<AccountId>
) -> (Nescrow, VMContextBuilder) {
    let mut context = VMContextBuilder::new();

    let contract_owner_account_id = contract_owner_account_id.unwrap_or(owner());
    context.current_account_id(contract_owner_account_id.clone());
    //setting predecessor to owner to simulate contract deployment.
    //with contract deployment the predecessor will be equal to current account.
    context.predecessor_account_id(contract_owner_account_id.clone());

    context.account_balance(50 * ONE_NEAR);

    testing_env!(context.build());

    let mut contract = Contract::new();

    //now, after the contract has been deployed we can switch predecessor to whatever our test requires.
    context.predecessor_account_id(contract_predecessor_account_id.unwrap_or(owner()));

    testing_env!(context.build());

    (contract, context, quest)
}