use std::collections::HashSet;

use chrono::Utc;
use enums::storage_keys::StorageKeys;
use near_sdk::json_types::U128;
use near_sdk::store::{IterableMap, IterableSet, LookupMap};
use near_sdk::{env, log, near, AccountId, Gas, NearToken, Promise, PromiseError};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use types::borsh::date_utc::UtcDateTime;
use types::common_types::{TaskId, UsdtBalance, UsdtBalanceExt};
use types::ft_transfer_message::FtOnTransferMessage;
use types::task::Task;

mod constants;
mod enums;
mod types;

//no calculations performed, just guessing. This also includes gas for tasks approval.
const USER_REGISTRATION_STORAGE_USAGE_DEPOSIT: u128 = NearToken::from_millinear(10).as_yoctonear();

const NESCROW_OWNER_FEE: Decimal = dec!(0.5);
const NESCROW_FREELANCER_FEE: Decimal = dec!(0.5);

#[near(contract_state)]
struct Nescrow {
    deposits: LookupMap<String, IterableMap<AccountId, UsdtBalance>>, //email as a root level key
    tasks: LookupMap<TaskId, Task>,
    tasks_per_owner: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_per_engineer: IterableMap<AccountId, HashSet<TaskId>>,
}

impl Default for Nescrow {
    fn default() -> Self {
        Self {
            deposits: LookupMap::new(StorageKeys::Deposits),
            tasks: LookupMap::new(StorageKeys::Tasks),
            tasks_per_owner: IterableMap::new(StorageKeys::TasksPerOwner),
            tasks_per_engineer: IterableMap::new(StorageKeys::TasksPerEngineer),
        }
    }
}

#[near]
#[allow(dead_code)]
impl Nescrow {
    #[init]
    pub fn new() -> Self {
        Self {
            deposits: LookupMap::new(StorageKeys::Deposits),
            tasks: LookupMap::new(StorageKeys::Tasks),
            tasks_per_owner: IterableMap::new(StorageKeys::TasksPerOwner),
            tasks_per_engineer: IterableMap::new(StorageKeys::TasksPerEngineer),
        }
    }

    #[payable]
    pub fn register_customer(&mut self, email: String) {
        if String::is_empty(&email) {
            panic!("Email should be provided");
        }

        //TODO. Add email regex validation.

        if self.deposits.contains_key(&email) {
            panic!("Email already register");
        }

        let email_hash = env::sha256_array(&email.as_bytes());

        let account_balance_map = IterableMap::new(StorageKeys::AccountBalance { email_hash });

        self.deposits.insert(email, account_balance_map);

        let attached_deposit = env::attached_deposit();

        assert!(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT <= attached_deposit.as_yoctonear(),
            "Attached deposit should be >= {}",
            NearToken::from_yoctonear(USER_REGISTRATION_STORAGE_USAGE_DEPOSIT)
        );

        let refund = attached_deposit.as_yoctonear() - USER_REGISTRATION_STORAGE_USAGE_DEPOSIT;

        log!("Deposit to return {}", refund);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(NearToken::from_yoctonear(refund));
        }
    }

    pub fn is_registered(self, sender_email: String) -> bool {
        let is_email_registered = self.deposits.contains_key(&sender_email);

        return is_email_registered;
    }

    pub fn get_deposit_by_email(&self, sender_email: String) -> UsdtBalance {
        let deposits = self
            .deposits
            .get(&sender_email)
            .unwrap_or_else(|| panic!("Email not registered"));

        let mut total_balance: u128 = 0;

        deposits.iter().for_each(|(_, &balance)| {
            total_balance += balance.0;
        });

        return U128(total_balance);
    }

    pub fn get_withdrawable_amount_by_account(
        &self,
        sender_email: String,
        account_id: AccountId,
    ) -> UsdtBalance {
        let deposits = self
            .deposits
            .get(&sender_email)
            .unwrap_or_else(|| panic!("Email not registered"));

        let account_deposit = deposits.get(&account_id);

        match account_deposit {
            None => return UsdtBalance::from(0),
            Some(deposit) => return deposit.clone(),
        };
    }

    pub fn ft_on_transfer(
        &mut self,
        sender_id: &AccountId,
        amount: UsdtBalance,
        msg: String,
    ) -> UsdtBalance {
        let usdt_contract_id = Nescrow::get_usdt_contract();

        if usdt_contract_id != env::predecessor_account_id() {
            panic!("untrusted contract");
        }

        log!("ft_on_transfer called {} {:?} {}", sender_id, amount, msg);

        let parsed_message_result: Result<FtOnTransferMessage, near_sdk::serde_json::Error> =
            near_sdk::serde_json::from_str(&msg);

        if parsed_message_result.is_err() {
            panic!("Error parsing message");
        }

        let sender_email = parsed_message_result.unwrap().email;

        let sender_deposits = self
            .deposits
            .get_mut(&sender_email)
            .expect("Customer is not registered. Register the customer first.");

        let ammount_to_add: u128 = amount.into();

        let existing_deposit = sender_deposits.get(sender_id);

        match existing_deposit {
            None => sender_deposits.insert(sender_id.clone(), amount),
            Some(balance) => {
                sender_deposits.insert(sender_id.clone(), U128(balance.0 + ammount_to_add))
            }
        };

        return U128(0);
    }

    pub fn withdraw(&self, receiver_email: String, amount: UsdtBalance) -> Promise {
        let is_email_registered = self.deposits.contains_key(&receiver_email);

        assert!(is_email_registered, "Email is not registered");

        let receiver_account_id = env::predecessor_account_id();

        let withdrawable_amount = self.get_withdrawable_amount_by_account(
            receiver_email.clone(),
            receiver_account_id.clone(),
        );

        assert!(withdrawable_amount.0 > 0, "Nothing to withdraw");

        assert!(amount.0 > 0, "Amount should be positive value");

        assert!(
            amount.0 <= withdrawable_amount.0,
            "Max withdraw is {:#?}",
            UsdtBalance::to_usdt(withdrawable_amount)
        );

        let usdt_contract_id = Nescrow::get_usdt_contract();

        let ft_transfer_promise = Promise::new(usdt_contract_id).function_call(
            "ft_transfer".to_string(),
            near_sdk::serde_json::json!({
                "amount": amount.0.to_string(),
                "receiver_id": receiver_account_id.clone(),
            })
            .to_string()
            .into_bytes(),
            NearToken::from_yoctonear(1),
            Gas::from_tgas(3),
        );

        return ft_transfer_promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(3))
                .ft_transfer_callback(receiver_email.clone(), receiver_account_id, amount),
        );
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_transfer_callback(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        receiver_email: String,
        receiver_account_id: AccountId,
        amount: UsdtBalance,
    ) {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            panic!("There was an error during ft_transfer");
        }

        let sender_deposits = self
            .deposits
            .get_mut(&receiver_email)
            .expect("Customer is not registered. Register the customer first.");

        let ammount_to_deduct: u128 = amount.into();

        let existing_deposit = sender_deposits
            .get(&receiver_account_id)
            .unwrap_or_else(|| panic!("Deposit doesn't exist"));

        assert!(
            existing_deposit.0 >= ammount_to_deduct,
            "Amount to deduct is bigger then deposit"
        );

        sender_deposits.insert(
            receiver_account_id.clone(),
            U128(existing_deposit.0 - ammount_to_deduct),
        );
    }

    // the task is created when the owner accepts the contractor
    pub fn create_task(&mut self, task_id: TaskId, contractor: AccountId, reward: UsdtBalance) {
        assert!(
            !self.tasks.contains_key(&task_id),
            "Taks has already been created"
        );

        let task_owner = env::predecessor_account_id();

        let task = Task {
            contractor: contractor.clone(),
            owner: task_owner.clone(),
            reward,
            signed_by_contractor_on: None,
            signed_by_owner_on: None,
            submitted_by_contractor_on: None,
            approved_on: None,
            dispute_initiated_on: None,
            dispute_resolved_on: None,
            completion_percentage: None,
            claimed_by_contractor_on: None,
            claimed_by_owner_on: None,
        };

        self.tasks.insert(task_id.clone(), task);

        let existing_tasks_per_owner_unwrapped = self.tasks_per_owner.get_mut(&task_owner);
        if existing_tasks_per_owner_unwrapped.is_some() {
            existing_tasks_per_owner_unwrapped
                .unwrap()
                .insert(task_id.clone());
        } else {
            let mut new_tasks_per_owner = HashSet::new();
            new_tasks_per_owner.insert(task_id.clone());

            self.tasks_per_owner
                .insert(task_owner.clone(), new_tasks_per_owner);
        }

        let existing_tasks_per_engineer_unwrapped = self.tasks_per_engineer.get_mut(&task_owner);
        if existing_tasks_per_engineer_unwrapped.is_some() {
            existing_tasks_per_engineer_unwrapped
                .unwrap()
                .insert(task_id.clone());
        } else {
            let mut new_tasks_per_engineer = HashSet::new();
            new_tasks_per_engineer.insert(task_id.clone());

            self.tasks_per_engineer
                .insert(task_owner.clone(), new_tasks_per_engineer);
        }
    }

    // the task is removed when the owner decides to unaccept the contractor
    pub fn remove_task(mut self, task_id: TaskId) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_owner = env::predecessor_account_id();
        let task = self.tasks.remove(&task_id).expect("Task not found");

        assert_eq!(
            task.owner, task_owner,
            "Only task owner can remove the task"
        );
        assert!(
            task.signed_by_owner_on.is_none(),
            "The task is signed by owner. Unsign first before removal."
        );
        assert!(
            task.signed_by_contractor_on.is_none(),
            "The task is signed by contractor. Task removal is impossible."
        );

        self.tasks_per_owner.remove(&task_owner);
        self.tasks_per_engineer.remove(&task.contractor);
    }

    // the task is signed by owner when he is happy with the selected contractor and wants to proceed to work started
    pub fn sign_task_as_owner(mut self, owner_email: String, task_id: TaskId) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_owner_account_id = env::predecessor_account_id();

        let withdrawable_amount =
            self.get_withdrawable_amount_by_account(owner_email, task_owner_account_id.clone());

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_owner_account_id.clone(),
            task.owner,
            "Task has different owner."
        );

        assert!(
            task.signed_by_owner_on.is_none(),
            "Task is already signed by owner."
        );

        assert!(
            Decimal::from(withdrawable_amount.0)
                >= Decimal::from(task.reward.0) + Decimal::from(task.reward.0) * NESCROW_OWNER_FEE,
            "You have not enought deposit to cover the reward for this task."
        );

        task.signed_by_owner_on = Some(UtcDateTime(Utc::now()));
    }

    // the task is signed by owner when he is happy with the selected contractor
    pub fn sign_task_as_contractor(mut self, task_id: TaskId) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_contractor_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_contractor_account_id.clone(),
            task.contractor,
            "Task has different contractor."
        );

        assert!(
            task.signed_by_owner_on.is_some(),
            "Task should be signed by the owner first."
        );

        assert!(
            task.signed_by_contractor_on.is_none(),
            "Task is already signed by contractor."
        );

        task.signed_by_contractor_on = Some(UtcDateTime(Utc::now()));
    }

    // the task is approved by owner when he is happy with the work done
    pub fn approve_task(mut self, task_id: TaskId) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_owner_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_owner_account_id.clone(),
            task.owner,
            "Task has different owner."
        );

        assert!(task.approved_on.is_none(), "Task is already approved.");

        task.approved_on = Some(UtcDateTime(Utc::now()));
        task.completion_percentage = Some(100);
    }

    fn get_usdt_contract() -> AccountId {
        let current_account_id = env::current_account_id();

        if current_account_id.to_string().ends_with(".testnet") {
            return "usdt.fakes.testnet".parse().unwrap();
        }

        if current_account_id.to_string().ends_with(".near") {
            return "usdt.near".parse().unwrap();
        }

        panic!("unknown network");
    }
}

#[cfg(test)]
mod tests;
