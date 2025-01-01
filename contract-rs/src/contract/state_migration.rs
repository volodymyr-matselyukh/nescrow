use std::collections::HashSet;

use crate::{
    enums::storage_keys::StorageKeys,
    types::{
        common_types::{TaskId, UsdtBalance},
        task::Task,
    },
};

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
pub struct NescrowV1 {
    deposits: LookupMap<String, IterableMap<AccountId, UsdtBalance>>, //user name as a root level key
    tasks: LookupMap<TaskId, Task>,
    tasks_per_owner: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_per_engineer: IterableMap<AccountId, HashSet<TaskId>>,
    tasks_for_dispute_resolution: IterableSet<TaskId>,
}

#[near()]
pub struct NescrowV2 {
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
        let old_state: NescrowV1 = env::state_read().expect("Old state doesn't exist");

        let current_version = Nescrow::state_version_read();
        near_sdk::log!("Migrating from version: {:?}", current_version);

        match current_version {
            StateVersion::V1 => {
                // migration from V1 to V2
                let investors = IterableSet::new(StorageKeys::Investors);
                env::state_write(&NescrowV2 {
                    deposits: old_state.deposits,
                    investors,
                    tasks_per_engineer: old_state.tasks_per_engineer,
                    tasks_per_owner: old_state.tasks_per_owner,
                    tasks_for_dispute_resolution: old_state.tasks_for_dispute_resolution,
                    tasks: old_state.tasks,
                });
                Nescrow::state_version_write(&StateVersion::V2);
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
            .unwrap_or(StateVersion::V1) // StateVersion is introduced in V2 State.
    }

    fn state_version_write(version: &StateVersion) {
        let data = to_vec(&version).expect("Cannot serialize the contract state.");
        env::storage_write(VERSION_KEY, &data);
        near_sdk::log!("Migrated to version: {:?}", version);
    }
}
