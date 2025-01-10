use std::borrow::Borrow;
use std::ops::{Add, Sub};

use near_sdk::{env, log, near, AccountId, Gas, NearToken, Promise, PromiseError};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::utils::{get_task_reserverd_amount, get_usdt_contract};
use crate::types::common_types::{UsdtBalance, UsdtBalanceExt};
use crate::types::ft_transfer_message::FtOnTransferMessage;

use super::{Nescrow, NescrowExt};

#[near]
#[allow(dead_code)]
impl Nescrow {
    pub fn get_available_deposit_by_username(&self, sender_username: String) -> UsdtBalance {
        let deposits = self
            .deposits
            .get(&sender_username.clone())
            .unwrap_or_else(|| panic!("Username not registered"));

        let mut total_balance = dec!(0);

        deposits.iter().for_each(|(account_id, &balance)| {
            let reserved_ammount =
                self.get_reserved_deposit_by_tasks(account_id.clone(), sender_username.clone());

            total_balance += balance;
            total_balance = total_balance.sub(reserved_ammount);

            log!("username {sender_username}, account{account_id} balance {total_balance}")
        });

        return total_balance;
    }

    // this will include deposit which is blocked by incomplete tasks
    pub fn get_total_deposit_by_username(&self, sender_username: String) -> UsdtBalance {
        let deposits = self
            .deposits
            .get(&sender_username.clone())
            .unwrap_or_else(|| panic!("Username not registered"));

        let mut total_balance = dec!(0);

        deposits.iter().for_each(|(account_id, &balance)| {
            let completed_tasks_rewards =
                self.get_completed_tasks_rewards(account_id.clone(), sender_username.clone());

            total_balance += balance;
            total_balance = total_balance.sub(completed_tasks_rewards);

            log!("username {sender_username}, account{account_id} balance {total_balance}")
        });

        return total_balance;
    }

    pub fn get_deposit_by_username_including_tasks_reservations(
        &self,
        sender_username: String,
    ) -> UsdtBalance {
        let deposits = self
            .deposits
            .get(&sender_username.clone())
            .unwrap_or_else(|| panic!("Username not registered"));

        let mut total_balance = dec!(0);

        deposits.iter().for_each(|(account_id, &balance)| {
            let reserved_ammount =
                self.get_reserved_deposit_by_tasks(account_id.clone(), sender_username.clone());

            total_balance += balance;
            total_balance = total_balance.sub(reserved_ammount);
        });

        return total_balance;
    }

    pub fn get_withdrawable_amount_by_account(
        &self,
        sender_username: String,
        account_id: AccountId,
    ) -> UsdtBalance {
        let deposits = self
            .deposits
            .get(&sender_username)
            .unwrap_or_else(|| panic!("Username not registered"));

        let account_deposit = deposits.get(&account_id);

        let tasks_rewards_sum = self.get_reserved_deposit_by_tasks(account_id, sender_username);

        match account_deposit {
            None => return UsdtBalance::from(0),
            Some(deposit) => return deposit.clone().sub(tasks_rewards_sum),
        };
    }

    // deposit which has been put into the task reward
    fn get_reserved_deposit_by_tasks(&self, account_id: AccountId, username: String) -> Decimal {
        let mut tasks_rewards_sum = dec!(0);

        let sender_tasks = self.tasks_per_owner.get(&account_id);

        if sender_tasks.is_some() {
            let unwrapped_tasks = sender_tasks.unwrap();

            if unwrapped_tasks.len() > 0 {
                unwrapped_tasks.iter().for_each(|task_id| {
                    let task = self.tasks.get(task_id);

                    if task.is_none() {
                        return;
                    }

                    let task_unwrapped = task.unwrap();

                    if task_unwrapped.owner_username != username {
                        return;
                    }

                    // if task_unwrapped.dispute_resolved_on.is_some()
                    //     || task_unwrapped.approved_on.is_some()
                    // {
                    //     return;
                    // }

                    log!(
                        "task id {}, task reward {}",
                        task_unwrapped.task_id,
                        task_unwrapped.reward
                    );

                    let task_reward_with_fees = get_task_reserverd_amount(task_unwrapped);

                    tasks_rewards_sum = tasks_rewards_sum.add(task_reward_with_fees);
                });
            }
        }

        return tasks_rewards_sum;
    }

    // get deposit spent only on completed tasks
    fn get_completed_tasks_rewards(&self, account_id: AccountId, username: String) -> Decimal {
        let mut tasks_rewards_sum = dec!(0);

        let sender_tasks = self.tasks_per_owner.get(&account_id);

        if sender_tasks.is_some() {
            let unwrapped_tasks = sender_tasks.unwrap();

            if unwrapped_tasks.len() > 0 {
                unwrapped_tasks.iter().for_each(|task_id| {
                    let task = self.tasks.get(task_id);

                    if task.is_none() {
                        return;
                    }

                    let task_unwrapped = task.unwrap();

                    if task_unwrapped.owner_username != username {
                        return;
                    }

                    if task_unwrapped.dispute_resolved_on.is_none()
                        && task_unwrapped.approved_on.is_none()
                    {
                        return;
                    }

                    log!(
                        "task id {}, task reward {}",
                        task_unwrapped.task_id,
                        task_unwrapped.reward
                    );

                    let task_reward_with_fees = get_task_reserverd_amount(task_unwrapped);

                    tasks_rewards_sum = tasks_rewards_sum.add(task_reward_with_fees);
                });
            }
        }

        return tasks_rewards_sum;
    }

    pub fn get_total_deposit(&self) -> UsdtBalance {
        let mut total_balance = dec!(0);

        self.investors.iter().for_each(|investor| {
            let investor_deposit = self.get_total_deposit_by_username(investor.clone());
            total_balance += investor_deposit;
        });

        return total_balance;
    }

    // called by USDT contract
    pub fn ft_on_transfer(
        &mut self,
        sender_id: &AccountId,
        amount: UsdtBalance,
        msg: String,
    ) -> UsdtBalance {
        let usdt_contract_id = get_usdt_contract();

        if usdt_contract_id != env::predecessor_account_id() {
            panic!("untrusted contract");
        }

        log!("ft_on_transfer called {} {:?} {}", sender_id, amount, msg);

        let nescrow_ammount = UsdtBalance::from_usdt_to_human(amount);

        let parsed_message_result: Result<FtOnTransferMessage, near_sdk::serde_json::Error> =
            near_sdk::serde_json::from_str(&msg);

        if parsed_message_result.is_err() {
            panic!("Error parsing message");
        }

        let sender_username = parsed_message_result.unwrap().username;

        let sender_deposits = self
            .deposits
            .get_mut(&sender_username)
            .expect("Customer is not registered. Register the customer first.");

        let existing_deposit = sender_deposits.get(sender_id);

        match existing_deposit {
            None => sender_deposits.insert(sender_id.clone(), nescrow_ammount),
            Some(balance) => sender_deposits.insert(sender_id.clone(), balance + nescrow_ammount),
        };

        self.investors.insert(sender_username.clone());

        return dec!(0);
    }

    pub fn withdraw(&self, receiver_username: String, amount: UsdtBalance) -> Promise {
        let is_username_registered = self.deposits.contains_key(&receiver_username);

        assert!(is_username_registered, "Username is not registered");

        let receiver_account_id = env::predecessor_account_id();

        let withdrawable_amount = self.get_withdrawable_amount_by_account(
            receiver_username.clone(),
            receiver_account_id.clone(),
        );

        assert!(withdrawable_amount > dec!(0), "Nothing to withdraw");

        assert!(amount > dec!(0), "Amount should be positive value");

        assert!(
            amount <= withdrawable_amount,
            "Max withdraw is {:#?}",
            withdrawable_amount
        );

        let usdt_contract_id = get_usdt_contract();

        let ft_transfer_promise = Promise::new(usdt_contract_id).function_call(
            "ft_transfer".to_string(),
            near_sdk::serde_json::json!({
                "amount": UsdtBalance::from_human_to_usdt(amount).to_string(),
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
                .ft_transfer_callback(receiver_username.clone(), receiver_account_id, amount),
        );
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_transfer_callback(
        &mut self,
        #[callback_result] call_result: Result<(), PromiseError>,
        receiver_username: String,
        receiver_account_id: AccountId,
        amount: UsdtBalance, // amount here will be in human readable form
    ) {
        if call_result.is_err() {
            panic!("There was an error during ft_transfer");
        }

        let sender_deposits = self
            .deposits
            .get_mut(&receiver_username)
            .expect("Customer is not registered. Register the customer first.");

        let ammount_to_deduct = *amount.borrow();

        let existing_deposit = sender_deposits
            .get(&receiver_account_id)
            .unwrap_or_else(|| panic!("Deposit doesn't exist"));

        assert!(
            *existing_deposit >= ammount_to_deduct,
            "Amount to deduct is bigger then deposit"
        );

        sender_deposits.insert(
            receiver_account_id.clone(),
            existing_deposit - ammount_to_deduct,
        );
    }
}
