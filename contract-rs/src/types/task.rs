use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;

use super::common_types::UsdtBalance;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Task {
    pub owner: AccountId,
    pub contractor: AccountId,
    pub reward: UsdtBalance,
    pub signed_by_owner_on: Option<u64>,
    pub signed_by_contractor_on: Option<u64>,
    pub submitted_by_contractor_on: Option<u64>,
    pub approved_on: Option<u64>,
    pub dispute_initiated_on: Option<u64>,
    pub dispute_resolved_on: Option<u64>,
    pub completion_percentage: Option<u8>,
    pub claimed_by_contractor_on: Option<u64>,
    pub claimed_by_owner_on: Option<u64>,
}
