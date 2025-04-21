use std::collections::HashSet;

use crate::{enums::storage_keys::StorageKeys, types::{
    common_types::{TaskId, UsdtBalance},
    task::Task, task_legacy::LegacyTask,
}};

use super::{Nescrow, NescrowExt};
use borsh::{to_vec, BorshDeserialize};
use near_sdk::{
    env, near,
    store::{IterableMap, IterableSet, LookupMap},
    AccountId,
};

#[near]
#[derive(Debug)]
pub(crate) enum StateVersion {
    V1,
    V2,
    V3,
}

const VERSION_KEY: &[u8] = b"VERSION";

#[near()]
pub struct NescrowV2 {
    deposits: LookupMap<String, IterableMap<AccountId, UsdtBalance>>, //user name as a root level key
    investors: IterableSet<String>, //usernames of people who have deposited money
    tasks: LookupMap<TaskId, LegacyTask>,
    tasks_per_owner: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_per_engineer: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_for_dispute_resolution: IterableSet<TaskId>,
}

#[near()]
pub struct NescrowV3 {
    deposits: LookupMap<String, IterableMap<AccountId, UsdtBalance>>, //user name as a root level key
    investors: IterableSet<String>, //usernames of people who have deposited money
    tasks: LookupMap<TaskId, Task>,
    tasks_per_owner: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_per_engineer: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_for_dispute_resolution: IterableSet<TaskId>,
}

#[near]
#[allow(dead_code)]
impl Nescrow {
    #[private]
    pub fn migrate_state() {
        let old_state: NescrowV2 = env::state_read().expect("Old state doesn't exist");

        let current_version = Nescrow::state_version_read();
        near_sdk::log!("Migrating from version: {:?}", current_version);

        match current_version {
            StateVersion::V2 => {
                // migration from V2 to V3
                let mut new_tasks = LookupMap::new(StorageKeys::Tasksv3);

                near_sdk::log!("Before loop");

                old_state.tasks_per_owner.iter().for_each(|(_, owner_tasks)| {
                    
                    owner_tasks.iter().for_each(|task_id| {
                        near_sdk::log!("Reading task");

                        let old_task = old_state.tasks.get(task_id).expect(&format!("Task {:?} not found", task_id));
                    
                        near_sdk::log!("Creating new task");

                        let new_task = Task {
                            task_id: task_id.clone(),
                            contractor_username: old_task.contractor_username.clone(),
                            contractor_account_id: old_task.contractor_account_id.clone(),
                            owner_username: old_task.owner_username.clone(),
                            owner_account_id: old_task.owner_account_id.clone(),
                            reward: old_task.reward,
                            task_hash: old_task.task_hash.clone(),
                            signed_by_contractor_on: old_task.signed_by_contractor_on.clone(),
                            signed_by_owner_on: old_task.signed_by_owner_on.clone(),
                            submitted_by_contractor_on: old_task.submitted_by_contractor_on.clone(),
                            approved_on: old_task.approved_on.clone(),
                            dispute_initiated_on: old_task.dispute_initiated_on.clone(),
                            dispute_initiated_by: old_task.dispute_initiated_by.clone(),
                            dispute_resolved_on: old_task.dispute_resolved_on.clone(),
                            dispute_resolver_account_id: old_task.dispute_resolver_account_id.clone(),
                            dispute_resolver_username: old_task.dispute_resolver_username.clone(),
                            completion_percentage: old_task.completion_percentage.clone(),
                        };

                        near_sdk::log!("Inserting task");

                        new_tasks.insert(task_id.clone(), new_task);
                    });
                    
                });

                env::state_write(&NescrowV3 {
                    deposits: old_state.deposits,
                    investors: old_state.investors,
                    tasks_per_engineer: old_state.tasks_per_engineer,
                    tasks_per_owner: old_state.tasks_per_owner,
                    tasks_for_dispute_resolution: old_state.tasks_for_dispute_resolution,
                    tasks: new_tasks,
                });
                Nescrow::state_version_write(&StateVersion::V3);
            }
            _ => {
                near_sdk::log!("Migration done.");
                return env::value_return(b"\"Migration done.\"");
            }
        }
    }

    fn state_version_read() -> StateVersion {
        env::storage_read(VERSION_KEY)
            .map(|data| {
                StateVersion::try_from_slice(&data).expect("Cannot deserialize the contract state.")
            })
            .unwrap_or(StateVersion::V2) // StateVersion is introduced in V2 State.
    }

    fn state_version_write(version: &StateVersion) {
        let data = to_vec(&version).expect("Cannot serialize the contract state.");
        env::storage_write(VERSION_KEY, &data);
        near_sdk::log!("Migrated to version: {:?}", version);
    }
}
