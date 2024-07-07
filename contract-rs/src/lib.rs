use enums::storage_keys::StorageKeys;
use near_sdk::json_types::U128;
use near_sdk::{ env, log, near, AccountId, NearToken, Promise };
use near_sdk::store::LookupMap;
use types::common_types::UsdtBalance;
use types::ft_transfer_message::FtOnTransferMessage;

mod enums;
mod types;
mod constants;

//no calculations performed, just guessing. This also includes gas for works approval.
const USER_REGISTRATION_STORAGE_USAGE: u128 = NearToken::from_millinear(10).as_yoctonear();

#[near(contract_state)]
struct Nescrow {
    deposits: LookupMap<String, UsdtBalance>, //email, usdt balance
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

        self.deposits.insert(email, U128(0));

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
        return self.deposits.get(&sender_email).unwrap_or(&U128(0)).clone();
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

        let mut sender_deposit: u128 = self.deposits
            .get(&sender_email)
            .expect("Customer is not registered. Register the customer first.")
            .to_owned()
            .into();

        let ammount_to_add: u128 = amount.into();
        sender_deposit += ammount_to_add;

        self.deposits.insert(sender_email, U128(sender_deposit));

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
