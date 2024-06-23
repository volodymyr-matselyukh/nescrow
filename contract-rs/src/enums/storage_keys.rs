use near_sdk::{borsh::{ self, BorshSerialize }, BorshStorageKey};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
  Deposits
}