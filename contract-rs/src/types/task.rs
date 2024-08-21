use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;

use super::{borsh::date_utc::UtcDateTime, common_types::UsdtBalance};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Task {
    pub owner: AccountId,
    pub contractor: AccountId,
    pub reward: UsdtBalance,
    pub signed_by_owner_on: Option<UtcDateTime>,
    pub signed_by_contractor_on: Option<UtcDateTime>,
    pub submitted_by_contractor_on: Option<UtcDateTime>,
    pub approved_on: Option<UtcDateTime>,
    pub dispute_initiated_on: Option<UtcDateTime>,
    pub resolution: Option<u8>, // percents
}
