use std::str::FromStr;

use near_sdk::json_types::U128;
use rust_decimal::Decimal;

use crate::types::common_types::{UsdtBalance, UsdtBalanceExt};

#[test]
fn test_decimal_to_usdt_balance() {
    let decimal_amount = Decimal::from_str("34.56").expect("String to decimal issue");

    assert_eq!(decimal_amount, Decimal::new(3456, 2), "Decimals should match");

    let usdt = UsdtBalance::from_decimal(decimal_amount);

    assert_eq!(usdt, U128(34_560_000), "Decimals should match");
}