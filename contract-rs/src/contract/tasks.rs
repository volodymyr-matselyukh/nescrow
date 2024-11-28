use std::collections::HashSet;
use std::ops::Add;

use near_sdk::env::block_timestamp_ms;
use near_sdk::{env, log, near, AccountId, Gas, NearToken, Promise};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::contract::utils::{
    get_dispute_resolution_amount, get_nescrow_beneficiary_contract, get_usdt_contract,
};
use crate::types::common_types::{TaskId, UsdtBalance};
use crate::types::pagination::Pagination;
use crate::types::task::Task;

use super::{
    Nescrow, NescrowExt, NESCROW_BENEFICIARY_USERNAME,
    NESCROW_DISPUTE_RESOLUTION_FEE, NESCROW_FREELANCER_FEE, NESCROW_OWNER_FEE,
    USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT,
};

#[near]
#[allow(dead_code)]
impl Nescrow {
    // the task is created when the owner accepts the contractor
    #[payable]
    pub fn create_task(
        &mut self,
        task_id: TaskId,
        owner_username: String,
        contractor: AccountId,
        contractor_username: String,
        reward: UsdtBalance,
    ) {
        assert!(
            !self.tasks.contains_key(&task_id),
            "Taks has already been created"
        );

        let task_owner_account_id = env::predecessor_account_id();

        let withdrawable_amount =
            self.get_withdrawable_amount_by_account(owner_username.clone(), task_owner_account_id.clone());

        let task = Task {
            task_id: task_id.clone(),
            contractor_username,
            contractor_account_id: contractor.clone(),
            owner_username: owner_username.clone(),
            owner_account_id: task_owner_account_id.clone(),
            reward: reward.clone(),
            task_hash: None,
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

        let existing_tasks_per_owner_unwrapped =
            self.tasks_per_owner.get_mut(&task_owner_account_id);
        if existing_tasks_per_owner_unwrapped.is_some() {
            existing_tasks_per_owner_unwrapped
                .unwrap()
                .insert(task_id.clone());
        } else {
            let mut new_tasks_per_owner = HashSet::new();
            new_tasks_per_owner.insert(task_id.clone());

            self.tasks_per_owner
                .insert(task_owner_account_id.clone(), new_tasks_per_owner);
        }

        let existing_tasks_per_engineer_unwrapped = self.tasks_per_engineer.get_mut(&contractor);
        if existing_tasks_per_engineer_unwrapped.is_some() {
            existing_tasks_per_engineer_unwrapped
                .unwrap()
                .insert(task_id.clone());
        } else {
            let mut new_tasks_per_engineer = HashSet::new();
            new_tasks_per_engineer.insert(task_id.clone());

            self.tasks_per_engineer
                .insert(contractor.clone(), new_tasks_per_engineer);
        }

        let attached_deposit = env::attached_deposit();

        assert!(
            USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT <= attached_deposit.as_yoctonear(),
            "Attached deposit should be >= {} for task creation on blockchain",
            NearToken::from_yoctonear(USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT)
        );

        let task_reward_including_fees = Decimal::from(reward.0)
            + Decimal::from(reward.0) * NESCROW_OWNER_FEE
            + Decimal::from(reward.0) * NESCROW_DISPUTE_RESOLUTION_FEE;

        assert!(
            Decimal::from(withdrawable_amount.0) >= task_reward_including_fees,
            "You have not enought deposit to cover the reward for this task."
        );

        let refund = attached_deposit.as_yoctonear() - USER_TASK_CREATION_STORAGE_USAGE_DEPOSIT;

        log!("Deposit to return {}", refund);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(NearToken::from_yoctonear(refund));
        }
    }

    pub fn update_task_reward(&mut self, task_id: TaskId, reward: UsdtBalance) {
        let task_owner_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        if task_owner_account_id.clone() != task.owner_account_id {
            panic!("Operation forbidden. You must be an owner of the task");
        }

        if task.signed_by_owner_on.is_some() && task.signed_by_contractor_on.is_some() {
            panic!("Operation forbidden. Task has been signed by both owner and contractor. Modifying is impossible");
        }

        assert!(
            task.signed_by_owner_on.is_none(),
            "Task is already signed by owner. Unsign the task first"
        );

        task.reward = reward;
    }

    pub fn get_owner_tasks(
        &self,
        task_owner: AccountId,
        pagination: Option<Pagination>,
    ) -> Vec<&Task> {
        let pagination = pagination.unwrap_or_default();

        let tasks_per_owner = self.tasks_per_owner.get(&task_owner);

        if !tasks_per_owner.is_some() {
            return Vec::new();
        }

        return tasks_per_owner
            .unwrap()
            .iter()
            .take(pagination.take())
            .skip(pagination.skip())
            .filter_map(|task_id| {
                let task = self.tasks.get(task_id);

                if task.is_none() {
                    return None;
                }

                let task_unwrapped = task.unwrap();

                return Some(task_unwrapped);
            })
            .collect();
    }

    pub fn get_engineer_tasks(
        &self,
        task_contractor: AccountId,
        pagination: Option<Pagination>,
    ) -> Vec<&Task> {
        let pagination = pagination.unwrap_or_default();

        let tasks_per_contractor = self.tasks_per_engineer.get(&task_contractor);

        if !tasks_per_contractor.is_some() {
            return Vec::new();
        }

        return tasks_per_contractor
            .unwrap()
            .iter()
            .take(pagination.take())
            .skip(pagination.skip())
            .filter_map(|task_id| {
                let task = self.tasks.get(task_id);

                if task.is_none() {
                    return None;
                }

                let task_unwrapped = task.unwrap();

                return Some(task_unwrapped);
            })
            .collect();
    }

    pub fn get_task(&self, task_id: TaskId) -> &Task {
        let task = self.tasks.get(&task_id);

        assert!(task.is_some(), "Task not found");

        return task.unwrap();
    }

    // the task is removed when the owner decides to reject the contractor
    pub fn remove_task(&mut self, task_id: TaskId) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_owner = env::predecessor_account_id();
        let task = self.tasks.remove(&task_id).expect("Task not found");

        assert_eq!(
            task.owner_account_id, task_owner,
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
        self.tasks_per_engineer.remove(&task.contractor_account_id);
    }

    // the task is signed by owner when he is happy with the selected contractor and wants to proceed to work started
    pub fn sign_task_as_owner(&mut self, task_id: TaskId, task_hash: String) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_owner_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        if task_owner_account_id.clone() != task.owner_account_id {
            panic!("Operation forbidden. You must be an owner of the task.");
        }

        assert!(
            task.signed_by_owner_on.is_none(),
            "Task is already signed by owner."
        );

        let is_hash_changed: bool = task.task_hash.is_some()
            && task.task_hash.as_ref().unwrap().as_str() != task_hash.as_str();

        if task.signed_by_contractor_on.is_some() && is_hash_changed {
            task.signed_by_contractor_on = None;
        }

        task.signed_by_owner_on = Some(block_timestamp_ms());
        task.task_hash = Some(task_hash);
    }

    // the task is signed by owner when he is happy with the selected contractor and wants to proceed to work started
    pub fn unsign_task_as_owner(&mut self, task_id: TaskId) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_owner_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        if task_owner_account_id.clone() != task.owner_account_id {
            panic!("Operation forbidden. You must be an owner of the task");
        }

        assert!(task.signed_by_owner_on.is_some(), "Task is not signed yet");

        assert!(
            task.signed_by_contractor_on.is_none(),
            "Task is signed by contractor. Unsigning is impossible"
        );

        task.signed_by_owner_on = None;
        task.task_hash = None;
    }

    // the task is signed by owner when he is happy with the selected contractor
    pub fn sign_task_as_contractor(&mut self, task_id: TaskId, task_hash: String) {
        assert!(self.tasks.contains_key(&task_id), "Taks does not exist");

        let task_contractor_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_contractor_account_id.clone(),
            task.contractor_account_id,
            "Task has different contractor"
        );

        assert!(
            task.signed_by_contractor_on.is_none(),
            "Task is already signed by contractor"
        );

        assert!(
            task.signed_by_owner_on.is_some(),
            "Task should be signed by the owner first"
        );

        assert_eq!(
            task_hash.as_str(),
            task.task_hash.as_ref().unwrap(),
            "Task hash is different from that signed by owner"
        );

        task.signed_by_contractor_on = Some(block_timestamp_ms());
    }

    // contractor performs a work submit when the work is done
    pub fn submit_work(&mut self, task_id: TaskId) {
        let task_contractor_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_contractor_id.clone(),
            task.contractor_account_id,
            "Task has different contractor."
        );

        assert!(
            task.submitted_by_contractor_on.is_none(),
            "Task is already submitted."
        );

        task.submitted_by_contractor_on = Some(block_timestamp_ms());
    }

    // the task is approved by owner when he is happy with the work done
    pub fn approve_task(&mut self, task_id: TaskId) {
        let task_owner_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_owner_account_id.clone(),
            task.owner_account_id,
            "Task has different owner."
        );

        assert!(task.approved_on.is_none(), "Task is already approved.");

        assert!(
            task.submitted_by_contractor_on.is_some(),
            "Work should be submitted first."
        );

        // handle candidate deposit
        let candidate_deposit = self
            .deposits
            .get_mut(&task.contractor_username)
            .expect("Candidate is not registered");

        let candidate_account_deposit = candidate_deposit
            .get_mut(&task.contractor_account_id.clone())
            .expect("Candidate account not found");

        let nescrow_felancer_fee = Decimal::from(task.reward.0) * NESCROW_FREELANCER_FEE;
        let candidate_reward_without_nescrow_fee = Decimal::from(task.reward.0)
            .add(-nescrow_felancer_fee)
            .to_u128();

        let candidate_new_deposit = candidate_account_deposit
            .0
            .add(candidate_reward_without_nescrow_fee.unwrap());

        candidate_account_deposit.0 = candidate_new_deposit;

        // handle owner deposit
        let nescrow_owner_fee = Decimal::from(task.reward.0) * NESCROW_OWNER_FEE;
        let dispute_resolution_amount = get_dispute_resolution_amount(task.reward);

        let owner_deposit = self
            .deposits
            .get_mut(&task.owner_username.clone())
            .expect("Owner is not registered");

        let owner_account_deposit = owner_deposit
            .get_mut(&task.owner_account_id.clone())
            .expect("Owner account not found");

        owner_account_deposit.0 = owner_account_deposit.0.add(dispute_resolution_amount.0);

        // handle nescrow deposit
        let nescrow_earnings = nescrow_owner_fee.add(nescrow_felancer_fee);

        let nescrow_deposit = self
            .deposits
            .get_mut(NESCROW_BENEFICIARY_USERNAME)
            .expect("Nescrow is not registered");

        let nescrow_account_deposit = nescrow_deposit
            .get_mut(&get_nescrow_beneficiary_contract())
            .expect("Owner account not found");

        nescrow_account_deposit.0 = nescrow_account_deposit
            .0
            .add(nescrow_earnings.to_u128().unwrap());

        task.approved_on = Some(block_timestamp_ms());
        task.completion_percentage = Some(100);
    }

    pub fn claim_by_contractor(&mut self, task_id: TaskId) -> Promise {
        let task_contractor_account_id = env::predecessor_account_id();

        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        assert_eq!(
            task_contractor_account_id.clone(),
            task.contractor_account_id,
            "Task has different contractor."
        );

        assert!(task.approved_on.is_some(), "Task is not approved.");
        assert!(
            task.completion_percentage.is_some(),
            "Task completion percantage is undefined."
        );

        let nescrow_felancer_fee = Decimal::from(task.reward.0) * NESCROW_FREELANCER_FEE;
        let nescrow_owner_fee = Decimal::from(task.reward.0) * NESCROW_OWNER_FEE;

        let amount_to_claim =
            Decimal::from((task.completion_percentage.unwrap() / 100) as u128 * task.reward.0)
                - nescrow_felancer_fee;

        let usdt_contract_id = get_usdt_contract();

        let contractor_transfer_promise = Promise::new(usdt_contract_id.clone()).function_call(
            "ft_transfer".to_string(),
            near_sdk::serde_json::json!({
                "amount": amount_to_claim.round().to_string(),
                "receiver_id": task_contractor_account_id.clone(),
            })
            .to_string()
            .into_bytes(),
            NearToken::from_yoctonear(1),
            Gas::from_tgas(3),
        );

        task.claimed_by_contractor_on = Some(block_timestamp_ms());

        return contractor_transfer_promise.then(
            Promise::new(usdt_contract_id.clone()).function_call(
                "ft_transfer".to_string(),
                near_sdk::serde_json::json!({
                    "amount": (nescrow_felancer_fee + nescrow_owner_fee).round().to_string(),
                    "receiver_id": get_nescrow_beneficiary_contract(),
                })
                .to_string()
                .into_bytes(),
                NearToken::from_yoctonear(1),
                Gas::from_tgas(3),
            ),
        );
    }

    pub fn reset_claim(&mut self, task_id: TaskId) {
        let task = self.tasks.get_mut(&task_id).expect("Task not found");

        task.claimed_by_contractor_on = None;
    }
}
