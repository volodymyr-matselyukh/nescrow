use super::utils::get_trusted_admin_accounts;
use super::{Nescrow, NescrowExt};
use crate::contract::USER_REGISTRATION_STORAGE_USAGE_DEPOSIT;
use crate::enums::storage_keys::StorageKeys;
use near_sdk::store::IterableMap;
use near_sdk::{env, log, near, AccountId, NearToken, Promise};
use rust_decimal_macros::dec;

#[near]
#[allow(dead_code)]
impl Nescrow {
    #[payable]
    pub fn register_customer(&mut self, username: String, account_id: AccountId) {
        if String::is_empty(&username) {
            panic!("Username should be provided");
        }

        if self.deposits.contains_key(&username)
            && self
                .deposits
                .get(&username)
                .unwrap()
                .contains_key(&account_id)
        {
            panic!("Combination of username and account id is already registered");
        }

        if self.deposits.contains_key(&username) {
            let existing_username_account_balance_map = self
                .deposits
                .get_mut(&username)
                .expect("Account balance map wasn't found");

            existing_username_account_balance_map.insert(account_id, dec!(0));
        } else {
            let username_hash = env::sha256_array(&username.as_bytes());

            let mut account_balance_map =
                IterableMap::new(StorageKeys::AccountBalance { username_hash });
            account_balance_map.insert(account_id, dec!(0));

            self.deposits.insert(username, account_balance_map);
        }

        let attached_deposit = env::attached_deposit();

        assert!(
            USER_REGISTRATION_STORAGE_USAGE_DEPOSIT <= attached_deposit.as_yoctonear(),
            "Attached deposit should be >= {} for registering on blockchain",
            NearToken::from_yoctonear(USER_REGISTRATION_STORAGE_USAGE_DEPOSIT)
        );

        let refund = attached_deposit.as_yoctonear() - USER_REGISTRATION_STORAGE_USAGE_DEPOSIT;

        log!("Deposit to return {}", refund);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(NearToken::from_yoctonear(refund));
        }
    }

    pub fn is_registered(&self, sender_username: String) -> bool {
        let is_username_registered = self.deposits.contains_key(&sender_username);

        return is_username_registered;
    }

    pub fn is_username_account_registered(
        &self,
        sender_username: String,
        account_id: AccountId,
    ) -> bool {
        let is_username_registered = self.deposits.contains_key(&sender_username);

        if !is_username_registered {
            return false;
        }

        let username_deposits = self
            .deposits
            .get(&sender_username)
            .expect("Username should be registered");

        let is_username_account_registered = username_deposits.contains_key(&account_id);

        return is_username_account_registered;
    }

    pub fn get_is_admin(account_id: AccountId) -> bool {
        let trusted_admins = get_trusted_admin_accounts();

        return trusted_admins.contains(&account_id);
    }
}
