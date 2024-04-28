use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault};
use near_sdk::{collections::Vector, collections::LookupMap};
use near_sdk::json_types::U64;

const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
  pub premium: bool, 
  pub sender: AccountId,
  pub text: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct GuestBook {
  messages: Vector<PostedMessage>,
  deposits: LookupMap<AccountId, Balance>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct OldState {
  messages: Vector<PostedMessage>,
}

#[near_bindgen]
impl GuestBook {

  #[init]
  pub fn new() -> Self {
    Self{
      messages: Vector::new("messages".as_bytes()),
      deposits: LookupMap::new("deposits".as_bytes())
    }
  }

  #[payable]
  pub fn add_message(&mut self, text: String) {
    // If the user attaches more than 0.01N the message is premium
    let attached_deposit = env::attached_deposit();
    let premium = attached_deposit >= POINT_ONE;
    let sender = env::predecessor_account_id();

    let is_sender_present = self.deposits.contains_key(&sender);

    if is_sender_present {
      let mut current_deposit = self.deposits.get(&sender).unwrap_or_default();
      current_deposit = current_deposit + attached_deposit;

      self.deposits.insert(&sender, &current_deposit);
    } else {
      self.deposits.insert(&sender, &attached_deposit);
    }

    let message = PostedMessage{premium, sender, text};
    self.messages.push(&message);
  }

  pub fn get_messages(&self, from_index:Option<U64>, limit:Option<U64>) -> Vec<PostedMessage>{
    let from = u64::from(from_index.unwrap_or(U64(0)));
    let limit = u64::from(limit.unwrap_or(U64(10)));

    self.messages.iter()
    .skip(from as usize)
    .take(limit as usize)
    .collect()
  }

  pub fn get_my_deposit(&self, sender: AccountId) -> u128 {
    let is_sender_present = self.deposits.contains_key(&sender);

    if is_sender_present {
      return self.deposits.get(&sender).unwrap_or_default()
    }

    return 0
  }

  pub fn total_messages(&self) -> u64 { self.messages.len() }

  pub fn ft_on_transfer(&self, sender_id: AccountId, amount: String, msg: String) -> String {
    log!("ft_on_transfer called {} {} {}", sender_id, amount, msg);

    near_sdk::env::panic_str("not found");

    // return "0".to_string()
  }

  #[private]
  #[init(ignore_state)]
  pub fn migrate() -> Self {
      // retrieve the current state from the contract
      let mut new_messages: Vector<PostedMessage> = Vector::new("messages".as_bytes());

      let removal_result = near_sdk::env::storage_remove("new_messages\x00\x00\x00\x00\x00\x00\x00\x00".as_bytes());
      log!("Removal result {}", removal_result);

      let removal_result1 = near_sdk::env::storage_remove("new_messages\x01\x00\x00\x00\x00\x00\x00\x00".as_bytes());
      log!("Removal result {}", removal_result1);
      
      let premium = false;
      let sender = env::predecessor_account_id();
      let text = "initial text";
      let initial_message = PostedMessage{premium, sender, text: String::from(text)};
      new_messages.push(&initial_message);

      // return the new state
      Self {
          messages: new_messages,
          deposits: LookupMap::new("deposits".as_bytes())
      }
  }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_message() {
    let mut contract = GuestBook::default();
    contract.add_message("A message".to_string());

    let posted_message = &contract.get_messages(None, None)[0];
    assert_eq!(posted_message.premium, false);
    assert_eq!(posted_message.text, "A message".to_string());
  }

  #[test]
  fn iters_messages() {
    let mut contract = GuestBook::default();
    contract.add_message("1st message".to_string());
    contract.add_message("2nd message".to_string());
    contract.add_message("3rd message".to_string());
    
    let total = &contract.total_messages();
    assert!(*total == 3); 

    let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
    assert_eq!(last_message.premium, false);
    assert_eq!(last_message.text, "3rd message".to_string());
  }
}