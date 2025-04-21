use near_sdk::{
    borsh::{self, BorshSerialize},
    BorshStorageKey, CryptoHash,
};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Deposits,
    AccountBalance { username_hash: CryptoHash },
    TasksPerOwner,
    TasksPerEngineer,
    TasksForDisputeResolution,
    Investors,
    Tasksv3
}
