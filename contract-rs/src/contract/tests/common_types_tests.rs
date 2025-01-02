use std::str::FromStr;

use near_sdk::json_types::U128;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::types::common_types::{UsdtBalance, UsdtBalanceExt};

#[test]
fn test_decimal_to_usdt_balance() {
    let decimal_amount = Decimal::from_str("34.56").expect("String to decimal issue");

    assert_eq!(decimal_amount, Decimal::new(3456, 2), "Decimals should match");

    let usdt = UsdtBalance::to_usdt(decimal_amount);

    assert_eq!(usdt, U128(34_560_000), "Decimals should match");
}

#[test]
pub fn test_usdt_fakes_to_human() {
    let fakes_usdt:  UsdtBalance = Decimal::from_str("55000000").unwrap();
    let human_usdt = UsdtBalance::from_usdt_to_human(fakes_usdt);

    assert_eq!(human_usdt, dec!(55), "USDT conversion to human is wrong");
}