use crate::{
    enums::storage_keys::StorageKeys,
    types::{common_types::TaskId, task::Task},
};

use super::{Nescrow, NescrowExt};
use near_sdk::{env, log, near, store::{IterableMap, LookupMap}};

#[near]
#[allow(dead_code)]
impl Nescrow {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract is already initialized");

        log!("Initializing the contract");

        Self {
            deposits: LookupMap::new(StorageKeys::Deposits),
            tasks: LookupMap::new(StorageKeys::Tasks),
            tasks_per_owner: IterableMap::new(StorageKeys::TasksPerOwner),
            tasks_per_engineer: IterableMap::new(StorageKeys::TasksPerEngineer),
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate_state() -> Self {
        let old_state: Self = env::state_read().expect("Old state doesn't exist");

        let mut new_tasks: LookupMap<TaskId, Task> = LookupMap::new(StorageKeys::Tasksv1);

        // for (_, tasks) in old_state.tasks_per_owner.iter() {
        //     for task_id in tasks.iter() {
        //         let old_task = old_state
        //             .legacy_tasks
        //             .get(task_id)
        //             .expect(format!("Task {} wasn't found", task_id).as_str())
        //             .clone();

        //         new_tasks.insert(task_id.clone(), Task { ..old_task });
        //     }
        // }

        return Self {
            deposits: old_state.deposits,
            tasks_per_engineer: old_state.tasks_per_engineer,
            tasks_per_owner: old_state.tasks_per_owner,
            tasks: new_tasks,
            //legacy_tasks: LookupMap::new(StorageKeys::LegacyTasks)
        };
    }
}
