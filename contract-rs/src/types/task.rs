use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use super::common_types::UsdtBalance;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Clone)]
pub struct Task {
    pub task_id: String,
    pub owner_username: String,
    pub owner_account_id: AccountId,
    pub contractor_username: String,
    pub contractor_account_id: AccountId,
    pub reward: UsdtBalance,
    pub task_hash: Option<String>, // has reward, title, description
    pub signed_by_owner_on: Option<u64>,
    pub signed_by_contractor_on: Option<u64>,
    pub submitted_by_contractor_on: Option<u64>,
    pub approved_on: Option<u64>,
    pub dispute_initiated_on: Option<u64>,
    pub dispute_initiated_by: Option<AccountId>,
    pub dispute_resolved_on: Option<u64>,
    pub dispute_resolver_account_id: Option<AccountId>,
    pub dispute_resolver_username: Option<String>, // nescrow admin username
    pub completion_percentage: Option<u8>,
    pub claimed_by_contractor_on: Option<u64>,
    pub claimed_by_owner_on: Option<u64>,
}
