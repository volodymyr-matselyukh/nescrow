use near_sdk::{
    borsh::{self, BorshSerialize},
    BorshStorageKey, CryptoHash,
};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Deposits,
    AccountBalance { username_hash: CryptoHash },
    Tasks,
    TasksPerOwner,
    TasksPerEngineer,
}
