use enums::storage_keys::StorageKeys;
use near_sdk::json_types::U128;
use near_sdk::{ env, log, near, AccountId, NearToken, Promise };
use near_sdk::store::{ IterableMap, LookupMap };
use types::common_types::UsdtBalance;
use types::ft_transfer_message::FtOnTransferMessage;

mod enums;
mod types;
mod constants;

//no calculations performed, just guessing. This also includes gas for tasks approval.
const USER_REGISTRATION_STORAGE_USAGE: u128 = NearToken::from_millinear(10).as_yoctonear();

#[near(contract_state)]
struct Nescrow {
    deposits: LookupMap<String, IterableMap<AccountId, UsdtBalance>>, //email as a root level key
}

impl Default for Nescrow {
    fn default() -> Self {
        Self {
            deposits: LookupMap::new(StorageKeys::Deposits),
        }
    }
}

#[near]
#[allow(dead_code)]
impl Nescrow {
    #[init]
    pub fn new() -> Self {
        Self {
            deposits: LookupMap::new(StorageKeys::Deposits),
        }
    }

    #[payable]
    pub fn register_customer(&mut self, email: String) {
        if String::is_empty(&email) {
            panic!("Email should be provided");
        }

        if self.deposits.contains_key(&email) {
            return;
        }

        let email_hash = env::sha256_array(&email.as_bytes());

        let account_balance_map = IterableMap::new(StorageKeys::AccountBalance { email_hash });

        self.deposits.insert(email, account_balance_map);

        let attached_deposit = env::attached_deposit();

        assert!(
            USER_REGISTRATION_STORAGE_USAGE <= attached_deposit.as_yoctonear(),
            "Attached deposit too small"
        );

        let refund = attached_deposit.as_yoctonear() - USER_REGISTRATION_STORAGE_USAGE;

        log!("Deposit to return {}", refund);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(NearToken::from_yoctonear(refund));
        }
    }

    pub fn get_my_deposit(&self, sender_email: String) -> UsdtBalance {
        let deposits = self.deposits
            .get(&sender_email)
            .unwrap_or_else(|| panic!("Email not registered"));

        let mut total_balance: u128 = 0;

        deposits.iter().for_each(|(_, &balance)| {
            total_balance += balance.0;
        });

        return U128(total_balance);
    }

    pub fn ft_on_transfer(
        &mut self,
        sender_id: &AccountId,
        amount: UsdtBalance,
        msg: String
    ) -> UsdtBalance {
        let usdt_contract_id = Nescrow::get_usdt_contract();

        if usdt_contract_id != env::predecessor_account_id() {
            panic!("untrusted contract");
        }

        log!("ft_on_transfer called {} {:?} {}", sender_id, amount, msg);

        let parsed_message_result: Result<
            FtOnTransferMessage,
            near_sdk::serde_json::Error
        > = near_sdk::serde_json::from_str(&msg);

        if parsed_message_result.is_err() {
            panic!("Error parsing message");
        }

        let sender_email = parsed_message_result.unwrap().email;

        let sender_deposits = self.deposits
            .get_mut(&sender_email)
            .expect("Customer is not registered. Register the customer first.");

        let ammount_to_add: u128 = amount.into();

        let existing_deposit = sender_deposits.get(sender_id);

        match existing_deposit {
            None => sender_deposits.insert(sender_id.clone(), amount),
            Some(balance) =>
                sender_deposits.insert(sender_id.clone(), U128(balance.0 + ammount_to_add)),
        };

        return U128(0);
    }

    fn get_usdt_contract() -> AccountId {
        let current_account_id = env::current_account_id();

        if current_account_id.to_string().ends_with(".testnet") {
            return "usdt.fakes.testnet".parse().unwrap();
        }

        if current_account_id.to_string().ends_with(".near") {
            return "usdt.near".parse().unwrap();
        }

        panic!("unknown network");
    }
}

#[cfg(test)]
mod tests;
